---
name: repository-rules
description: Rules specific to template - the dual-surface contract, the stdout ban, where things go, and what must not be introduced.
---

# Repository Rules

`template` is a dual-purpose MCP server and CLI over one implementation, and it is
also the template other LXAgents MCP repositories are scaffolded from. Both facts
constrain what may be changed here.

## Mode and shared set

This repository is a **Mode B consumer**. The shared instruction set is resolved
through the `lxagents-agents-base` MCP connector and is never copied into this tree.
See the bootstrap block in [`../../AGENTS.md`](../../AGENTS.md).

## The two surfaces stay in step

The CLI (`template`) and the MCP server (`template-server`) are two doors onto one
implementation. A tool reachable from one is reachable from the other, with the same
name and the same description.

* Tools are declared in exactly one place: `TOOL_MODULES` in `src/routes/server.rs`.
* `src/cli.rs` never maintains its own list - it prints `list_tools()` from
  `src/routes/server.rs`.
* `tests/server.rs` pins the agreement, and `tests/http.rs` pins it again over a socket. A change that makes the two surfaces
  disagree fails the suite, and that failure is the point.

## Nothing writes to stdout except the CLI

On the stdio transport, stdout **is** the JSON-RPC channel. A stray `println!` on
the server path corrupts the protocol stream and the client reports a parse error
that names nothing useful.

* Server-side logging goes to stderr.
* `serve` prints nothing of its own.
* Only CLI commands write to stdout.

## Where things go

| Thing | Path |
|---|---|
| Tool list (`TOOL_MODULES`) and the MCP server | `src/routes/server.rs` |
| CLI commands | `src/cli.rs` (binary `template`) |
| Server entry point | `src/main.rs` (binary `template-server`) |
| The runtime and transport choice | `src/routes/mod.rs` |
| Transports: axum app, HTTP listener, stdio | `src/routes/{app,http,stdio}.rs` |
| Tool modules | `src/controllers/` |
| The tool contract shared across folders | `src/models/tool.rs` |
| Environment variables and the version | `src/config/` |
| Tests | `tests/{subject}.rs`, helpers in `tests/common/mod.rs`; unit tests beside the code |

## Commands

```bash
cargo build                                 # debug build of both binaries
cargo build --release
cargo test                                  # unit, in-memory, stdio and HTTP suites
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --bin template -- tools
cargo run --bin template-server             # stdio
MCP_TRANSPORT=http cargo run --bin template-server
npx @modelcontextprotocol/inspector ./target/debug/template-server
```

## What must not be introduced

* TypeScript or JavaScript source, `package.json`, or a Node toolchain. The server is Rust on
  `rmcp` and `axum`.
* `unsafe` code. `src/lib.rs` forbids it, and a test that wants a different environment
  starts a child process instead of calling `std::env::set_var`.
* A warning. `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check` are clean on
  every commit.
* A second source of truth for the tool list.
* Shared instruction content. If it can be read from `agents://`, it must not exist
  here as a file.

## Template mode

While `PROMPT.md` still exists at the root, this repository is a template and the
extra rules in [`template-mode.md`](template-mode.md) apply.
