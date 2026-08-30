---
name: memory-state-repository-state
description: Current known state of template - what exists after the instruction-system adoption and the tool-layer refactor, and the next obvious step.
---

# Repository State

## What this repository is right now

`template` is a working dual-purpose MCP server and CLI at version `0.0.0`, and still a
template: `PROMPT.md` is present, and `src/tools/` holds four disposable samples.

## Stack

Node.js 20+, ESM, no build step. Two runtime dependencies:
`@modelcontextprotocol/sdk` and `zod`. Tests are `node --test`, no framework.

## What exists

* **Tool layer.** `src/tools/` with one file per tool, each exporting `config` and
  `handler`. Four samples cover the four combinations of parameters and API key.
  `src/server.js` imports them individually and registers each with
  `server.tool(name, description, schema?, handler)`; the old frozen `TOOLS` array is
  gone.
* **Authentication.** One server-wide key, `process.env.API_KEY`, checked inside the
  handlers that need it.
* **Surface parity.** `src/cli.js` prints `listTools()` from `src/server.js`;
  `test/server.test.js` pins the CLI list against the MCP client's `tools/list`.
* **Instruction system.** Mode B - `AGENTS.md` plus `.agents/`, resolving the shared set
  through the `lxagents-agents-base` connector. Local rules: `repository`,
  `tool-authoring`, `secrets`, `template-mode`. No overrides.
* **Documentation.** `wiki/information/` and `wiki/environments/`, all updated in the
  same commit as the code change they describe.

## What is not built

* No tool does real work - every sample returns a canned or computed value, and none
  calls an external service.
* `API_KEY` is checked for presence only. Nothing validates it against anything.
* The HTTP transport is stateless and unauthenticated; `/healthz` and `/mcp` are open.
* No CI workflow, no linter, no formatter.
* No release has been logged - `wiki/logs/` is empty and `package.json` is at `0.0.0`.

## Shared set

Resolved through the `lxagents-agents-base` MCP connector. Nothing shared is vendored
here, and there are no overrides - see
[`../../index/root-index.md`](../../index/root-index.md).

## Next obvious step

Teach `PROMPT.md` to strip the four samples back to a single `ping`, then cut the first
release - tasks 4 and 5 of
[`../tasks/mcp-tools-refactor.md`](../tasks/mcp-tools-refactor.md).
