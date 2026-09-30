# 0.2.0

**Released:** 2026-09-30

The HTTP transport becomes an express application, a `node:cluster` primary forks
workers that share the one `PORT`, and the template gains the `MCP_ALLOWED_HOSTS` guard
and `HOST` support it did not have.

This is the one repository of the five where the guard is **new behaviour rather than
preserved behaviour**. `src/index.js` read `PORT` and `MCP_TRANSPORT` and never read
`MCP_ALLOWED_HOSTS`; no route checked the `Host` header. It is added here, in the shape
the siblings use, so every project scaffolded from this template inherits it. Saying so
plainly matters more than usual: a plan that described a new guard as a preserved one
would be making a claim about this repository that was false until this release.

Nothing a caller does today changes. With `MCP_ALLOWED_HOSTS` unset — the default — no
guard is mounted and every request is served, exactly as before.

## Added

- **The `MCP_ALLOWED_HOSTS` guard.** New in this repository. Its semantics match the
  other four exactly:
  - unset, empty, or separators-only → the middleware is **not mounted at all**, every
    request is served, and the startup line says so
  - set → the SDK's `hostHeaderValidation` refuses a host outside the list with **403**
    and a JSON-RPC body, port-agnostic, bracketed IPv6 matched as `[::1]`
  - mounted **above the body parser and above every route**, `/healthz` included

- **`HOST` support, defaulting to `0.0.0.0`,** matching the other four. New: this
  repository previously bound every interface by omission rather than by decision.

- **Cluster workers.** The HTTP transport forks `MCP_CLUSTER_WORKERS` processes —
  `os.availableParallelism()` by default — each binding the same `PORT` through the
  cluster's shared handle. `MCP_CLUSTER_WORKERS=1` means no fork at all, which is what
  makes it bisectable against the previous commit. The primary binds nothing, so the
  startup line prints once per worker; it relays the signal and waits for the last
  worker, and workers exit on `disconnect`.

  **stdio never forks.** stdout is the JSON-RPC channel there, and a worker's copy of
  it would corrupt the stream.

- `test/http.test.js`, **new** — this repository had no HTTP test at all, so the
  transport was untested and the guard would otherwise have landed unverified.

## Changed

- **The transport is an express application.** `src/index.js` no longer builds a
  `node:http` server and no longer hand-parses request bodies. The application is
  `src/app.js`, and the MCP endpoint is strictly `POST /mcp`, with `GET /healthz`
  preserved.
- `express` is a direct dependency, pinned to the version the lockfile already resolved.
- `PROMPT.md` and `.agents/rules/template-mode.md`, because a scaffold inherits them.
- Docs: `wiki/information/architecture.md`, `wiki/environments/env.md`,
  `wiki/environments/setup.md`, `README.md`, and the repository map.

## What a scaffold inherits

- The guard, the cluster workers, and the `HOST` decision above, in the same shape the
  four sibling repositories use.
- The four sample tools are **not** moved by this change. `calculate_sum`,
  `get_server_time`, `search_secure_data` and `get_secure_summary` are the template's
  demonstration surface and `test/server.test.js` pins their behaviour, including that
  the authenticated tools fail descriptively with no API key and that the key is never
  echoed back.

## Not done

- **No `Dockerfile`, and none added.** There is none here, so the "do not modify the
  Dockerfile" rule has nothing to act on. Writing a container image is a separate
  decision, not a side effect of a transport change.
- No version bump was made for a scaffolded project; `PROMPT.md` leaves that to the
  project that adopts it.
