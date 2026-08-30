# Environment Variables

Two variables, both optional. The server starts with neither of them set.

| Variable | Default | Read by | Effect |
|---|---|---|---|
| `MCP_TRANSPORT` | `stdio` | `src/index.js` | `stdio` or `http` (`streamable-http` is accepted too). |
| `PORT` | `3000` | `src/index.js` | The port the HTTP transport listens on. Ignored on stdio. |

## Usage

```bash
# stdio (default)
npm start

# streamable HTTP on 3000
npm run start:http

# streamable HTTP on another port
MCP_TRANSPORT=http PORT=8080 node src/index.js

# the same, through the CLI
template serve --http --port 8080
```

The CLI's `serve` command sets both variables from its flags, so `--http`, `--stdio`,
and `--port` are equivalent to exporting them.

## Secrets

This project reads no secrets and needs no credentials to run. `.env` is gitignored so
that a project scaffolded from this template has somewhere safe to put them from the
start.

## Related pages

* [`setup.md`](setup.md) — installing and running both modes.
* [`../information/overview.md`](../information/overview.md) — what the project is.
