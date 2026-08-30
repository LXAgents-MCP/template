---
name: memory-state-repository-state
description: Current known state of template - what exists after the instruction-system adoption, and the next obvious step.
---

# Repository State

## What this repository is right now

`template` is a working dual-purpose MCP server and CLI at version `0.0.0`, and still a
template: `PROMPT.md` is present at the root.

## Stack

Node.js 20+, ESM, no build step. One runtime dependency,
`@modelcontextprotocol/sdk`. Tests are `node --test`, no framework.

## What exists

* **Server.** `src/server.js` declares a frozen `TOOLS` array holding one tool, `ping`,
  and registers each entry through `registerTool` with read-only annotations.
* **Transports.** stdio and stateless streamable HTTP, with `/healthz`, in `src/index.js`.
* **CLI.** `help`, `version`, `tools`, `serve` in `src/cli.js`, reading the same `TOOLS`
  array the server registers.
* **Instruction system.** Mode B - `AGENTS.md` plus `.agents/`, resolving the shared set
  through the `lxagents-agents-base` connector. Local rules: `repository`,
  `template-mode`. No overrides.
* **Documentation.** `wiki/information/` and `wiki/environments/`.

## What is not built

* The tool layer is a single array in one file. There is no per-tool module, no
  parameter validation, and no authentication of any kind.
* `ping` does no real work; nothing calls an external service.
* No CI workflow, no linter, no formatter.
* No release has been logged - `wiki/logs/` is empty and `package.json` is at `0.0.0`.

## Shared set

Resolved through the `lxagents-agents-base` MCP connector. Nothing shared is vendored
here, and there are no overrides - see
[`../../index/root-index.md`](../../index/root-index.md).

## Next obvious step

Replace the frozen `TOOLS` array with a per-file tool layer that supports optional
parameters and optional authentication - that is task 3 of
[`../tasks/mcp-tools-refactor.md`](../tasks/mcp-tools-refactor.md).
