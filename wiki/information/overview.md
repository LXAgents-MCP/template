# Overview

`@mcagents-mcp/template` is a **dual-purpose** package: the same implementation is
reachable as a terminal command and as an MCP server. It is also the template other
LXAgents MCP repositories are scaffolded from.

## The two surfaces

| Surface | Bin | Who uses it |
|---|---|---|
| CLI | `template` | A person running it by hand or from a script |
| MCP server | `template-server` | An MCP client, an editor, an agent, or a connector |

Both share one implementation and one tool declaration, so a result produced through one
is identical to the same result produced through the other. `npm test` pins that
agreement.

## What it ships

* An MCP server over **stdio** and **streamable HTTP**, with a `/healthz` endpoint on
  the HTTP transport.
* A CLI with `help`, `version`, `tools`, and `serve`.
* One tool, `ping`, which returns `pong` and proves the server is reachable.
* A test asserting that both surfaces expose the same tools.

## Using it as a template

This repository is a starting point, not a finished server. `PROMPT.md` at the root is
the scaffolding procedure: it collects the new project's names, rewrites the files that
carry them, and deletes itself. Everything shipped here is inherited by the project
scaffolded from it.

## Requirements

Node.js 20 or newer. One dependency (`@modelcontextprotocol/sdk`), and **no build step**
— the package ships source and Node runs it directly.

## Related pages

* [`architecture.md`](architecture.md) — how the pieces fit together.
* [`../environments/setup.md`](../environments/setup.md) — installing and running it.
* [`../environments/env.md`](../environments/env.md) — environment variables.
