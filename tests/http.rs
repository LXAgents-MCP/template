//! The streamable HTTP transport, over a real socket.
//!
//! `tests/server.rs` exercises the server in memory. What that cannot reach is the part that only
//! exists when there is a socket: the route table, the transport's framing, the interface it
//! binds, the shape of its errors, the `Host` allow-list, the body limit, the worker threads, and
//! what a shutdown does. Each test starts the real `template-server` as a child process.

mod common;

use std::net::TcpListener;
use std::thread;

use common::{
    READY_TIMEOUT, call, connect, get, is_iso_millis, json_of, raw_request, start_http, text_of,
};
use rmcp::ServiceExt;
use serde_json::{Value, json};
use template::routes::app::BODY_LIMIT_BYTES;
use template::routes::server::{SERVER_ID, TemplateServer};

/// One sum per entry, all at once and each on its own connection; every caller must get its own
/// answer. Interleaved calls on one client would pass against a shared server too; several
/// connections with several different answers would not.
async fn assert_isolated_sums(url: &str, sums: &[(i64, i64)]) {
    let tasks: Vec<_> = sums
        .iter()
        .map(|&(a, b)| {
            let url = url.to_owned();
            tokio::spawn(async move {
                let client = connect(&url).await;
                let result = call(&client, "calculate_sum", json!({ "a": a, "b": b })).await;
                (a + b, result)
            })
        })
        .collect();
    for task in tasks {
        let (expected, result) = task.await.unwrap();
        assert_eq!(
            text_of(&result),
            expected.to_string(),
            "a request was answered with another request's arguments"
        );
    }
}

fn error_message(body: &Value) -> String {
    body["error"]["message"]
        .as_str()
        .unwrap_or_default()
        .to_owned()
}

#[test]
fn the_health_check_answers_without_a_session() {
    let server = start_http(&[]);
    let response = get(server.port, "/healthz");
    let body = json_of(&response);
    assert_eq!(response.status, 200);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["server"], SERVER_ID);
    assert_eq!(body["version"], "0.0.0");
}

#[test]
fn the_startup_line_names_the_interface_it_bound() {
    let server = start_http(&[]);
    let expected = format!("serving over http on :{}/mcp (127.0.0.1)", server.port);
    assert!(server.output().contains(&expected), "{}", server.output());
}

#[tokio::test]
async fn a_server_process_writes_nothing_to_stdout() {
    let server = start_http(&[]);
    let client = connect(&server.url).await;
    client.list_all_tools().await.unwrap();
    text_of(&call(&client, "calculate_sum", json!({ "a": 2, "b": 40 })).await);
    client.cancel().await.unwrap();

    assert_eq!(server.stdout(), "", "stdout must stay empty");
    assert!(
        !server.stderr().is_empty(),
        "the startup line went to stderr"
    );
}

#[tokio::test]
async fn the_http_transport_serves_the_same_tools_as_the_in_memory_one() {
    let server = start_http(&[]);
    let http = connect(&server.url).await;

    let (client_io, server_io) = tokio::io::duplex(64 * 1024);
    let memory_server = tokio::spawn(async move { TemplateServer::new().serve(server_io).await });
    let memory = ().serve(client_io).await.unwrap();
    let _memory_server = memory_server.await.unwrap().unwrap();

    let mut via_http: Vec<(String, String)> = http
        .list_all_tools()
        .await
        .unwrap()
        .into_iter()
        .map(|tool| {
            (
                tool.name.to_string(),
                tool.description.unwrap_or_default().to_string(),
            )
        })
        .collect();
    let mut in_memory: Vec<(String, String)> = memory
        .list_all_tools()
        .await
        .unwrap()
        .into_iter()
        .map(|tool| {
            (
                tool.name.to_string(),
                tool.description.unwrap_or_default().to_string(),
            )
        })
        .collect();
    via_http.sort();
    in_memory.sort();
    assert_eq!(via_http.len(), 4, "the four sample tools");
    assert_eq!(via_http, in_memory);
}

#[tokio::test]
async fn a_sample_tool_answers_over_http_the_way_it_answers_over_stdio() {
    let server = start_http(&[]);
    let client = connect(&server.url).await;
    assert_eq!(
        text_of(&call(&client, "calculate_sum", json!({ "a": 2, "b": 40 })).await),
        "42"
    );
    let time = text_of(&call(&client, "get_server_time", json!({})).await);
    assert!(is_iso_millis(&time), "{time}");
}

#[test]
fn an_unknown_route_says_what_this_server_does_not_serve() {
    let server = start_http(&[]);
    let response = get(server.port, "/nope");
    assert_eq!(response.status, 404);
    let body = json_of(&response);
    assert_eq!(body["jsonrpc"], "2.0");
    assert_eq!(body["error"]["code"], -32601);
    assert!(error_message(&body).contains("Not found: /nope"));
}

#[test]
fn get_mcp_is_refused_rather_than_served() {
    let server = start_http(&[]);
    for method in ["GET", "DELETE", "PUT"] {
        let response = raw_request(server.port, method, "/mcp", "127.0.0.1", "");
        assert_eq!(response.status, 405, "{method}");
        assert!(
            error_message(&json_of(&response))
                .contains(&format!("{method} is not supported in stateless mode"))
        );
    }
}

#[tokio::test]
async fn concurrent_requests_do_not_share_state() {
    let server = start_http(&[]);
    assert_isolated_sums(&server.url, &[(1, 1), (20, 22), (-5, 5)]).await;
}

#[test]
fn a_shutdown_drains_and_stops_accepting() {
    for signal in ["INT", "TERM"] {
        let mut server = start_http(&[]);
        server.signal(signal);
        assert_eq!(
            server.wait_exit(READY_TIMEOUT),
            Some(0),
            "the handler did not run for SIG{signal}"
        );
        assert!(
            server.output().contains(&format!("SIG{signal}, draining")),
            "{}",
            server.output()
        );
        assert!(
            std::net::TcpStream::connect(("127.0.0.1", server.port)).is_err(),
            "the port is no longer served"
        );
    }
}

#[test]
fn a_second_signal_stops_the_wait_rather_than_queueing_behind_the_first() {
    let mut server = start_http(&[]);
    server.signal("INT");
    server.signal("INT");
    assert_eq!(server.wait_exit(READY_TIMEOUT), Some(0));
    assert_eq!(
        server.count("SIGINT, draining"),
        1,
        "the drain line is written once"
    );
}

#[test]
fn after_a_kill_the_port_is_free_again() {
    let mut server = start_http(&[("MCP_CLUSTER_WORKERS", "2")]);
    server.signal("KILL");
    assert_eq!(server.wait_exit(READY_TIMEOUT), None, "SIGKILL ends it");
    TcpListener::bind(("127.0.0.1", server.port)).expect("nothing is left holding the port");
}

/* The Host guard. */

#[test]
fn with_no_allow_list_set_nothing_is_refused() {
    let server = start_http(&[]);
    for host in ["example.test", "evil.test", "127.0.0.1"] {
        let response = raw_request(server.port, "GET", "/healthz", host, "");
        assert_eq!(
            response.status, 200,
            "{host} must be served when no allow-list is set"
        );
    }
}

#[test]
fn the_startup_line_announces_that_no_allow_list_is_applied() {
    let server = start_http(&[]);
    assert!(
        server
            .output()
            .contains("MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied")
    );
}

#[test]
fn the_allow_list_is_applied_when_mcp_allowed_hosts_is_set() {
    let server = start_http(&[(
        "MCP_ALLOWED_HOSTS",
        "template.example.test, other.example.test",
    )]);
    let port = server.port;
    assert_eq!(
        raw_request(port, "GET", "/healthz", "template.example.test", "").status,
        200
    );
    assert_eq!(
        raw_request(port, "GET", "/healthz", "other.example.test", "").status,
        200
    );

    let refused = raw_request(port, "GET", "/healthz", "evil.test", "");
    assert_eq!(refused.status, 403);
    let body = json_of(&refused);
    assert_eq!(body["jsonrpc"], "2.0");
    assert!(error_message(&body).contains("Invalid Host: evil.test"));

    // The port is not part of the match.
    let with_port = format!("template.example.test:{port}");
    assert_eq!(
        raw_request(port, "GET", "/healthz", &with_port, "").status,
        200
    );

    // And the MCP endpoint is guarded too.
    assert_eq!(
        raw_request(port, "POST", "/mcp", "evil.test", "{}").status,
        403
    );
}

#[test]
fn the_allow_list_guards_the_health_check_too() {
    let server = start_http(&[("MCP_ALLOWED_HOSTS", "template.example.test")]);
    assert_eq!(
        raw_request(server.port, "GET", "/healthz", "evil.test", "").status,
        403
    );
    assert_eq!(
        raw_request(server.port, "GET", "/healthz", "template.example.test", "").status,
        200
    );
}

#[test]
fn setting_mcp_allowed_hosts_silences_the_warning() {
    let server = start_http(&[("MCP_ALLOWED_HOSTS", "template.example.test")]);
    assert!(!server.output().contains("MCP_ALLOWED_HOSTS is unset"));
    assert!(
        !server
            .output()
            .contains("no Host header allow-list is applied")
    );
}

#[test]
fn an_allow_list_of_nothing_but_separators_still_counts_as_unset() {
    let server = start_http(&[("MCP_ALLOWED_HOSTS", " , , ")]);
    assert_eq!(
        raw_request(server.port, "GET", "/healthz", "anything.test", "").status,
        200
    );
    assert!(server.output().contains("MCP_ALLOWED_HOSTS is unset"));
}

/* The body limit. */

#[test]
fn the_body_limit_is_4_mb() {
    assert_eq!(BODY_LIMIT_BYTES, 4 * 1024 * 1024);
}

#[test]
fn a_body_over_the_limit_is_refused_in_the_json_rpc_envelope() {
    let server = start_http(&[]);
    let oversized = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "calculate_sum",
            "arguments": { "a": 1, "b": 1 },
            "padding": "x".repeat(BODY_LIMIT_BYTES),
        },
    })
    .to_string();
    assert!(
        oversized.len() > BODY_LIMIT_BYTES,
        "the payload must actually exceed the limit"
    );

    let response = raw_request(server.port, "POST", "/mcp", "127.0.0.1", &oversized);
    assert_eq!(response.status, 400);
    let body = json_of(&response);
    assert_eq!(body["jsonrpc"], "2.0");
    assert_eq!(body["error"]["code"], -32700);
}

#[test]
fn malformed_json_gets_exactly_the_same_answer_as_a_body_that_is_too_large() {
    let server = start_http(&[]);
    let response = raw_request(
        server.port,
        "POST",
        "/mcp",
        "127.0.0.1",
        "{ this is not json",
    );
    assert_eq!(response.status, 400);
    let body = json_of(&response);
    assert_eq!(body["error"]["code"], -32700);
    assert_eq!(
        body["error"]["message"],
        "Parse error: request body is not valid JSON"
    );
}

#[test]
fn no_response_advertises_the_framework() {
    let server = start_http(&[]);
    for (method, path) in [("GET", "/healthz"), ("GET", "/nope"), ("GET", "/mcp")] {
        let response = raw_request(server.port, method, path, "127.0.0.1", "");
        assert!(
            !response.headers.contains("x-powered-by"),
            "X-Powered-By leaked on {path}"
        );
        assert!(
            !response.headers.contains("\r\nserver:"),
            "Server leaked on {path}"
        );
    }
}

/* The worker threads. */

#[tokio::test]
async fn mcp_cluster_workers_1_runs_one_thread_and_serves() {
    let server = start_http(&[("MCP_CLUSTER_WORKERS", "1")]);
    let output = server.output();
    assert!(
        output.contains("MCP_CLUSTER_WORKERS is 1, so the server runs on one thread."),
        "{output}"
    );
    assert!(!output.contains("HTTP worker threads"));
    assert_eq!(server.count("serving over http"), 1);

    let client = connect(&server.url).await;
    assert_eq!(client.list_all_tools().await.unwrap().len(), 4);
}

#[test]
fn mcp_cluster_workers_2_serves_from_two_threads() {
    let server = start_http(&[("MCP_CLUSTER_WORKERS", "2")]);
    let expected = format!("starting 2 HTTP worker threads on :{}/mcp", server.port);
    assert!(server.output().contains(&expected), "{}", server.output());
    assert_eq!(
        server.count("serving over http"),
        1,
        "one process, one listener"
    );

    let port = server.port;
    let answers: Vec<_> = (0..8)
        .map(|_| thread::spawn(move || get(port, "/healthz")))
        .collect();
    for answer in answers {
        let response = answer.join().unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(json_of(&response)["server"], SERVER_ID);
    }
}

#[tokio::test]
async fn concurrent_tool_calls_stay_isolated_across_worker_threads() {
    let server = start_http(&[("MCP_CLUSTER_WORKERS", "2")]);
    assert_isolated_sums(&server.url, &[(1, 1), (500, 24), (-7, 3)]).await;
}

#[test]
fn a_port_already_in_use_is_reported_and_exits_non_zero() {
    let holder = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = holder.local_addr().unwrap().port().to_string();
    let output = std::process::Command::new(common::SERVER_BIN)
        .env("MCP_TRANSPORT", "http")
        .env("HOST", "127.0.0.1")
        .env("PORT", &port)
        .env("MCP_CLUSTER_WORKERS", "1")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("stopped:"));
    drop(holder);
}
