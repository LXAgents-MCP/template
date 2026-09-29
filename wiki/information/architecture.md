# Architecture

Five source files and a folder of tools. There is no build step and no code generation.

```
src/
  index.js     entry point: picks a transport, and owns the cluster
  app.js       the express application, as a pure factory — builds, never listens
  server.js    builds the McpServer, registers every tool, exports listTools()
  cli.js       the CLI: help, version, tools, serve
  version.js   reads the version out of package.json at import
  tools/       one file per tool
```

`app.js` and `index.js` are split on purpose. `app.js` returns an express app and
nothing else — it does not listen and does not read `MCP_CLUSTER_WORKERS`. A file that
both builds the app and binds a port cannot be reasoned about, or tested, without
binding one.

## Entry point and transports

`src/index.js` reads `MCP_TRANSPORT` and serves either way:

* **stdio** (default) — one `McpServer` connected to a `StdioServerTransport` for the
  life of the process. Never forked: stdout is the JSON-RPC channel, and a worker's copy
  of it would corrupt the stream.
* **streamable HTTP** — an express application exposing `GET /healthz` and `POST /mcp`,
  and nothing else, served by `node:cluster` workers on one `PORT`.

The HTTP transport is **stateless**: a fresh `McpServer` and transport are built for
each request and closed when the response closes. That is deliberate — `McpServer`
holds per-connection state, so hoisting one to module scope would leak state between
unrelated callers.

The bind is named rather than implied: `HOST`, default `0.0.0.0`, printed on the startup
line. The `node:http` server this replaced called `listen(port)` with no host at all, so
the port was open on every interface because a line was shorter, not because anyone
decided.

### The `Host` guard

`MCP_ALLOWED_HOSTS` guards what reaches the port. When it is set, every request —
`/healthz` included — is matched against it first, and a `Host` outside the list is
refused with a 403 carrying a JSON-RPC error. The match is port-agnostic, because a
client reaching the server through a proxy sends `host:port`. The middleware is the
SDK's `hostHeaderValidation`, mounted natively by `src/app.js`.

**Unset means no allow-list is applied**, and the startup line says so. The middleware
is not mounted at all in that state rather than mounted with an empty list, because
"no allow-list configured" and "refuse everything" are different answers. A control
that is off silently reads as present, so the absence is announced instead.

**This guard is new in this repository.** The four sibling LXAgents MCP servers have
had it for some time; the template did not, so a project scaffolded from it would have
started unguarded. The code is deliberately identical to the siblings' — see
[`../../.agents/memory/decisions/express-for-http-transport.md`](../../.agents/memory/decisions/express-for-http-transport.md).

### The body limit

`express.json({ limit })` is 4 MB. An oversized body and a malformed one are both
answered **400 / `-32700`**, and that collapse is deliberate: the hand-rolled body
reader this replaced threw one failure for both, so a client was never taught to expect
anything different for the second case. `X-Powered-By` is disabled — it would hand an
unauthenticated caller the framework and its version for free.

## Workers

On HTTP, the primary forks `MCP_CLUSTER_WORKERS` processes (default:
`os.availableParallelism()`) and every worker binds the same `PORT`. The kernel's shared
listening handle and the round-robin scheduler do the distribution, so there is no
sticky-session logic to write and no `SO_REUSEPORT` set by hand — the scheduler already
has the information such a scheme would have to reconstruct.

`MCP_CLUSTER_WORKERS=1` means **no fork at all**: one process, one listener, the
pre-cluster behaviour. That is what makes the cluster bisectable — the same code answers
with and without workers, so a difference between them is a difference in the fork rather
than in the transport.

The primary binds nothing, so the startup lines in a container's log describe ports that
are genuinely open, from the processes that opened them. A worker whose primary is gone
exits on `disconnect`: it would otherwise hold the port for whoever starts next, and
fail the *next* run with `EADDRINUSE` for a reason that has nothing to do with the code
under test.

### Shutdown drains before it closes

`SIGINT` and `SIGTERM` run the same three steps, in this order: stop accepting
connections, close idle keep-alive sockets, and then give what is genuinely still in
flight a short grace period before cutting it off. The idle sockets are closed
separately because `server.close()` waits on them, and a client that opened one and went
quiet would otherwise hold the process open for a request that no longer exists.

Because the transport is stateless there is no session to drain — what drains is the
requests. The primary relays the signal to its workers and waits for the last one to go,
so the port is closed before the process that started it is; a worker logs its own
`draining` line because the worker is the process actually draining. A second signal
during the drain is the operator saying they have stopped waiting, and it exits at once
rather than queueing behind the first.

### stdout belongs to the protocol

On stdio, stdout **is** the JSON-RPC channel. Server-side logging goes to stderr and
`serve` prints nothing of its own; only CLI commands write to stdout. A `console.log`
on the server path corrupts the stream, and the client reports a parse error that
points nowhere useful.

## The tool layer

Each tool is one file at `src/tools/{tool_name}.js`, exporting a `config` and a
`handler`:

```js
export const config = {
  name: "calculate_sum",
  description: "Add two numbers and return the sum. Requires no API key.",
  schema: {                              // optional
    a: z.number().describe("The first addend."),
    b: z.number().describe("The second addend."),
  },
};

export async function handler({ a, b }) {
  return { content: [{ type: "text", text: String(a + b) }] };
}

export default { config, handler };
```

`src/server.js` imports each module individually, collects them into one
`TOOL_MODULES` array, and registers each:

```js
server.tool(config.name, config.description, config.schema, handler);
```

A tool that declares no `schema` is registered with the three-argument form instead.

`schema` is a **zod raw shape** — a plain object of validators, not a `z.object(...)`.
The MCP SDK wraps it itself and converts it to the JSON Schema the client sees;
wrapping it first produces a tool that advertises no parameters and receives none.

## Authentication

There is one key for the whole server, `process.env.API_KEY`, and it is read **inside
the handler** of each tool that needs it:

```js
const apiKey = process.env.API_KEY;
if (!apiKey) throw new Error("search_secure_data requires an API key. Set …");
```

Reading it at call time rather than at import means a process that sets the key after
startup still works, and it keeps the stateless HTTP path correct — the module cache
outlives any single request.

Registration never depends on the key. Every tool is advertised whether or not one is
set, because a server that hides its authenticated tools reports "no such tool", which
is indistinguishable from the tool not existing.

Thrown errors become error results for the caller; the SDK does that conversion, so a
handler never hand-builds one.

## The parity guarantee

`src/server.js` holds the only tool list. `listTools()` derives name/description pairs
from the same `TOOL_MODULES` array used for registration, and `src/cli.js` prints that
rather than keeping a list of its own.

`test/server.test.js` asserts that what the CLI would print matches what an MCP client
receives from `tools/list`, so the two surfaces cannot drift apart without failing the
suite.

## Related pages

* [`overview.md`](overview.md) — what this project is.
* [`../environments/setup.md`](../environments/setup.md) — running it.
