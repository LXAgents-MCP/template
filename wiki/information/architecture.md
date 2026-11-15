# Architecture

Two binaries over one library, in the same folders `lxagents-github`, `lxagents-gitlab` and
`cli` use. The server is built on the official MCP Rust SDK, `rmcp`, with `axum` for HTTP and
`tokio` underneath. There is no code generation beyond what `rmcp`'s and `schemars`' derives do.

```
Cargo.toml          package template 0.0.0: lib `template`, bins `template-server` and `template`
src/
  lib.rs            the module tree, and #![forbid(unsafe_code)]
  main.rs           template-server: read Settings, run the transport they name
  cli.rs            template: help, --version, tools, serve
  routes/
    mod.rs          run(): builds the tokio runtime and starts stdio or HTTP
    server.rs       TemplateServer (the rmcp ServerHandler), TOOL_MODULES, list_tools()
    app.rs          the axum Router, as a pure factory - builds, never listens
    http.rs         binds HOST:PORT, prints the startup lines, drains on a signal
    stdio.rs        serves stdin/stdout
  controllers/      one file per tool
  models/
    tool.rs         the tool module contract
  config/
    settings.rs     every environment variable the server reads, parsed once into Settings
    version.rs      VERSION, from Cargo.toml at compile time
tests/              in memory, over stdio and over a real socket
```

Each file has one job, and a folder imports only from the folders below it in the tree:
`routes/` from `controllers/`, `models/` and `config/`, `controllers/` from `models/`.
`config/settings.rs` is the one place an environment variable is read (`API_KEY` aside, see
below), so a new knob has one home and the rest of the code receives values. The CLI's `serve`
flags override fields on `Settings` rather than writing to the environment.

`app.rs` and `http.rs` are split on purpose. `app.rs` returns an axum `Router` and nothing else -
it does not listen and does not read `MCP_CLUSTER_WORKERS`. A file that both builds the app and
binds a port cannot be reasoned about, or tested, without binding one.

## Entry point and transports

`src/main.rs` reads `Settings` and hands them to `routes::run`, which serves either way:

* **stdio** (default) - one `TemplateServer` served over rmcp's stdio transport for the life of
  the process, on a single-threaded runtime. stdout is the JSON-RPC channel.
* **streamable HTTP** - an axum application exposing `GET /healthz` and `POST /mcp`, and nothing
  else, on a runtime with `MCP_CLUSTER_WORKERS` threads.

The HTTP transport is **stateless**: `/mcp` is rmcp's `StreamableHttpService` with legacy
sessions off and JSON responses on, and its factory builds a fresh `TemplateServer` for each
request. An rmcp service holds per-connection state, so sharing one would leak state between
unrelated callers.

The bind is named rather than implied: `HOST`, default `0.0.0.0`, printed on the startup line.

### The `Host` guard

`MCP_ALLOWED_HOSTS` guards what reaches the port. When it is set, every request - `/healthz`
included - is matched against it first, and a `Host` outside the list is refused with a 403
carrying a JSON-RPC error, `Invalid Host: {hostname}`. The match is port-agnostic and
case-insensitive, because a client reaching the server through a proxy sends `host:port`. The
guard is this repository's own axum middleware in `src/routes/app.rs`.

**Unset means no allow-list is applied**, and the startup line says so. The middleware is not
mounted at all in that state rather than mounted with an empty list, because "no allow-list
configured" and "refuse everything" are different answers.

rmcp has a Host check of its own, and it is **turned off** here. Its default list is
loopback-only, which would refuse every request to a server bound on `0.0.0.0`, and two guards
with two different answers is one too many.

### The body limit

`BODY_LIMIT_BYTES` is 4 MiB. A middleware on `/mcp` reads the body under that limit and checks it
is JSON before rmcp sees it, so an oversized body and a malformed one are both answered **400 /
`-32700`**, `Parse error: request body is not valid JSON`. The collapse is deliberate and older
than this implementation: a client was never taught to tell the two apart. No response carries
`X-Powered-By` or `Server`.

Any other method on `/mcp` gets a JSON-RPC 405, `{METHOD} is not supported in stateless mode`;
any other path a JSON-RPC 404, `Not found: {path}`.

## Workers

On HTTP the tokio runtime has `MCP_CLUSTER_WORKERS` worker threads (default: the number of CPUs
the process was given), and tokio spreads connections across them. There is one process and one
listener, so there is nothing to fork, respawn or orphan. `MCP_CLUSTER_WORKERS=1` runs a
single-threaded runtime. The variable keeps the name it had when the TypeScript server forked
`node:cluster` processes, so deployments and the sibling repositories keep one contract.

### Shutdown drains before it closes

`SIGINT` and `SIGTERM` do the same thing: the line `{id} {version} SIGINT, draining` goes to
stderr, the listener stops accepting, and axum waits for the requests in flight. The wait is
capped at five seconds, and a second signal is the operator saying they have stopped waiting:
either ends the process at once. The exit status is 0 either way. Because the transport is
stateless there is no session to drain - what drains is the requests.

### stdout belongs to the protocol

On stdio, stdout **is** the JSON-RPC channel. Server-side logging goes to stderr and `serve`
prints nothing of its own; only CLI commands write to stdout.

## The tool layer

Each tool is one file at `src/controllers/{tool_name}.rs`:

```rust
pub const NAME: &str = "calculate_sum";
pub const DESCRIPTION: &str = "Add two numbers and return the sum. Requires no API key.";

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculateSumArgs {
    /// The first addend.
    pub a: f64,
    /// The second addend.
    pub b: f64,
}

pub async fn handler(Parameters(CalculateSumArgs { a, b }): Parameters<CalculateSumArgs>) -> ToolOutcome {
    Ok((a + b).to_string())
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(declaration(NAME, DESCRIPTION, parameters::<CalculateSumArgs>()), handler)
}
```

A tool with no arguments has no args struct, a handler that takes none, and
`no_parameters()` in place of `parameters::<…>()`. The helpers and the `ToolOutcome` type are in
`src/models/tool.rs`.

`src/routes/server.rs` lists each module's `route` in one `TOOL_MODULES` array and folds them
into rmcp's `ToolRouter`. The `#[tool_router]` macro is not used, because it would put every
tool in one `impl` block.

Each field's `///` doc comment becomes its `description` in the advertised JSON Schema - that
text is what the calling model reads. rmcp deserializes the arguments before the handler runs,
and a call with the wrong types is answered with an error result.

## Authentication

There is one key for the whole server, `API_KEY`, and it is read **inside the handler** of each
tool that needs it:

```rust
if std::env::var("API_KEY").map_or(true, |key| key.is_empty()) {
    return Err("search_secure_data requires an API key. Set …".to_owned());
}
```

Reading it at call time means a process whose environment changes still sees the current value.
Registration never depends on the key: every tool is advertised whether or not one is set,
because a server that hides its authenticated tools reports "no such tool", which is
indistinguishable from the tool not existing.

A handler's `Err` becomes a tool result with `isError: true`; rmcp does that conversion, so a
handler never hand-builds one.

## The parity guarantee

`src/routes/server.rs` holds the only tool list. `list_tools()` derives name/description pairs
from the same `TOOL_MODULES` array the server registers, and `src/cli.rs` prints that rather
than keeping a list of its own. `tests/server.rs` asserts the CLI list matches what an MCP client
receives from `tools/list`, and `tests/http.rs` asserts it again over a socket.

## Related pages

* [`overview.md`](overview.md) - what this project is.
* [`../environments/setup.md`](../environments/setup.md) - running it.
