---
name: agents-entry-point
description: Entry point for template - a dual-purpose MCP server and CLI.
---

# AGENTS.md

This repository is `template`: an MCP server and a CLI over one
implementation. It consumes the LXAgents shared agent instruction set through the
`lxagents-agents-base` connector rather than keeping its own copy of it.

## Shared Instruction Setup

Before doing any work, ensure the shared instruction set is available:
1. Clone the repository locally: `git clone https://github.com/LXAgents-MCP/shared-instruction.git ./mcp/shared-instruction`
2. Add it as an MCP server in your Agent IDE configuration using the path `./mcp/shared-instruction` and the ID `lxagents-agents-base`.

## Auto-activation

The shared instruction set is **always active**. It applies to every task here
whether or not the user mentions it. Resolve it through the connector, read
`agents://index/root-index.md`, and route from there.

Three files load on **every** request rather than on a trigger - the task workflow,
the branching strategy, and the commit conventions - along with the two permission
gates that ride with them: ask before opening a pull request, ask before merging.
See `agents://rules/shared-instructions.md`  H.

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
