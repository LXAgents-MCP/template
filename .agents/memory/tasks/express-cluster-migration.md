---
name: memory-tasks-express-cluster-migration
description: Task record for replacing the node:http transport with an express application mapped strictly to POST /mcp, adding the MCP_ALLOWED_HOSTS Host guard this template never had, and for answering requests from cluster workers on one port.
---

# Task: express transport and cluster workers

Three branches, stacked. Local commits only; nothing is pushed from them. **No version
change** — the version needs the owner, and so does the release log.

## The plan

| # | Task | Branch | Scope |
|---|---|---|---|
| 1 | The record | `chore/express-cluster-plan` | This file, and the `.gitignore` rule that keeps the plan out of history. |
| 2 | express transport + guard | `feat/express-transport` | `node:http` → express at `/mcp`, **and the `Host` guard this template never had**. |
| 3 | cluster workers | `feat/cluster-workers` | A `cluster` primary forking workers onto the one `PORT`. |

The working plan is untracked, under `.agents/plans/`, and is deleted or abandoned when
the work merges. Where the two disagree, this record wins.

## The instruction that could not apply here

The request said the current implementation includes a custom DNS-rebinding guard
utilizing `MCP_ALLOWED_HOSTS`, and that this exact security logic must be preserved.

**In this repository there is no such guard.** `src/index.js` reads `PORT` and
`MCP_TRANSPORT`; it never reads `MCP_ALLOWED_HOSTS`, and no route checks the `Host`
header. The instruction is vacuous here, and describing an existing guard as preserved
would be a false claim in a file a reviewer reads.

The owner decided: **add the guard**, adopting the shape the sibling repositories use,
so every project scaffolded from this template inherits it. So this is the one
repository of the five where the guard is **new behaviour rather than preserved
behaviour**, and this record says so rather than letting the diff imply otherwise.

Its semantics are fixed by the owner's instruction and match the other four exactly:
unset, empty, or separators-only → the middleware is **not mounted** and every request
is served; set → the SDK's `hostHeaderValidation` refuses a host outside the list with
403 and a JSON-RPC body, port-agnostic, bracketed IPv6 matched as `[::1]`, and
`/healthz` guarded too.

## Baseline

`npm test` before any change: **10 tests, 10 pass, 0 fail** (Node 24.21.0, `node
--test`, no framework).

There is **no `test/http.test.js`** in this repository, so the transport is untested
today and the new guard would land unverified. Task 2 adds one.

## What is preserved

| Property | What happens |
|---|---|
| `MCP_TRANSPORT` as the selector, default `stdio` | Preserved — nothing adds a second entry point, so a scaffold gets one obvious door |
| `PORT` default 3000 | Preserved |
| The four sample tools and their API-key behaviour | Preserved — `test/server.test.js` pins them and this change must not move them |
| `MCP_TRANSPORT=stdio` never forks | Preserved; stdout is the JSON-RPC channel there |

## What is new

- `src/app.js` — the express application as a pure factory.
- `HOST`, defaulting to `0.0.0.0` — this repository currently binds every interface by
  omission rather than by decision.
- **The `MCP_ALLOWED_HOSTS` guard.**
- `test/http.test.js`.
- `MCP_CLUSTER_WORKERS` — worker count, defaulting to `availableParallelism()`. `1`
  disables forking.
- The `PROMPT.md` and `.agents/rules/template-mode.md` text a scaffold inherits. If
  those are not corrected, the next repository generated from this template ships
  without knowing it has a guard.

## Out of scope

The four sample tools, the version, the release log, and a `Dockerfile` — there is
none, so the "do not modify the Dockerfile" rule has nothing to act on, and writing a
container image is a separate decision rather than a side effect of a transport
change.

## Test counts

| Point | Tests | Pass | Fail |
|---|---|---|---|
| Baseline, before any change | 10 | 10 | 0 |
| After task 2 — `feat/express-transport` | 29 | 29 | 0 |
| After task 3 — `feat/cluster-workers` | *pending* | | |

---

### Task 1 — `chore/express-cluster-plan`

Created this record with the confirmed task list, before any of the work. Also lands
the owner's `.gitignore` addition of `/.agents/plans/`, which this repository was
missing: it is the only thing keeping a plan out of history, and the first plan file
in this repository is created in the same commit. Registered in
[`.agents/index/memory-index.md`](../../index/memory-index.md) in this commit.

Task 2 branches from this branch and adds its own entry here in its own commit.

---

### Task 2 — `feat/express-transport`

`node:http` → express at `POST /mcp`, **and the `Host` guard this repository never
had**.

`src/app.js` is new and holds the whole surface as a pure factory: `POST /mcp`,
`GET /healthz`, a JSON-RPC 405 for any other method on `/mcp`, a JSON-RPC 404 for
everything else, and a four-argument error handler that answers an oversized body and a
malformed one with the same 400 / `-32700`. It does not listen. `src/index.js` keeps the
transport switch, the port, the interface, the startup lines and the shutdown, and
`readBody` and `rpcError` are gone with the server they belonged to.

**The guard is new behaviour, not preserved behaviour.** Nothing in the previous tree
read `MCP_ALLOWED_HOSTS` or looked at the `Host` header, so this commit adds a control
the repository did not have. The four sibling repositories already had it; the code is
deliberately identical to theirs so a scaffold inherits the same control, and
`PROMPT.md`, `.agents/rules/template-mode.md`, `README.md`, `wiki/` and the agent wiki
all say so in the same words. A guard documented as "preserved" would be a false claim
in a file a reviewer reads.

Its off-by-default semantics are preserved-from-the-siblings, not invented here, and
`test/http.test.js` asserts them: unset, empty, or separators-only and the middleware is
**not mounted at all** — every request is served and the startup line says so. Set, and
a `Host` outside the list is refused with 403 and a JSON-RPC body, port-agnostically,
with `/healthz` guarded as well.

Also new: `HOST`, defaulting to `0.0.0.0` and named on the startup line. The
`node:http` server bound every interface by omission; an exposure nobody decided is an
exposure nobody has reviewed.

`express@^5.2.1` was already resolved in the lockfile transitively through the SDK, so
the lockfile diff is the direct-dependency marking and nothing else — no version moved.

`test/server.test.js` is **untouched** and still passes. The four sample tools, their
schemas and their API-key behaviour are exactly as they were; `test/http.test.js` adds
19 tests over a real socket and re-checks the four names and descriptions against the
in-memory transport, so the two doors onto this server cannot drift.

**No `Dockerfile` exists in this repository and none was added.** The instruction not to
modify one had nothing to act on, and writing a container image is a separate decision
rather than a side effect of a transport change.
