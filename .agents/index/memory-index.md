---
name: memory-index
description: Index of .agents/memory/ - repository state, decisions, and task records. Read every session so work continues rather than restarts.
---

# Memory Index

**Scope:** `.agents/memory/`
**Parent:** [`root-index.md`](root-index.md)

This index is the standing exception to the routing protocol: it is read **every
session**, because continuity depends on it. Load only the rows whose scope matches the
current request.

## State

| File | Purpose |
|---|---|
| [`../memory/state/repository-state.md`](../memory/state/repository-state.md) | Current known state: what exists, the stack, what is not built, and the next obvious step. |

## Decisions

| File | Purpose |
|---|---|
| [`../memory/decisions/harness-branch-naming.md`](../memory/decisions/harness-branch-naming.md) | Why a harness-designated branch never overrides the branching strategy. |
| [`../memory/decisions/express-for-http-transport.md`](../memory/decisions/express-for-http-transport.md) | Why the transport moved to express, and why the `MCP_ALLOWED_HOSTS` guard is new here, mounted natively, and left off when unset. |

## Tasks

| File | Purpose |
|---|---|
| [`../memory/tasks/mcp-tools-refactor.md`](../memory/tasks/mcp-tools-refactor.md) | Adopting the shared instruction set and refactoring the tool layer onto per-file modules. |
| [`../memory/tasks/express-cluster-migration.md`](../memory/tasks/express-cluster-migration.md) | Express at `POST /mcp`, cluster workers, and the `MCP_ALLOWED_HOSTS` guard this template never had — so a scaffolded project inherits it. |
| [`../memory/tasks/code-structure.md`](../memory/tasks/code-structure.md) | Giving `src/` the same layout as `github` and `gitlab`, with `src/index.js` split into settings, worker and primary; released as 0.2.1. |
| [`../memory/tasks/http-test-duplication.md`](../memory/tasks/http-test-duplication.md) | Removing the duplicated blocks Sonar flagged in `test/http.test.js` with helpers, same 37 tests; released as 0.2.2. |
| [`../memory/tasks/template-version.md`](../memory/tasks/template-version.md) | Pinning the template's own version at `0.0.0` for good; scaffolding sets a project's version. |
| [`../memory/tasks/typescript-migration.md`](../memory/tasks/typescript-migration.md) | Moving `src/` and `test/` to strict TypeScript compiled to `dist/`, dependencies on latest, a clean `npm audit`; version stays `0.0.0`. |

## Maintenance

Any file added to or removed from `.agents/memory/` is reflected here **in the same
commit**. Memory is written freely and needs no approval — see
`agents://rules/memory-policy.md`.
