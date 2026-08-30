---
name: memory-tasks-mcp-tools-refactor
description: Task record for adopting the shared instruction set and refactoring the tool layer onto per-file modules with zod schemas and optional API keys.
---

# Task Record - MCP Tools Refactor

## Goal

Make `template` a credible starting point for a real MCP server: give the repository a
working agent instruction system, and replace the single hard-coded `TOOLS` array with a
tool layer where each tool is its own file, may declare `zod` parameters, and may require
an API key.

## Objective

Done when `.agents/` exists and routes, `src/tools/` holds four sample tools covering the
four capability combinations, both surfaces (server and CLI) list the same tools, the test
suite pins that agreement plus the API-key behaviour, and `PROMPT.md` tells a scaffolding
agent how to strip the samples back to a single `ping`.

## Detail

* Single unified API key for the whole server: `process.env.API_KEY`, read inside the
  handler, never at import time.
* Tools register through `server.tool(name, description, schema?, handler)`.
* The four samples are disposable by design - `PROMPT.md` deletes them at scaffold time.
* No pull request. The user instructed "only push, never create pr", so every branch in
  the stack is pushed and left for the user to open pull requests against if they choose.

## Decisions

| Decision | Value | Why |
|---|---|---|
| Mode | B - consumer | The `lxagents-agents-base` connector resolves in this session. |
| Branches | One per task, stacked | `agents://git/branching-strategy.md` and `agents://planning/task-workflow.md` §C. See the decision record below. |
| Version | `0.1.0` | User approved the bump; first changelog is `wiki/logs/0/1/0/`. |
| License | MIT, LXAgents-MCP, 2026 | Already present and correct; left untouched. |
| Local instructions | `tool-authoring`, `secrets`, `template-mode` | Selected by the user from the proposals. |
| Tool annotations | Dropped | `server.tool(name, description, schema, cb)` has no annotations slot; the required signature wins. |

The branch question is recorded separately in
[`../decisions/harness-branch-naming.md`](../decisions/harness-branch-naming.md), because
it recurs in every harness-run session and is otherwise re-litigated each time.

## Tasks

| # | Title | Scope (one line) | Repository | Branch | Files / areas | PR |
|---|---|---|---|---|---|---|
| 1 | Task record | The confirmed plan, written before the work | `template` | `chore/mcp-tools-refactor-plan` | `.agents/memory/` | none |
| 2 | Agent instruction system | Mode B adoption of the shared set | `template` | `docs/agents-setup` | `AGENTS.md`, `.agents/`, `wiki/`, `README.md` | none |
| 3 | Tool layer refactor | Per-file tools, zod, optional API key | `template` | `refactor/tool-layer` | `src/`, `test/`, `package.json`, `.agents/rules/`, `wiki/` | none |
| 4 | Scaffolding instructions | Teach `PROMPT.md` to strip the samples | `template` | `docs/scaffolding-prompt` | `PROMPT.md` | none |
| 5 | Release | Version, changelog, index rows, close-out | `template` | `chore/release` | `package.json`, `wiki/logs/` | none |

Branches stack in dependency order: task 1 from `master`, task `k` from task `k-1`'s
branch. `PR` is `none` throughout - the user instructed push-only, so no pull request
exists to number.

### Why task 2 documents the repository before task 3 changes it

Task 2 writes the instruction and knowledge system against the repository **as it stands**
- one `ping` tool behind a frozen `TOOLS` array. Task 3 then changes that structure and
updates every page describing it **in the same commit**, per
`agents://rules/change-propagation.md`.

The alternative - documenting task 3's structure up front - would leave the `docs/agents-setup`
branch describing code that does not exist on it, and a stacked branch has to be correct on
its own, not only once the branch above it lands.

The two rules that govern the new structure, `tool-authoring` and `secrets`, are therefore
created in task 3 alongside the structure they govern, not in task 2.

### Task 1 - chore/mcp-tools-refactor-plan

Created this record with the confirmed plan before any other file was written, plus the
branch-naming decision record.

Captured the three decisions the user was asked for (version, instruction selection, and
- after a first attempt was rejected - branching), so a later session does not re-litigate
them.

Next task depends on: nothing.
