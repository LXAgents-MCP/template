---
name: memory-tasks-rust-migration
description: Rewriting template from TypeScript to Rust on the official MCP Rust SDK (rmcp) with axum for HTTP; same tools, transports and contracts; the version stays 0.0.0.
---

# Rust migration — template

## 2026-11-14 — complete

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

**Status:** complete. The pull requests, merged in order by rebase, are in the table.

**Detail.** The version stays `0.0.0` (template-mode rule); the change ends on `release/0.0.0` with a
dated section in `wiki/logs/0/0/0/CHANGELOG.md`, no tag. Only this repository is in scope; the other
three have their own tasks. The full plan is in the project's shared reports folder as
`rust-template-plan.md`.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/rust` | [#28](https://github.com/LXAgents-MCP/template/pull/28) |
| 2 | Rewrite `src/` and the tests in Rust | `refactor/rust` | [#29](https://github.com/LXAgents-MCP/template/pull/29) |
| 3 | Docs, rules, `PROMPT.md` and `Dockerfile` for the Rust build | `docs/rust` | [#30](https://github.com/LXAgents-MCP/template/pull/30) |
| 4 | Release | `release/0.0.0` | [#31](https://github.com/LXAgents-MCP/template/pull/31) |

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

### Task 2 — refactor/rust

Landed. `package.json`, `package-lock.json`, both `tsconfig` files, every `.ts` file under `src/`
and `test/` are deleted; `Cargo.toml`, `Cargo.lock`, `src/**/*.rs` and `tests/**/*.rs` replace them.
`.gitignore` ignores `/target/` instead of `node_modules/` and `dist/`.

- Layout keeps the folders: `src/{main,cli,lib}.rs`, `routes/{mod,server,app,http,stdio}.rs`,
  `controllers/` (four samples), `models/tool.rs`, `config/{settings,version}.rs`.
  `routes/primary.ts` and `routes/worker.ts` became `routes/http.rs` plus the runtime choice in
  `routes/mod.rs` (D6).
- Dependencies: `rmcp` 3.10 (`server`, `macros`, `transport-io`,
  `transport-streamable-http-server`), `axum` 0.8, `tokio` 1, `serde`, `serde_json`, `schemars`
  1, `chrono` 0.4, `tokio-util` 0.7. Dev: `rmcp` client features and `reqwest` 0.13.
- Tests: 5 unit tests, `tests/server.rs` (8, in memory), `tests/stdio.rs` (6, the binary over
  stdio, API-key cases with a per-child environment), `tests/http.rs` (25, the binary over a
  socket). Retired with the worker processes: the orphan, respawn and primary-relay tests.
  New: an empty `query` is refused, an empty `API_KEY` counts as none, SIGTERM drains like
  SIGINT, a port in use exits 1, the Host guard covers `POST /mcp`.

Checked: `cargo fmt --check` clean; `cargo clippy --all-targets -- -D warnings` clean;
`cargo test` 44 passing, none failing; `cargo audit` no advisories (166 crates);
`template --version` prints `0.0.0` and `template tools` lists the four samples.

### Task 3 — docs/rust

Landed. No code under `src/` or `tests/` changes.

- `Dockerfile`: a `rust:1-slim-bookworm` build stage (`cargo build --release --locked --bins`)
  and a `debian:bookworm-slim` runtime with the two binaries, a non-root user, `EXPOSE 3000`,
  `CMD ["template-server"]`, no `ENTRYPOINT`. No Docker daemon was available here, so the build
  stage was replayed by hand in a scratch directory with only `Cargo.toml`, `Cargo.lock` and
  `src/`: it built, `/healthz` answered `{"server":"template","status":"ok","version":"0.0.0"}`,
  and `template tools` listed four tools.
- `README.md`, `AGENTS.md`, `PROMPT.md` (the Rust `ping.rs` scaffold, `Cargo.toml` renames,
  `cargo` checks), `wiki/information/{overview,architecture}.md`,
  `wiki/environments/{setup,env}.md`.
- `.agents/rules/{repository,tool-authoring,secrets,template-mode}.md`: Rust paths, commands
  and examples. *What must not be introduced* now bans TypeScript/JavaScript, `unsafe` and
  warnings in place of JavaScript and a second compiler. These describe the new stack; no new
  rule was added.
- `.agents/wiki/context/repository-map.md` and `.agents/memory/state/repository-state.md`
  rewritten; new decision `.agents/memory/decisions/rust-rmcp-axum.md` and its index row.
- Not changed: earlier task records, the express decision and released changelogs, which keep
  their paths as history.

### Task 4 — release/0.0.0

Landed. No version change and no tag (template-mode rule). `wiki/logs/0/0/0/CHANGELOG.md` gains the
dated *2026-11-14 — Rust* section, and the `0.0.0` row in `logs-index.md` names it. This commit
fills the `PR` column and closes the record.

Final state checked on this branch: `cargo fmt --check` clean, `cargo clippy --all-targets -- -D
warnings` clean, `cargo test` 44 passing, `cargo audit` no advisories.
