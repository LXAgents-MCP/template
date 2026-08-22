# template

MCP server and CLI for template

- **Organization:** `MCAgents-MCP`
- **Repository:** `template`
- **Server ID:** `template`
- **Package:** `@mcagents-mcp/template`
- **Dual-purpose:** a CLI (`template`) and an MCP server (`template-server`).

## Quick start

```bash
npm install
npm test
npm run cli -- tools
npm start
```

## Modes

### CLI

```bash
npm run cli -- --help
npm run cli -- --version
npm run cli -- tools
```

### MCP Server

Stdio:

```bash
npm start
```

HTTP:

```bash
npm run start:http
```

Health check:

```bash
curl -s http://localhost:3000/healthz
```

MCP endpoint:

```text
http://localhost:3000/mcp
```

## Documentation

See [`wiki/environments/setup.md`](wiki/environments/setup.md).

## License

MIT
