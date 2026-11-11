---
name: repository-rules
description: Rules specific to template - the dual-surface contract, the stdout ban, where things go, and what must not be introduced.
---

# Repository Rules

`template` is a dual-purpose MCP server and CLI over one implementation, and it is
also the template other LXAgents MCP repositories are scaffolded from. Both facts
constrain what may be changed here.

## Mode and shared set

This repository is a **Mode B consumer**. The shared instruction set is resolved
through the `lxagents-agents-base` MCP connector and is never copied into this tree.
See the bootstrap block in [`../../AGENTS.md`](../../AGENTS.md).

## The two surfaces stay in step

The CLI (`template`) and the MCP server (`template-server`) are two doors onto one
implementation. A tool reachable from one is reachable from the other, with the same
name and the same description.

* Tools are declared in exactly one place: `src/routes/server.ts`.
* `src/cli.ts` never maintains its own list - it reads the declaration from
  `src/routes/server.ts`.
* `test/server.test.ts` pins the agreement. A change that makes the two surfaces
  disagree fails the suite, and that failure is the point.

## Nothing writes to stdout except the CLI

On the stdio transport, stdout **is** the JSON-RPC channel. A stray `console.log` on
the server path corrupts the protocol stream and the client reports a parse error
that names nothing useful.

* Server-side logging goes to stderr.
* `serve` prints nothing of its own.
* Only CLI commands write to stdout.

## Where things go

| Thing | Path |
|---|---|
| Tool declarations | `src/routes/server.ts` |
| CLI commands | `src/cli.ts` |
| Entry point (chooses the transport) | `src/index.ts` |
| Transports: express app, MCP server, HTTP worker, cluster primary | `src/routes/` |
| Tool modules | `src/controllers/` |
| Types shared across folders (the tool contract) | `src/models/` |
| Environment variables and the version | `src/config/` |
| Tests | `test/{subject}.test.ts` |

## Commands

```bash
npm install       # Node 20+; builds dist/ through the prepare script
npm run build     # tsc: src/ -> dist/
npm test          # build + typecheck, then node --import tsx --test test/*.test.ts
npm run cli -- tools
npm start         # stdio
npm run start:http
npm run inspect   # MCP Inspector against the stdio server
```

## What must not be introduced

* JavaScript source. `src/` and `test/` are TypeScript under `strict`; `dist/` is build
  output, git-ignored, and never edited by hand.
* A bundler or a second compiler. `tsc` is the build; one source file is one output file.
* A second source of truth for the tool list.
* Shared instruction content. If it can be read from `agents://`, it must not exist
  here as a file.

## Template mode

While `PROMPT.md` still exists at the root, this repository is a template and the
extra rules in [`template-mode.md`](template-mode.md) apply.
