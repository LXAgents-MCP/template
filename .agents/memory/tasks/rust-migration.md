---
name: memory-tasks-rust-migration
description: Rewriting template from TypeScript to Rust on the official MCP Rust SDK (rmcp) with axum for HTTP; same tools, transports and contracts; the version stays 0.0.0.
---

# Rust migration — template

## 2026-11-14 — in progress

**Goal.** The owner asked for every repository to move from TypeScript to Rust, on the official MCP
Rust SDK (`rmcp`, github.com/modelcontextprotocol/rust-sdk) with `axum` as the web framework. This is
the template, so every project scaffolded from it starts in Rust.

**Objective.**

- `Cargo.toml` (package `template` `0.0.0`, edition 2024) builds a library and two binaries,
  `template-server` and `template`, on `rmcp` 3.10, `axum` 0.8 and `tokio`.
- One file per tool under `src/controllers/` stays; the four samples, their names, descriptions and
  schemas do not change.
- stdio and stateless streamable HTTP keep every contract the TypeScript server had: `/mcp`,
  `/healthz`, the JSON-RPC 404 and 405, the 4 MB body limit with one 400 / `-32700` for oversized and
  malformed bodies, no `X-Powered-By`, the `MCP_ALLOWED_HOSTS` guard (off when unset), `HOST`,
  `PORT`, `MCP_CLUSTER_WORKERS`, and the SIGINT/SIGTERM drain.
- The 37 TypeScript tests are ported to `cargo test`; `cargo fmt --check` and
  `cargo clippy --all-targets -- -D warnings` are clean.
- No TypeScript, `package.json`, lockfile or `tsconfig` remains.

**Status:** in progress.

**Detail.** The version stays `0.0.0` (template-mode rule); the change ends on `release/0.0.0` with a
dated section in `wiki/logs/0/0/0/CHANGELOG.md`, no tag. Only this repository is in scope; the other
three have their own tasks. The full plan is in the project's shared reports folder as
`rust-template-plan.md`.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/rust` | — |
| 2 | Rewrite `src/` and the tests in Rust | `refactor/rust` | — |
| 3 | Docs, rules, `PROMPT.md` and `Dockerfile` for the Rust build | `docs/rust` | — |
| 4 | Release | `release/0.0.0` | — |

Branches stacked: task 1 from `master`, task `k` from task `k-1`.

## Decisions

| # | Decision |
|---|---|
| D1 | One file per tool stays: each controller exports `NAME`, `DESCRIPTION`, an `async fn handler` and `route<S>() -> ToolRoute<S>`; `src/routes/server.rs` holds `TOOL_MODULES` and folds it into rmcp's `ToolRouter`. The `#[tool_router]` macro is not used because it puts every tool in one `impl`. |
| D2 | Handlers return `Result<String, String>`; `Err` becomes an `isError` text result, as a thrown `Error` did. |
| D3 | Parameters are `serde` + `schemars` structs whose doc comments are the field descriptions. `search_secure_data` checks `query` is non-empty itself, because serde does not enforce `minLength`. |
| D4 | HTTP is rmcp's `StreamableHttpService`, stateless (`legacy_session_mode` off, JSON responses), a fresh server per request, nested in an axum `Router`. |
| D5 | The body limit and the Host guard are this repository's own axum middleware, so their answers stay exactly as before. rmcp's built-in Host check is disabled: its default is loopback-only and would refuse every request to a server bound on `0.0.0.0`. |
| D6 | `node:cluster` worker processes become tokio worker threads. `MCP_CLUSTER_WORKERS` keeps its name and default; `1` runs a single-threaded runtime. Tests for orphaned and respawned worker processes are retired: there are no child processes. |
| D7 | Environment variables are parsed once into `Settings`; the CLI overrides fields instead of setting variables. `API_KEY` is still read inside the handler at call time. |
| D8 | The shared instruction connector is not registered in this session, so `commit_conventions` and the other shared tools could not be loaded. Commits follow the format in this repository's history; the rules are not reconstructed and nothing is vendored. |
| D9 | The owner's instruction for this task sets aside the plan, pull request and merge approval gates: plan, then proceed, and merge by rebase without human review. |

## Task entries

### Task 1 — plan/rust

Landed: this record and its row in `memory-index.md`. Nothing outside `.agents/` changes.
