---
name: agents-entry-point
description: Entry point for template - a dual-purpose MCP server and CLI.
---

# AGENTS.md

This repository is `template`: an MCP server and a CLI over one
implementation. It consumes the LXAgents shared agent instruction set through the
`lxagents-agents-base` connector rather than keeping its own copy of it.

## Shared Instruction Set

The conventions this repository follows — branching, commits, pull requests, task
workflow, the creators — live in the shared instruction set served by the
**`lxagents-agents-base`** MCP server. This repository carries only what is its
own. **Resolve the shared set before doing any work:**

1. If the `lxagents-agents-base` connector is available in this session, that is
   the shared set. Refer to it as `{shared}`; its files are addressed as
   `agents://{folder}/{file}.md`.
2. Read `agents://manifest.json` once. It lists every shared file with its `name`,
   path and description — one read instead of twenty, and it is what the routing
   tables below are checked against.
3. Read `agents://index/root-index.md` and route from there. Do not bulk-read the
   set.
4. If the connector is not available, say so plainly and continue with this
   repository's local instruction set only. **Do not reconstruct the missing rules
   from memory, and do not clone or copy them into this repository.**

Never commit shared content into this repository. A file that can be read from
`agents://` must not exist here as a copy — see
`{shared}/rules/duplicate-instruction-audit.md`.

**Local overrides shared.** A file in `.agents/` whose `name` matches a shared
file's `name` replaces that shared file entirely for this repository.

### If the connector will not resolve

**Registering a server does not reach a session that is already running.** Connectors
are loaded at session start, so adding one mid-session leaves a server that reports
healthy and is still absent from the tool surface. Restart the session before
concluding the server is broken.

Only if it is genuinely unavailable, run the set locally — as a **runtime, not a copy**:

```bash
mkdir -p ./mcps/LXAgents-MCP
git clone https://github.com/LXAgents-MCP/shared-instruction ./mcps/LXAgents-MCP/shared-instruction
cd ./mcps/LXAgents-MCP/shared-instruction && npm install
```

Then register it as a stdio server named `lxagents-agents-base`, with `command: node`,
`args: ["src/index.js"]`, and `cwd: ./mcps/LXAgents-MCP/shared-instruction`.

Three conditions make that a runtime rather than vendored content, and all three are
required: **`mcps/` is gitignored** and never committed; instructions are still read as
`agents://` resources, never by a path into `./mcps/`; and nothing is copied out of it
into this repository.

## Auto-activation

The shared instruction set is **always active**. It applies to every task here
whether or not the user mentions it, links to it, or asks for it. Treat these files as
standing orders, not as optional reference material.

At the start of every session, before doing any work:

1. Read `AGENTS.md` (this file).
2. Resolve the shared set, per the section above — and say so if it will not resolve.
3. Read `agents://manifest.json`, then `agents://index/root-index.md`, and route.
4. Load the four mandatory standard files, whatever the request looks like.
5. Match the request against the trigger table below and load the files it names —
   local first, shared second.

Four files load on **every** request rather than on a trigger — the task workflow,
the branching strategy, the commit conventions, and the discovery protocol — along
with the two permission gates that ride with them: ask before opening a pull request,
ask before merging. See `agents://rules/shared-instructions.md` §H.

If a rule conflicts with a habit, a default, or a template you would otherwise follow,
the rule wins. If it conflicts with an explicit instruction from the user in this
session, the user wins — and you say out loud which rule you are setting aside.

## Trigger table

Mirrors `agents://rules/auto-activation.md`, which is the authority. Rows are
reproduced unchanged and in order; local rows are appended below them, never
interleaved.

| When you are about to… | Load and obey |
|---|---|
| Take in any new request of more than one step | `agents://planning/task-workflow.md` |
| Create a branch | `agents://git/branching-strategy.md` |
| Write a commit message | `agents://git/commit-conventions.md` |
| Open or update a pull request | `agents://git/pull-request-template.md` |
| Write **any** commit, tag, PR, comment, or file that will be committed or posted | `agents://rules/no-session-links.md` |
| Wonder whether something is local or shared, or need to override a shared rule | `agents://rules/shared-instructions.md` |
| Decide where a new file goes | `agents://rules/directories.md` |
| Resolve, connect, or fail to reach the shared set | `agents://rules/mcp-connector.md` |
| Add, move, rename, or delete any file in a set or in `wiki/` | `agents://creators/index-creator.md` |
| Write a rule or instruction | `agents://creators/instruction-creator.md` |
| Write documentation, an SOP, or a domain guideline | `agents://creators/information-creator.md` |
| Change code or structure that a document describes | `agents://rules/change-propagation.md` |
| Record progress, a decision, or session state | `agents://creators/memory-creator.md` |
| Touch anything that carries a version number | `agents://rules/versioning.md` |
| Record a release | `agents://creators/changelog-creator.md` |
| Report finished work back to the user | `agents://rules/work-summary.md` |
| Scaffold a new project from this template | [`PROMPT.md`](PROMPT.md) |

`agents://rules/duplicate-instruction-audit.md` is the one rule that does **not**
auto-activate: it runs on request only.

## Project Scaffolding (Template Mode)

This repository is currently a template. If the user asks to initialize, scaffold, or setup a new project based on this template, you must immediately read `PROMPT.md` at the root of this repository and follow its instructions to gather requirements and modify the codebase.

## Local rules

* **Never copy a shared file into this repository.** If you can read it from
  `agents://`, it must not exist here as a file.
* **Nothing writes to stdout except the CLI.** On stdio, stdout is the JSON-RPC
  channel; logging goes to stderr.
* **Both surfaces stay in step.** A tool added to `src/server.js` is reachable from
  the CLI too, and a test pins that they agree.

## Documentation

* [`wiki/environments/setup.md`](wiki/environments/setup.md) - installing and running
  both CLI mode and server mode.
