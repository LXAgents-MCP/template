//! Every environment variable the server reads, in one place.
//!
//! They are read once, into [`Settings`], when a binary starts. The CLI's `serve` flags then
//! override fields on that struct rather than writing to the environment. `API_KEY` is the one
//! exception: it is a secret, and the tools that need it read it themselves at call time - see
//! `.agents/rules/secrets.md`.

use std::thread::available_parallelism;

/// The transport this process speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    /// JSON-RPC over stdin and stdout. The default.
    Stdio,
    /// Stateless streamable HTTP on `/mcp`.
    Http,
}

/// What the server was configured with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// `MCP_TRANSPORT`: `stdio` (default) or `http` (`streamable-http` is accepted too).
    pub transport: Transport,
    /// `PORT`: the HTTP port, `3000` when unset or unparseable.
    pub port: u16,
    /// `HOST`: the interface the HTTP transport binds, `0.0.0.0` when unset or empty.
    pub host: String,
    /// `MCP_ALLOWED_HOSTS`: the `Host` header allow-list. Empty means none is applied.
    pub allowed_hosts: Vec<String>,
    /// `MCP_CLUSTER_WORKERS`: how many threads serve HTTP, one per CPU when unset.
    pub workers: usize,
}

impl Settings {
    /// Read every variable from the process environment.
    pub fn from_env() -> Self {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    /// Read every variable through `lookup`, so tests never have to touch the real environment.
    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            transport: transport(lookup("MCP_TRANSPORT")),
            port: port(lookup("PORT")),
            host: host(lookup("HOST")),
            allowed_hosts: allowed_hosts(lookup("MCP_ALLOWED_HOSTS")),
            workers: workers(lookup("MCP_CLUSTER_WORKERS")),
        }
    }
}

/// `MCP_TRANSPORT`, lowercased. Anything that is not `http` or `streamable-http` is stdio.
pub fn transport(raw: Option<String>) -> Transport {
    match raw.map(|value| value.to_lowercase()).as_deref() {
        Some("http") | Some("streamable-http") => Transport::Http,
        _ => Transport::Stdio,
    }
}

/// `PORT`, `3000` when unset or not a port number.
pub fn port(raw: Option<String>) -> u16 {
    raw.and_then(|value| value.trim().parse().ok())
        .unwrap_or(3000)
}

/// The bind address, named rather than implied.
///
/// Binding every interface is what a published port needs and what a container gets - but it is
/// a decision, so it is made here, visibly, and the startup line reports it. `HOST=127.0.0.1` is
/// the way to take it back.
pub fn host(raw: Option<String>) -> String {
    raw.filter(|value| !value.is_empty())
        .unwrap_or_else(|| "0.0.0.0".to_owned())
}

/// The `Host` header allow-list, when one is configured.
///
/// Unset - or set to nothing but commas and spaces - means the check is skipped rather than
/// guessed at. An allow-list that silently refuses every request is a worse failure than an
/// absent one, so the absence is reported on startup instead of being replaced by a default.
pub fn allowed_hosts(raw: Option<String>) -> Vec<String> {
    raw.map(|value| {
        value
            .split(',')
            .map(str::trim)
            .filter(|host| !host.is_empty())
            .map(str::to_owned)
            .collect()
    })
    .unwrap_or_default()
}

/// How many threads serve HTTP.
///
/// Only a positive integer is honoured. Zero would be a server that answers nothing, which reads
/// like a configuration and behaves like an outage, so `0`, a negative number and anything
/// unparseable fall through to the default: the number of CPUs this process was given.
pub fn workers(raw: Option<String>) -> usize {
    raw.and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|count| *count >= 1)
        .unwrap_or_else(|| available_parallelism().map_or(1, usize::from))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn some(value: &str) -> Option<String> {
        Some(value.to_owned())
    }

    #[test]
    fn transport_defaults_to_stdio() {
        assert_eq!(transport(None), Transport::Stdio);
        assert_eq!(transport(some("HTTP")), Transport::Http);
        assert_eq!(transport(some("streamable-http")), Transport::Http);
        assert_eq!(transport(some("nonsense")), Transport::Stdio);
    }

    #[test]
    fn port_and_host_have_defaults() {
        assert_eq!(port(None), 3000);
        assert_eq!(port(some("8080")), 8080);
        assert_eq!(port(some("not a port")), 3000);
        assert_eq!(host(None), "0.0.0.0");
        assert_eq!(host(some("")), "0.0.0.0");
        assert_eq!(host(some("127.0.0.1")), "127.0.0.1");
    }

    #[test]
    fn separators_only_is_no_allow_list() {
        assert!(allowed_hosts(None).is_empty());
        assert!(allowed_hosts(some(" , , ")).is_empty());
        assert_eq!(
            allowed_hosts(some("a.test, b.test")),
            vec!["a.test", "b.test"]
        );
    }

    #[test]
    fn only_a_positive_worker_count_is_honoured() {
        assert_eq!(workers(some("2")), 2);
        assert_eq!(workers(some("1")), 1);
        assert!(workers(some("0")) >= 1);
        assert!(workers(some("-3")) >= 1);
        assert!(workers(None) >= 1);
    }
}
