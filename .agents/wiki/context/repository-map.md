---
name: agent-wiki-context-repository-map
description: Orientation for template - what lives where in the Rust crate, how to build and test it, the two surfaces, and the gotchas that bite first.
---

# Repository Map

Read this before touching anything in `template`.

## What this repository is

An MCP server and a CLI over one implementation, and the template every other LXAgents
MCP repository is scaffolded from. Rust, edition 2024, MSRV 1.88, on the official MCP Rust
SDK `rmcp` 3.10 with `axum` 0.8 for HTTP and `tokio` underneath. One Cargo package builds a
library and two binaries.

* Remote: `LXAgents-MCP/template`, default branch `master`.
* Package: `template` (Cargo, `publish = false`), version `0.0.0`.
* Binaries: `template` (CLI, `src/cli.rs`) and `template-server` (MCP server, `src/main.rs`).

## Layout

```
AGENTS.md                     entry point, connector bootstrap, trigger table
PROMPT.md                     scaffolding procedure - present only while this is a template
Cargo.toml                    package, lib, both bins, dependencies
Cargo.lock                    committed: the package ships binaries
Dockerfile                    rust build stage, debian-slim runtime with the two binaries
src/
  lib.rs                      module tree; #![forbid(unsafe_code)]
  main.rs                     template-server: Settings::from_env(), routes::run()
  cli.rs                      the CLI: help, --version, tools, serve [--http|--stdio] [--port N]
  routes/
    mod.rs                    run(): builds the tokio runtime and starts stdio or HTTP
    server.rs                 TemplateServer (ServerHandler), TOOL_MODULES, list_tools(), SERVER_ID
    app.rs                    the axum Router as a pure factory - builds, never listens; body and Host guards
    http.rs                   binds HOST:PORT, startup lines, drain on SIGINT/SIGTERM
    stdio.rs                  serves stdin/stdout
  controllers/
    mod.rs                    one `pub mod` per tool
    get_server_time.rs        sample: no parameters, no API key
    get_secure_summary.rs     sample: no parameters, API key required
    calculate_sum.rs          sample: typed parameters, no API key
    search_secure_data.rs     sample: typed parameters, API key required
  models/
    tool.rs                   the tool module contract: ToolModule, ToolOutcome, schema helpers
  config/
    settings.rs               every environment variable the server reads, into Settings
    version.rs                VERSION = env!("CARGO_PKG_VERSION")
tests/
  common/mod.rs               start the binary, wait for output, raw HTTP with a chosen Host
  server.rs                   in memory: registration, schemas, validation, CLI parity
  stdio.rs                    the binary over stdio: API-key behaviour, no echo, the CLI binary
  http.rs                     the binary over a socket: routes, guards, body limit, threads, drain
target/                       build output, git-ignored
wiki/                         human documentation
.agents/                      this set - rules, agent wiki, memory, indexes
```

## Commands

| Command | What it does |
|---|---|
| `cargo build` / `cargo build --release` | Builds the library and both binaries into `target/`. |
| `cargo test` | Unit tests, then `tests/server.rs`, `tests/stdio.rs` and `tests/http.rs`. The last two run the binaries cargo builds for the test run. |
| `cargo fmt --check` | Formatting. |
| `cargo clippy --all-targets -- -D warnings` | Lints, with warnings as errors. |
| `cargo run --bin template -- tools` | Lists registered tools through the CLI surface. |
| `cargo run --bin template-server` | Serves over stdio. |
| `MCP_TRANSPORT=http cargo run --bin template-server` | Serves over streamable HTTP on `HOST`:`PORT` (default `0.0.0.0:3000`). |
| `npx @modelcontextprotocol/inspector ./target/debug/template-server` | MCP Inspector against the stdio server. |

## Environment variables

| Variable | Read by | Effect |
|---|---|---|
| `API_KEY` | tool handlers, at call time | The single server-wide key. Absent or empty, the authenticated tools return an error result. |
| `MCP_TRANSPORT` | `src/config/settings.rs` | `stdio` (default) or `http` (Streamable HTTP on `/mcp`). |
| `PORT` | `src/config/settings.rs` | HTTP port, default `3000`. |
| `HOST` | `src/config/settings.rs` | Interface the HTTP transport binds, default `0.0.0.0` — every IPv4 interface. |
| `MCP_ALLOWED_HOSTS` | `src/config/settings.rs` | Comma-separated `Host` allow-list for the HTTP transport. **Unset means none is applied**; the server says so on startup. |
| `MCP_CLUSTER_WORKERS` | `src/config/settings.rs` | HTTP worker threads, default one per CPU. **`1` runs a single-threaded runtime.** stdio always uses one thread. |

`MCP_ALLOWED_HOSTS` is a filter, not a credential — it decides which `Host` values are
answered, not who is asking. Its off-by-default state is the deliberate one, matching the
sibling repositories.

## The two surfaces

`TOOL_MODULES` in `src/routes/server.rs` is the only tool list. `src/cli.rs` prints
`list_tools()` from it rather than keeping its own, and `tests/server.rs` asserts that what the
CLI would print matches what an MCP client receives from `tools/list`. Adding a tool in one
place therefore adds it in both.

## Gotchas

* **stdout is the protocol.** On stdio, a `println!` anywhere on the server path corrupts the
  JSON-RPC stream. Log with `eprintln!`. Only CLI commands print.
* **serde does not enforce schemars constraints.** `minLength`, `range` and the like are
  advertised only; the handler has to check them (`search_secure_data` does).
* **The API key is read inside handlers**, never into `Settings` - see
  [`../../rules/secrets.md`](../../rules/secrets.md).
* **A fresh `TemplateServer` per HTTP request.** `StreamableHttpService`'s factory builds one
  each time, in stateless mode. Do not hand it a shared instance.
* **rmcp's own Host check is disabled on purpose.** Its default list is loopback-only and would
  refuse every request to a server bound on `0.0.0.0`. The guard that applies is the axum
  middleware in `src/routes/app.rs`; do not turn rmcp's back on beside it.
* **rmcp would answer an oversized body with 413.** The body guard in `src/routes/app.rs` runs
  first so oversized and malformed bodies keep their one 400 / `-32700`. Keep it ahead of the
  service.
* **`src/routes/app.rs` must not bind.** It is a factory; `src/routes/http.rs` owns the port.
* **No test touches its own environment.** `set_var` is `unsafe` in edition 2024 and racy
  across parallel tests; a test that needs a variable starts the binary with it.
* **An unset `MCP_ALLOWED_HOSTS` is the guard being off.** Not "allow nothing". Do not "fix"
  this by refusing everything.
* **The four tools in `src/controllers/` are samples** and are deleted at scaffold time.
  Nothing outside that folder may depend on them - see
  [`../../rules/template-mode.md`](../../rules/template-mode.md).
* **The version comes from `Cargo.toml` at compile time.** Changing it means a rebuild, and
  while this is a template it never changes.

## Where the conventions come from

Branching, commits, pull requests, the task workflow and the creators are **not** in
this repository. They are served by the `lxagents-agents-base` connector and read as
`agents://` resources. This repository carries only what is its own.
