# template

MCP server and CLI for template.

- **Organization:** `LXAgents-MCP`
- **Repository:** `template`
- **Server ID:** `template`
- **Package:** `@mcagents-mcp/template`
- **Dual-purpose:** a CLI (`template`) and an MCP server (`template-server`).

One implementation behind two surfaces, so a result produced through the CLI is
identical to the same result produced through an MCP client. Node.js 20+, ESM, no build
step.

## Features

- MCP server over **stdio** and **streamable HTTP** (express), with a `/healthz` endpoint.
- The HTTP transport forks one worker per CPU onto the one port
  (`MCP_CLUSTER_WORKERS`); `1` runs it in a single process.
- A configurable `Host` allow-list for the HTTP transport (`MCP_ALLOWED_HOSTS`) —
  **off unless you set it**, and announced as off on startup, so a deployment that
  believes it is guarded can see that it is not.
- A CLI with `help`, `version`, `tools`, and `serve`.
- One file per tool under `src/tools/`, with optional [zod](https://zod.dev) parameters
  and an optional server-wide API key.
- A test suite that pins the CLI and the MCP server to the same tool list, in memory and
  over a real socket.

## Quick start

```bash
npm install
npm test
npm run cli -- tools
npm start
```

Tools that require authentication read one key for the whole server:

```bash
export API_KEY="your-key-here"
```

Leave it unset and everything still starts — only calling an authenticated tool fails.

The HTTP transport binds `0.0.0.0:3000` by default and answers any `Host` unless you
configure an allow-list:

```bash
MCP_TRANSPORT=http MCP_ALLOWED_HOSTS=mcp.example.com HOST=127.0.0.1 npm run start:http
```

## Sample tools

Four tools ship with this template to demonstrate the four combinations of *takes
parameters* and *requires an API key*. **They are deleted when a real project is
scaffolded from it.**

| Tool | Parameters | API key |
|---|---|---|
| `get_server_time` | none | no |
| `get_secure_summary` | none | yes |
| `calculate_sum` | `a`, `b` | no |
| `search_secure_data` | `query` | yes |

## Documentation

- [`wiki/information/overview.md`](wiki/information/overview.md) — what this project is.
- [`wiki/information/architecture.md`](wiki/information/architecture.md) — how the pieces
  fit together.
- [`wiki/environments/setup.md`](wiki/environments/setup.md) — installing and running
  both modes.
- [`wiki/environments/env.md`](wiki/environments/env.md) — environment variables.

Full map: [`.agents/index/project-wiki-index.md`](.agents/index/project-wiki-index.md).

## Scaffolding a new project

This repository is a template. To turn it into a real project, follow
[`PROMPT.md`](PROMPT.md).

## Working with agents

Start at [`AGENTS.md`](AGENTS.md). Shared conventions — branching, commits, pull
requests, the task workflow — are served by the `lxagents-agents-base` MCP connector and
are not stored in this repository.

## License

MIT — see [`LICENSE`](LICENSE).
