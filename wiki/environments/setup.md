# Local Setup

`template` is **dual-purpose**. The same code is reachable two ways:

| Mode | What it is | Who uses it |
|---|---|---|
| **CLI mode** | A terminal command | A person running it by hand or from a script |
| **Server mode** | An MCP server over stdio or streamable HTTP | An MCP client, an editor, an agent, or a connector |

Both modes share one implementation, so a result produced in one is identical to the
same result produced in the other.

## Requirements

Rust 1.88 or newer, through [rustup](https://rustup.rs). Nothing else is needed at runtime: the
release build is two self-contained binaries.

```bash
cargo build --release   # target/release/template-server and target/release/template
cargo test              # unit tests, then the in-memory, stdio and HTTP suites
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

The dependencies are `rmcp` (the official MCP Rust SDK), `axum`, `tokio`, `serde`,
`serde_json`, `schemars`, `chrono` and `tokio-util`, pinned by `Cargo.lock`.

## Authentication

Tools that need authentication read one server-wide key from `API_KEY`:

```bash
export API_KEY="your-key-here"
```

Leave it unset and the server still starts and still lists every tool — only calling an
authenticated tool fails, with a message naming the tool and the variable. Full list of
variables: [`env.md`](env.md).

## CLI mode

### Install

```bash
# From a checkout: both binaries onto your PATH (~/.cargo/bin)
cargo install --path .
```

Without installing anything:

```bash
cargo run --bin template -- --help
./target/release/template --help
```

### Use

```bash
template --help
template --version
template tools
```

`tools` prints every registered tool with its description:

```text
get_server_time     Return the server's current time as an ISO 8601 timestamp in UTC. …
get_secure_summary  Return a short authenticated status summary. …
calculate_sum       Add two numbers and return the sum. Requires no API key.
search_secure_data  Search the protected dataset and return matching records. …
```

The list comes from `list_tools()` in `src/routes/server.rs` — the same list the MCP server
registers — so the two surfaces cannot disagree.

### Exit codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | The server could not start or stopped on an error (a port in use, for instance) |
| `2` | The command line itself was wrong |

## Server mode

### Install

An MCP client spawns the server as a subprocess, so installing it means pointing the
client at it. Either binary works: `template-server` is the server directly, and
`template serve` reaches the same server through the CLI.

```json
{
  "mcpServers": {
    "template": {
      "command": "/path/to/template/target/release/template-server"
    }
  }
}
```

After `cargo install --path .` the binary can be named directly:

```json
{
  "mcpServers": {
    "template": {
      "command": "template-server"
    }
  }
}
```

For a remote connector, point the client at `https://<host>/mcp`, including the
`/mcp` path. If you set `MCP_ALLOWED_HOSTS`, the `<host>` in that URL has to be on the
list — the connector's `Host` header is matched against it before anything is served.

### Run

```bash
# stdio
template-server
template serve --stdio

# streamable HTTP
MCP_TRANSPORT=http template-server
template serve --http --port 3000

# streamable HTTP, bound to loopback, answering only an allow-listed Host
MCP_TRANSPORT=http MCP_ALLOWED_HOSTS=mcp.example.com HOST=127.0.0.1 template-server
```

The HTTP transport serves exactly two routes — `POST /mcp` and `GET /healthz` — and binds
`0.0.0.0` unless `HOST` says otherwise. It runs on one worker thread per CPU;
`MCP_CLUSTER_WORKERS=1` runs it on a single thread.

### Docker

```bash
docker build -t template .
docker run --rm -i template                                   # stdio
docker run --rm -p 3000:3000 -e MCP_TRANSPORT=http template   # HTTP
```

Check it is up:

```bash
curl -s http://localhost:3000/healthz
```

```json
{"server":"template","status":"ok","version":"0.0.0"}
```

`/healthz` answers without a session and without a tool call, which is what makes it
usable as a liveness probe. It is also **covered by `MCP_ALLOWED_HOSTS`**, so a probe
that uses a `Host` outside the list gets a 403 — point the probe at an allow-listed name
if you configure the variable. The startup lines tell you which state you are in:

```text
template 0.0.0 starting 4 HTTP worker threads on :3000/mcp
template 0.0.0 serving over http on :3000/mcp (all interfaces)
template 0.0.0 MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied.
```

With `MCP_CLUSTER_WORKERS=1`, or on a one-CPU container, the first line reads
`MCP_CLUSTER_WORKERS is 1, so the server runs on one thread.` instead.

### Inspect it

```bash
npx @modelcontextprotocol/inspector ./target/release/template-server
```

This runs the MCP Inspector against the stdio server, listing every tool and letting
you call them.

### stdout belongs to the protocol

On the stdio transport, stdout **is** the JSON-RPC channel. Logging goes to stderr,
and `serve` prints nothing of its own. Only CLI commands write to stdout.

A `println!` on the server path is a bug that corrupts the protocol stream.

## Related pages

- [`env.md`](env.md) — every environment variable this project reads
- [`../information/overview.md`](../information/overview.md) — what this project is
- [`../information/architecture.md`](../information/architecture.md) — how the pieces fit
- [`README.md`](../../README.md)
