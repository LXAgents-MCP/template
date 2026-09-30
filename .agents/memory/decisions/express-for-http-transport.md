---
name: memory-decisions-express-for-http-transport
description: Why the HTTP transport moved from node:http to express, and why the MCP_ALLOWED_HOSTS guard is mounted natively and left off when the variable is unset.
---

# Decision - express for the HTTP transport

## Context

The HTTP transport was a `node:http` server with two hand-written pieces around it: a
body reader that counted chunks and threw past a limit, and a JSON-RPC error helper.
Both existed because the routing, the parsing, the error mapping and the header
filtering all had to be written by hand against a framework that supplies them.

It also had **no `Host` guard at all**, which is the reason this decision is recorded
here rather than only in the task file. The four sibling LXAgents MCP servers have had
`MCP_ALLOWED_HOSTS` for some time; this template did not, so every repository scaffolded
from it would have started with an open `/mcp` and no way to close it but writing a
middleware by hand. The owner reviewed that during this migration and decided the
template should ship the guard.

## Decision

**`src/app.js` builds the express application and returns it. It does not listen.**

`src/index.js` keeps the port, the interface, the transport switch, and — after the
cluster change — the worker count. The split is not tidiness: a file that both builds
the app and binds a port cannot be reasoned about, or tested, without opening one, and
`test/http.test.js` starts the server as a real child process precisely so that the port
and the process lifetime are real.

`express@^5.2.1` was **already resolved in `package-lock.json`**, transitively through
`@modelcontextprotocol/sdk`, so promoting it to a direct dependency changed no installed
version. The lockfile diff is the direct-dependency marking and nothing else, which is
the check that would have caught a wrong claim here.

`hostHeaderValidation` is mounted natively, because the SDK middleware is Express-shaped
— it refuses by calling `res.status(code).json(body)` and hands on with `next()`. In
the sibling repositories that middleware had to be given those two methods by hand; an
express app has them, so here it is mounted and nothing is shimmed. The code is
deliberately identical to the siblings': a template that ships a *different* guard is a
template that ships a guard nobody has reviewed.

## The guard is new here, and it is off by default

**This repository is adding the guard, not preserving it.** Nothing in the previous
tree read `MCP_ALLOWED_HOSTS` or looked at the `Host` header; see the task record at
[`../tasks/express-cluster-migration.md`](../tasks/express-cluster-migration.md). Any
description of it as preserved behaviour would be false in a file a reviewer reads.

Its semantics are the siblings' and are not a matter of taste here:

- **Unset, empty, or separators-only → the middleware is not mounted at all** and every
  request is served. Not "allow nothing" — `hostHeaderValidation([])` would refuse
  everything, which is a different answer to a different question. The state is reported
  on startup, because a control that is off silently reads as present, and someone
  reading a container's startup log is the only person who can act on it.
- **Set → a `Host` outside the list is refused with 403** and a JSON-RPC body, matched
  port-agnostically, and `/healthz` is guarded too.
- Mounted **above the body parser and above every route**. An allow-list that guards
  `/mcp` and not `/healthz` is a green check on a service nothing can reach.

An oversized body and a malformed one are both answered **400 / `-32700`**, because the
hand-rolled reader this replaced threw one failure for both. Splitting them would be a
behaviour change nobody asked for.

## Consequences

- `X-Powered-By` is disabled, and this is a security control rather than an omission: it
  hands an unauthenticated caller the framework and its version.
- A 404 catches unknown paths and a 405 catches non-`POST` on `/mcp`, both in the
  JSON-RPC envelope, so a client never has to branch on content type to learn it was
  refused.
- `HOST` (default `0.0.0.0`) is new, and named on the startup line. The `node:http` server
  bound every interface by omission; an exposure that nobody decided is an exposure
  nobody has reviewed.
- `PROMPT.md` and `.agents/rules/template-mode.md` are part of this change, not a
  follow-up. They are the text a scaffolded project is built from, so a guard that is
  not described there is a guard the next repository does not know it has.
- **No `Dockerfile` exists in this repository and none was added.** Writing a container
  image is a separate decision, not a side effect of a transport change.
