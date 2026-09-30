---
name: template-mode
description: Rules that apply only while this repository is still a template - PROMPT.md is the scaffolding authority, the sample tools are disposable, and the transport a scaffold inherits comes with a Host guard.
---

# Template Mode

This repository is a template until `PROMPT.md` is deleted. While that file exists,
these rules apply on top of everything else.

## PROMPT.md is the scaffolding authority

A request to initialize, scaffold, or set up a new project from this template is
answered by reading [`../../PROMPT.md`](../../PROMPT.md) and following it, not by
improvising an equivalent. Its steps are ordered and its Step 3 is exhaustive.

Changing what a scaffolded project starts with means editing `PROMPT.md`. A change to
the template's own structure that Step 3 does not know about produces projects with
leftovers - the sample tools being the obvious case.

## A scaffold inherits the transport, and the guard comes with it

A scaffolded project starts with `src/app.js` and the express transport behind it:
`POST /mcp`, `GET /healthz`, a JSON-RPC 404 elsewhere, a 405 for any other method on
`/mcp`, a 4 MB body limit, `HOST` defaulting to `0.0.0.0`, and `MCP_ALLOWED_HOSTS`
applying the SDK's `hostHeaderValidation` when it is set. The `node:http` server this
template used to have is gone, and it had **no** `Host` guard at all - see
[`../../.agents/memory/decisions/express-for-http-transport.md`](../../.agents/memory/decisions/express-for-http-transport.md),
where that is recorded as the addition it is rather than the preservation the migration
request asked for.

Two consequences, and they are the reason this section exists:

- **The guard is off unless `MCP_ALLOWED_HOSTS` is set.** Unset, empty, or
  separators-only, the middleware is not mounted and every request is served. A
  scaffolded project therefore starts unguarded, on purpose and out loud - the startup
  line says `MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied`. A
  generated repository must not assume it is protected because it has a guard, and it
  must not be made to refuse-by-default to compensate: an allow-list that guesses wrong
  takes down a working deployment. The four sibling LXAgents MCP repositories all
  behave this way, and the template exists to produce a fifth that does too.
- **`/healthz` is guarded too.** A probe using a `Host` outside the list gets a 403, so
  the setup instructions a scaffold produces have to say which name to probe with. A
  green check on a service every real caller is refused from is worse than no check.

Step 8 of `PROMPT.md` is where the scaffolding agent asks where the server will be
reachable from and records the answer. Step 3 item 7 is what keeps the wiki honest
about it, and item 2 is what keeps the README from implying a protection that is off.
Do not treat any of the three as a follow-up.

## The sample tools are disposable

`src/tools/` currently holds four samples that exist to demonstrate the four
combinations of *takes parameters* and *requires an API key*:

| Tool | Parameters | API key |
|---|---|---|
| `get_server_time` | no | no |
| `get_secure_summary` | no | yes |
| `calculate_sum` | yes (zod) | no |
| `search_secure_data` | yes (zod) | yes |

They are documentation that happens to run. Scaffolding deletes all four and replaces
them with a single `ping`.

The consequence for anything added here: **nothing outside `src/tools/` may depend on a
sample.** No shared helper that only the samples import, no test that survives their
deletion, no dependency in `package.json` that goes unused once they are gone. If a
sample needs a helper, the helper lives in the sample.

## Names in the template are placeholders

`template`, `Template`, and `@mcagents-mcp/template` are scaffolding targets, not
names to preserve. They appear in `package.json`, `src/server.js`, `src/cli.js`,
`README.md`, `wiki/environments/setup.md`, and `test/http.test.js`, and Step 3 of
`PROMPT.md` replaces each one. A new occurrence added anywhere else needs a matching
line in Step 3.

## Leaving template mode

Scaffolding removes `PROMPT.md`, the "Project Scaffolding (Template Mode)" section of
`AGENTS.md`, and that section's trigger row. This file goes with them - a scaffolded
project is not a template, and a rule about template mode left behind is a rule that
can only mislead.
