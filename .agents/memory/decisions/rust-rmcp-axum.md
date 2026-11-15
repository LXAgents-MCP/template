---
name: memory-decisions-rust-rmcp-axum
description: Why template is Rust on rmcp and axum, why the body limit and Host guard are this repository's own axum middleware, and why node:cluster workers became tokio threads.
---

# Decision - Rust on rmcp and axum

## Context

On 2026-11-14 the owner asked for every LXAgents MCP repository to move from TypeScript to
Rust, on the official MCP Rust SDK (`rmcp`, github.com/modelcontextprotocol/rust-sdk) with
`axum` as the web framework. The template is the first of the four, and whatever it decides is
what every scaffolded project inherits.

## Decision

1. **`rmcp` serves both transports.** stdio through `rmcp::transport::stdio`, HTTP through
   `StreamableHttpService` with legacy sessions off and JSON responses on - stateless, a fresh
   `TemplateServer` per request, as the express server was.
2. **One file per tool stays**, built with `ToolRoute::new` rather than the `#[tool_router]`
   macro, which would gather every tool into one `impl` and make the samples hard to delete.
3. **The body limit and the Host guard are this repository's own axum middleware.** rmcp has
   both, but answers them differently: an oversized body is a 413 there, and its Host check
   defaults to loopback-only. Keeping our own preserves the 400 / `-32700` for oversized and
   malformed bodies, the off-by-default guard, and the `Invalid Host: …` refusal over
   `/healthz` as well as `/mcp`. rmcp's Host check is disabled so there is one guard.
4. **`node:cluster` processes became tokio worker threads.** A Rust process uses every core
   without forking, so there is no primary, no respawn and no orphan to guard against.
   `MCP_CLUSTER_WORKERS` keeps its name and default so deployments and the sibling
   repositories keep one contract; `1` runs a single-threaded runtime.
5. **Environment variables are parsed once into `Settings`**, and the CLI overrides fields on
   it. `std::env::set_var` is `unsafe` in edition 2024, so nothing sets variables at run time,
   and `#![forbid(unsafe_code)]` holds that. `API_KEY` stays a call-time read in the handler.

## Consequences

* The tests for orphaned and respawned worker processes were retired with the processes. The
  startup lines changed: `starting N HTTP worker threads on :PORT/mcp`, or
  `MCP_CLUSTER_WORKERS is 1, so the server runs on one thread.`, then one `serving over http`
  line instead of one per worker.
* `calculate_sum` formats with `f64`'s `Display`, which matches JavaScript for ordinary values
  (`42`, `0.30000000000000004`) and differs only from `1e21` up.
* The advertised schemas gain a `$schema` and `format: "double"`; names, types, required
  fields and descriptions are unchanged.

The record of the work is [`../tasks/rust-migration.md`](../tasks/rust-migration.md).
