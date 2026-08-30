# Architecture

Four source files. There is no framework, no build step, and no code generation.

```
src/
  index.js     entry point: picks a transport, owns the HTTP server
  server.js    the TOOLS declaration and createServer()
  cli.js       the CLI: help, version, tools, serve
  version.js   reads the version out of package.json at import
```

## Entry point and transports

`src/index.js` reads `MCP_TRANSPORT` and serves either way:

* **stdio** (default) — one `McpServer` connected to a `StdioServerTransport` for the
  life of the process.
* **streamable HTTP** — a plain `node:http` server exposing `GET /healthz` and
  `POST /mcp`.

The HTTP transport is **stateless**: a fresh `McpServer` and transport are built for
each request and closed when the response closes. That is deliberate — `McpServer`
holds per-connection state, so hoisting one to module scope would leak state between
unrelated callers.

### stdout belongs to the protocol

On stdio, stdout **is** the JSON-RPC channel. Server-side logging goes to stderr and
`serve` prints nothing of its own; only CLI commands write to stdout. A `console.log`
on the server path corrupts the stream, and the client reports a parse error that
points nowhere useful.

## Tools

`src/server.js` declares tools in one frozen array:

```js
export const TOOLS = Object.freeze([
  {
    name: "ping",
    title: "Ping",
    description: "Return pong, to prove the server is reachable. Takes no arguments.",
    run: async () => "pong",
  },
]);
```

`createServer()` walks that array and calls `registerTool` for each entry, attaching
read-only annotations and wrapping the entry's `run` in the MCP content envelope.

Every tool is currently argument-free: `run` takes nothing, and no entry declares an
input schema.

## The parity guarantee

`src/server.js` holds the only declaration. `src/cli.js` imports `TOOLS` rather than
keeping a list of its own, and `test/server.test.js` asserts that the names the CLI
would print match what an MCP client receives from `tools/list`, so the two surfaces
cannot drift apart without failing the suite.

## Related pages

* [`overview.md`](overview.md) — what this project is.
* [`../environments/setup.md`](../environments/setup.md) — running it.
