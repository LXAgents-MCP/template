---
name: memory-state-repository-state
description: Current known state of template - the Rust crate on rmcp and axum, what exists, what is not built, and the next obvious step.
---

# Repository State

## What this repository is right now

`template` is a working dual-purpose MCP server and CLI, and still a template: `PROMPT.md` is
present, and `src/controllers/` holds four disposable samples. Its version is `0.0.0` and stays
there while it is a template (owner's rule, 2026-11-11); see `.agents/rules/template-mode.md`.

## Stack

Rust, edition 2024 (since 2026-11-14; strict TypeScript before that, JavaScript before 2026-11-11).
One Cargo package: a library and two binaries, `template-server` and `template`. Runtime
dependencies: `rmcp` 3.10 (the official MCP Rust SDK), `axum` 0.8, `tokio`, `serde`,
`serde_json`, `schemars`, `chrono`, `tokio-util`. Tests are `cargo test`, no framework.

## What exists

* **Tool layer.** `src/controllers/` with one file per tool, each exporting `NAME`,
  `DESCRIPTION`, a handler and `route()`. Four samples cover the four combinations of
  parameters and API key. `TOOL_MODULES` in `src/routes/server.rs` is the only tool list.
* **Authentication.** One server-wide key, `API_KEY`, read inside the handlers that need it.
* **Transports.** stdio through rmcp, and stateless streamable HTTP through rmcp's
  `StreamableHttpService` inside an axum `Router` (`src/routes/app.rs`): `POST /mcp`,
  `GET /healthz`, a JSON-RPC 405 for any other method on `/mcp`, a JSON-RPC 404 elsewhere, a
  4 MB body guard whose oversized and malformed answers are the same 400 / `-32700`, and the
  `MCP_ALLOWED_HOSTS` guard - **off when unset**, announced on startup, covering `/healthz`.
  `HOST` defaults to `0.0.0.0`. See
  [`../decisions/rust-rmcp-axum.md`](../decisions/rust-rmcp-axum.md).
* **Worker threads.** `MCP_CLUSTER_WORKERS` sizes the tokio runtime, default one thread per
  CPU, `1` single-threaded. stdio always runs on one thread. SIGINT/SIGTERM drain, capped at
  five seconds; a second signal exits at once.
* **Surface parity.** `src/cli.rs` prints `list_tools()`; `tests/server.rs` pins it against the
  MCP client's `tools/list` in memory, and `tests/http.rs` pins it again over a socket.
* **Instruction system.** Mode B - `AGENTS.md` plus `.agents/`, resolving the shared set
  through the `lxagents-agents-base` connector. Local rules: `repository`,
  `tool-authoring`, `secrets`, `template-mode`. No overrides.
* **Documentation.** `wiki/information/` and `wiki/environments/`, and the rolling log at
  `wiki/logs/0/0/0/`.

## What is not built

* No tool does real work - every sample returns a canned or computed value, and none
  calls an external service.
* `API_KEY` is checked for presence only. Nothing validates it against anything.
* The HTTP transport is stateless and **unauthenticated**; `MCP_ALLOWED_HOSTS` filters
  which `Host` values are answered, not who is asking, and it is off until set.
* No CI workflow, no linter, no formatter.

## Shared set

Resolved through the `lxagents-agents-base` MCP connector. Nothing shared is vendored
here, and there are no overrides - see
[`../../index/root-index.md`](../../index/root-index.md).

## Next obvious step

Scaffold a real project from this template (follow `PROMPT.md`), or, if the template
itself is the thing being improved, add CI that runs `cargo fmt --check`, `cargo clippy` and
`cargo test` on push - the suite is
the only thing currently holding the two surfaces together, and nothing runs it
automatically.
