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

### Task 2 — refactor/code-structure

Landed. Files moved with `git mv`: `src/app.js` and `src/server.js` to `src/routes/`, `src/tools/` to
`src/controllers/`, `src/version.js` to `src/config/`. The old `src/index.js` became
`src/routes/primary.js` (`startPrimary`), and the parts that were not the primary went out of it:
`startWorker` to `src/routes/worker.js`, the `PORT`, `HOST`, `MCP_TRANSPORT` and `MCP_CLUSTER_WORKERS`
reads to `src/config/settings.js` beside `allowedHosts` (from `app.js`), and transport selection to a
new, short `src/index.js`. The settings are read through functions, with the same expressions as
before. `src/cli.js` imports from the new paths. No code inside a function changed beyond reading a
setting through its function.

Checked: no module imports from a folder above its own, and every relative import resolves.
`npm test` runs 37 passing and none failing, the same as before; the HTTP tests start the real entry
point, workers included. `node src/cli.js tools` lists the same four tools.

Left for task 3: `README.md`, `PROMPT.md`, the `Dockerfile` comments and the wiki pages name the old
paths; the `Dockerfile` comments also name a `src/http.js` that never existed here.

### Task 3 — docs/code-structure

Landed. Every living page names the new paths: `README.md`, `PROMPT.md` (the scaffold steps now
delete samples from `src/controllers/` and edit `src/routes/server.js`), `AGENTS.md`, the three rules
files, the repository map, and in `wiki/` the architecture, overview, setup and environment pages. The
environment table says every variable is read in `src/config/settings.js`. `wiki/information/architecture.md`
opens with the new tree and the rule that a folder imports only from the ones below it. The
`Dockerfile` comments named `node src/http.js`, a file this repository never had; the documented
override is now `npm run start:http`, which runs as written under the inherited entrypoint, and the
`EXPOSE` comment names `src/config/settings.js`. Released changelogs and earlier task records keep the
old paths.
