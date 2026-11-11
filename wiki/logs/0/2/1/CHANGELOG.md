# 0.2.1

**Released:** 2026-11-11

`src/` is split by role, one job per file, in the same folders as `lxagents-github`,
`lxagents-gitlab` and `cli`. Nothing a caller sees changes: the same four sample tools, the
same routes, status codes, messages, environment variables, defaults and startup lines.
`main`, `bin`, the npm scripts and the image's `CMD` do not move.

A project scaffolded from this release starts in the new shape. `PROMPT.md` names the new
paths in its scaffold steps.

## Modified

| Before | After |
|---|---|
| `src/index.js` (entry, environment, worker, primary) | `src/index.js` chooses the transport; `src/routes/worker.js` listens and drains; `src/routes/primary.js` forks and relays signals; `src/config/settings.js` reads every environment variable |
| `src/app.js` | `src/routes/app.js`; `allowedHosts` moved to `src/config/settings.js` |
| `src/server.js` | `src/routes/server.js` |
| `src/tools/` | `src/controllers/` |
| `src/version.js` | `src/config/version.js` |

The `Dockerfile` comments named an override, `node src/http.js`, for a file this repository never
had. They now name `npm run start:http`.

## Verified

`npm test`: 37 passing, none failing, the same as before. `node src/cli.js tools` lists the same
four tools.
