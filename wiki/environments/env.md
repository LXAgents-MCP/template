# Environment Variables

Six variables, all optional. The server starts with none of them set; only the tools
that require authentication fail in that state, and the HTTP port is guarded by
configuration rather than by default.

| Variable | Default | Read by | Effect |
|---|---|---|---|
| `API_KEY` | unset | Tool handlers | The single server-wide key. Tools that require it fail without it. |
| `MCP_TRANSPORT` | `stdio` | `src/config/settings.rs` | `stdio` or `http` (`streamable-http` is accepted too). `http` is **Streamable HTTP on `/mcp`** — it is the only HTTP transport this server has. |
| `PORT` | `3000` | `src/config/settings.rs` | The port the HTTP transport listens on. Ignored on stdio. |
| `HOST` | `0.0.0.0` | `src/config/settings.rs` | The interface the HTTP transport binds. Ignored on stdio. |
| `MCP_ALLOWED_HOSTS` | *unset* | `src/config/settings.rs` | Comma-separated `Host` allow-list. **Unset means no allow-list is applied.** Read on the HTTP transport only. |
| `MCP_CLUSTER_WORKERS` | *the CPU count* | `src/config/settings.rs` | How many threads serve HTTP. **`1` means a single-threaded runtime.** Read on the HTTP transport only; stdio always runs on one thread. |

`0.0.0.0` is every IPv4 interface. It is **not** the dual-stack `::`, so a client reaching the
server over IPv6 needs `HOST=::`.

## `API_KEY`

One key for the whole server, not one per tool.

```bash
export API_KEY="your-key-here"
template-server
```

Tools that need it read it **when they are called**, so setting it after the process
starts still works. Without it, those tools fail with a message naming the tool and the
variable:

```
get_secure_summary requires an API key. Set the API_KEY environment variable
before starting the server.
```

Tools that do not need it — `get_server_time`, `calculate_sum` in the shipped samples —
work with nothing configured.

Every tool is listed by `tools/list` whether or not a key is set. Only *calling* an
authenticated tool fails, which keeps "you are not authenticated" distinguishable from
"that tool does not exist".

### Keeping it out of the repository

`.env` is gitignored. The key is never logged, never included in tool output, and never
returned by `/healthz`. Do not paste it into a test fixture, a wiki page, or a commit.

## `MCP_TRANSPORT`, `PORT` and `HOST`

```bash
# stdio (default)
template-server

# streamable HTTP on 3000
MCP_TRANSPORT=http template-server

# streamable HTTP on another port
MCP_TRANSPORT=http PORT=8080 template-server

# streamable HTTP on loopback only
MCP_TRANSPORT=http HOST=127.0.0.1 template-server

# the same, through the CLI
template serve --http --port 8080
```

The CLI's `serve` command applies its flags on top of the environment, so `--http`,
`--stdio` and `--port` are equivalent to exporting `MCP_TRANSPORT` and `PORT`. There is no `--host` flag, for the same
reason there is no `--allowed-hosts` flag: one transport, one reason to want a different
value, and an environment variable covers both surfaces at once.

## `MCP_ALLOWED_HOSTS`

A comma-separated list of `Host` values the HTTP transport will answer. The port is not
part of the match, so a client arriving through a proxy that sends `host:port` still
matches the bare name, and the match ignores case. For a bracketed IPv6 address, list the brackets: `[::1]`.

```bash
MCP_TRANSPORT=http MCP_ALLOWED_HOSTS=mcp.example.com template-server
```

**Unset, empty, or set to nothing but commas and spaces, the allow-list is not applied at
all** — every request is served, and the startup line says so:

```text
template 0.0.0 MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied.
```

That is the shipped default, and it is a deliberate one: an allow-list that silently
refuses every request is a worse failure than an absent one, and a control that is off
silently reads as present. If this server is reachable from anywhere but your own
machine, set the variable.

When it is set, the check is mounted **above every route, `/healthz` included**, and a
`Host` outside the list is answered with a 403 and a JSON-RPC error body. A green
health check on a service that every real caller is being refused from is worse than no
health check at all.

It is a filter, not a credential. It decides which `Host` values are answered, not who is
calling: it does not stop a client on the same network from connecting to an
allow-listed name, and it is not a substitute for `API_KEY` or for TLS.

## `MCP_CLUSTER_WORKERS`

How many worker threads the HTTP transport's tokio runtime has. There is one process and one
listener; tokio spreads the connections across the threads.

Unset, the count is the number of CPUs this process was actually given, not a constant, so a
two-CPU container gets two threads and a laptop does not get eight.

```bash
# one thread per CPU (the default)
MCP_TRANSPORT=http template-server

# a single-threaded runtime
MCP_CLUSTER_WORKERS=1 MCP_TRANSPORT=http template-server

# four threads on a machine that reports two CPUs
MCP_CLUSTER_WORKERS=4 MCP_TRANSPORT=http template-server
```

Only a positive integer is honoured — `0`, a negative number and anything unparseable fall
through to the default, because a worker count of zero is a server that answers nothing and
reads like a configuration while behaving like an outage.

The name is older than this implementation: the TypeScript server forked one `node:cluster`
process per worker. It is kept so that a deployment, and the sibling LXAgents MCP repositories,
keep one contract. It has no CLI flag for the same reason `HOST` has none.

## Related pages

* [`setup.md`](setup.md) — installing and running both modes.
* [`../information/overview.md`](../information/overview.md) — what the project is.
