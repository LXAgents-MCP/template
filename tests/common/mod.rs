//! Helpers shared by the transport suites: start the real binary, wait for its output, and talk
//! to it over a socket.
//!
//! The suites start `template-server` as a child process rather than calling into the library,
//! because only a real process can be given its own port and environment, sent a signal, or
//! killed. No test changes its own process's environment: each child gets its own.

#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use rmcp::ServiceExt;
use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::service::{RoleClient, RunningService};
use rmcp::transport::StreamableHttpClientTransport;
use serde_json::{Map, Value};

/// The compiled server binary that `cargo test` built for this run.
pub const SERVER_BIN: &str = env!("CARGO_BIN_EXE_template-server");

/// How long to wait for a startup line. A deadline, not a sleep: the common case costs nothing.
pub const READY_TIMEOUT: Duration = Duration::from_secs(30);

/// A connected MCP client.
pub type Client = RunningService<RoleClient, ()>;

/// A running server process and everything it has written.
pub struct Server {
    pub child: Child,
    pub port: u16,
    pub url: String,
    stdout: Arc<Mutex<String>>,
    stderr: Arc<Mutex<String>>,
}

impl Server {
    /// Everything written to stdout so far.
    pub fn stdout(&self) -> String {
        self.stdout.lock().unwrap().clone()
    }

    /// Everything written to stderr so far.
    pub fn stderr(&self) -> String {
        self.stderr.lock().unwrap().clone()
    }

    /// Everything written on either stream so far.
    pub fn output(&self) -> String {
        format!("{}{}", self.stdout(), self.stderr())
    }

    /// How many times `needle` has appeared on either stream.
    pub fn count(&self, needle: &str) -> usize {
        self.output().matches(needle).count()
    }

    /// True once `needle` has appeared, false if `timeout` runs out first.
    pub fn wait_for(&self, needle: &str, timeout: Duration) -> bool {
        wait_until(timeout, || self.output().contains(needle))
    }

    /// Send `signal` (`"INT"`, `"TERM"`, `"KILL"`) to the process.
    pub fn signal(&self, signal: &str) {
        let status = Command::new("kill")
            .arg(format!("-{signal}"))
            .arg(self.child.id().to_string())
            .status()
            .expect("kill runs");
        assert!(status.success(), "kill -{signal} failed");
    }

    /// Wait for the process to exit and return its exit code, `None` if a signal ended it.
    pub fn wait_exit(&mut self, timeout: Duration) -> Option<i32> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                return status.code();
            }
            assert!(
                Instant::now() < deadline,
                "the server did not exit\n--- output ---\n{}",
                self.output()
            );
            thread::sleep(Duration::from_millis(20));
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Poll `check` every 20 ms until it holds or `timeout` runs out.
pub fn wait_until(timeout: Duration, mut check: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    loop {
        if check() {
            return true;
        }
        if Instant::now() > deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// A port nothing is listening on: bound and released at once.
pub fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .expect("a free port")
        .port()
}

/// Copy everything `source` writes into `sink`, on a thread of its own.
fn capture(mut source: impl Read + Send + 'static, sink: Arc<Mutex<String>>) {
    thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        while let Ok(read) = source.read(&mut buffer) {
            if read == 0 {
                break;
            }
            sink.lock()
                .unwrap()
                .push_str(&String::from_utf8_lossy(&buffer[..read]));
        }
    });
}

/// Start the server over HTTP on a free loopback port, with `env` on top, and wait until it is
/// listening.
///
/// Bound to loopback on purpose: the suite must never open a port on every interface of the
/// machine that runs it.
pub fn start_http(env: &[(&str, &str)]) -> Server {
    let port = free_port();
    let mut command = Command::new(SERVER_BIN);
    command
        .env_remove("API_KEY")
        .env_remove("MCP_ALLOWED_HOSTS")
        .env_remove("MCP_CLUSTER_WORKERS")
        .env("MCP_TRANSPORT", "http")
        .env("PORT", port.to_string())
        .env("HOST", "127.0.0.1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in env {
        command.env(name, value);
    }

    let mut child = command.spawn().expect("the server binary starts");
    let stdout = Arc::new(Mutex::new(String::new()));
    let stderr = Arc::new(Mutex::new(String::new()));
    capture(child.stdout.take().unwrap(), stdout.clone());
    capture(child.stderr.take().unwrap(), stderr.clone());

    let mut server = Server {
        child,
        port,
        url: format!("http://127.0.0.1:{port}"),
        stdout,
        stderr,
    };

    let ready = wait_until(READY_TIMEOUT, || {
        server.output().contains("serving over http")
            || matches!(server.child.try_wait(), Ok(Some(_)))
    });
    let exited = server.child.try_wait().expect("try_wait");
    assert!(
        ready && exited.is_none() && server.output().contains("serving over http"),
        "the server never came up ({exited:?})\n--- output ---\n{}",
        server.output()
    );
    server
}

/// Connect an MCP client to a running server over a real socket.
pub async fn connect(url: &str) -> Client {
    ().serve(StreamableHttpClientTransport::from_uri(format!(
        "{url}/mcp"
    )))
    .await
    .expect("the client connects")
}

/// `arguments` as the JSON object a tool call carries.
pub fn arguments(arguments: Value) -> Map<String, Value> {
    match arguments {
        Value::Object(map) => map,
        other => panic!("arguments must be an object, got {other}"),
    }
}

/// Call `name` with `args` and return the result.
pub async fn call(client: &Client, name: &'static str, args: Value) -> CallToolResult {
    client
        .call_tool(CallToolRequestParams::new(name).with_arguments(arguments(args)))
        .await
        .expect("the call is answered")
}

/// The text of the one text block of a result, asserting it is not an error.
pub fn text_of(result: &CallToolResult) -> String {
    assert_ne!(
        result.is_error,
        Some(true),
        "expected a successful result: {result:?}"
    );
    assert_eq!(
        result.content.len(),
        1,
        "expected exactly one content block"
    );
    first_text(result)
}

/// The text of the first content block, asserting it is text.
pub fn first_text(result: &CallToolResult) -> String {
    result
        .content
        .first()
        .and_then(|block| block.as_text())
        .map(|text| text.text.clone())
        .expect("a text content block")
}

/// True when `text` looks like `2026-01-01T00:00:00.000Z`.
pub fn is_iso_millis(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 24
        && text.ends_with('Z')
        && [4, 7].iter().all(|&i| bytes[i] == b'-')
        && bytes[10] == b'T'
        && [13, 16].iter().all(|&i| bytes[i] == b':')
        && bytes[19] == b'.'
        && text
            .char_indices()
            .filter(|(i, _)| ![4, 7, 10, 13, 16, 19, 23].contains(i))
            .all(|(_, c)| c.is_ascii_digit())
}

/// A raw HTTP/1.1 response: status and body.
pub struct RawResponse {
    pub status: u16,
    pub headers: String,
    pub body: String,
}

/// Send a raw HTTP/1.1 request with exactly the `Host` header given.
///
/// An HTTP client library fills `Host` in from the URL, and one that silently replaced ours would
/// test the allow-list with the loopback name every time - the worst way to test a security
/// control. So the request is written by hand.
pub fn raw_request(port: u16, method: &str, path: &str, host: &str, body: &str) -> RawResponse {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .unwrap();
    let request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(request.as_bytes()).unwrap();
    stream.write_all(body.as_bytes()).unwrap();

    let mut raw = Vec::new();
    let _ = stream.read_to_end(&mut raw);
    let raw = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = raw.split_once("\r\n\r\n").unwrap_or((&raw, ""));
    let status = head
        .split_whitespace()
        .nth(1)
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);
    RawResponse {
        status,
        headers: head.to_ascii_lowercase(),
        body: body.to_owned(),
    }
}

/// `GET path` with the loopback `Host`.
pub fn get(port: u16, path: &str) -> RawResponse {
    raw_request(port, "GET", path, "127.0.0.1", "")
}

/// The JSON body of a response.
pub fn json_of(response: &RawResponse) -> Value {
    serde_json::from_str(&response.body)
        .unwrap_or_else(|error| panic!("not JSON ({error}): {}", response.body))
}
