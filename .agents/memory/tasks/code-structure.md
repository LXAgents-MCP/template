---
name: memory-tasks-code-structure
description: Giving the template the same src layout as github and gitlab, with src/index.js split into settings, the HTTP worker and the cluster primary, and no change a client can see; released as 0.2.1.
---

# Code structure — template

## 2026-11-11 — in progress

**Goal.** The owner asked for every repository to be checked against `CODE_PERF.md`: one job per
file, no tangled dependencies, a directory layout that scales, and no redundant logic. `src/index.js`
holds four jobs (reading the environment, choosing the transport, the HTTP worker and the cluster
primary), `src/app.js` reads the environment as well as building the app, and the layout differs from
`github`, `gitlab` and `cli`. A project scaffolded from this template inherits whatever shape it has.

**Objective.** `src/` holds `index.js`, `cli.js` and the sibling folders, each file with one job:

| Path | Holds |
|---|---|
| `src/index.js` | Choosing the transport; nothing else. |
| `src/cli.js` | The `template` command (unchanged, still the `bin`). |
| `src/routes/app.js` | The express application (was `src/app.js`); reads no environment. |
| `src/routes/server.js` | The MCP server and its tool list (was `src/server.js`). |
| `src/routes/worker.js` | One HTTP listener and its drain on a signal. |
| `src/routes/primary.js` | The cluster primary: forking, respawning, relaying a signal. |
| `src/controllers/` | One module per tool (was `src/tools/`). |
| `src/config/settings.js` | Every environment variable the server reads, in one place. |
| `src/config/version.js` | The version, read from `package.json`. |

Imports only flow downward: routes, controllers, config. `npm test` passes with the same 37 tests, and
`node src/index.js`, `npm run start:http` and `node src/cli.js` behave as before.

**Detail.** A move and a split, not a rewrite: no tool, route, status code, message, variable, default
or log line changes. Files move with `git mv` and no shim is left at an old path. The owner is away and
asked for the recommended option wherever a question comes up; the choices are below. Baseline: 37
passing.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/code-structure` | |
| 2 | Split src by role | `refactor/code-structure` | |
| 3 | Corrected documentation | `docs/code-structure` | |
| 4 | Release | `release/0.2.1` | |

Branches stacked: task 1 from `master`, task `k` from task `k-1`. The same names are used in `cli`,
`github` and `gitlab`; no chain depends on another.

## Decisions

| # | Decision |
|---|---|
| D1 | The folders are the ones `github`, `gitlab` and `cli` use, so a scaffolded project starts in the shape its siblings have. The tool list stays explicit in `routes/server.js`; discovery is a separate decision. |
| D2 | The worker and the primary are transports, so they live in `routes/`. |
| D3 | The release is `0.2.1`, a patch: `main`, `bin`, the scripts and the image's `CMD` do not move. No tag, nothing published. |
| D4 | Behaviour noticed and left alone, because changing it is not a refactor: a 500 from `POST /mcp` echoes the raw error text to the caller. |

## Task entries

### Task 1 — plan/code-structure

Landed: this record and its row in `memory-index.md`. Nothing outside `.agents/` changes. The `PR`
column is filled by the release task.
