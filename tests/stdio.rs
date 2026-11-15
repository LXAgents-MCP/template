//! The real binary over a real pipe, driven by rmcp's own stdio client.
//!
//! The API-key cases live here, because each child gets its own environment: no test sets or
//! clears a variable in the test process itself.

mod common;

use std::process::Stdio;

use common::{Client, SERVER_BIN, call, first_text, is_iso_millis, text_of};
use rmcp::ServiceExt;
use rmcp::transport::TokioChildProcess;
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::process::{ChildStderr, Command};

const TEST_KEY: &str = "test-key-not-a-real-secret";

/// Start `template-server` over stdio, with `API_KEY` set to `key` or removed.
async fn stdio_client(key: Option<&str>) -> (Client, ChildStderr) {
    let mut command = Command::new(SERVER_BIN);
    command.env("MCP_TRANSPORT", "stdio").env_remove("API_KEY");
    if let Some(key) = key {
        command.env("API_KEY", key);
    }
    let (transport, stderr) = TokioChildProcess::builder(command)
        .stderr(Stdio::piped())
        .spawn()
        .expect("the server binary starts");
    let client = ().serve(transport).await.expect("the client connects");
    (client, stderr.expect("stderr is piped"))
}

#[tokio::test]
async fn stdio_serves_the_four_tools_and_forks_nothing() {
    let (client, mut stderr) = stdio_client(None).await;
    assert_eq!(
        client.list_all_tools().await.unwrap().len(),
        4,
        "the four sample tools"
    );
    assert!(is_iso_millis(&text_of(
        &call(&client, "get_server_time", json!({})).await
    )));
    client.cancel().await.unwrap();

    let mut log = String::new();
    stderr.read_to_string(&mut log).await.unwrap();
    assert!(
        log.contains("serving over stdio"),
        "startup line on stderr: {log}"
    );
    assert!(
        !log.contains("worker threads"),
        "stdio runs on one thread: {log}"
    );
}

#[tokio::test]
async fn the_authenticated_tools_fail_descriptively_with_no_api_key() {
    let (client, _stderr) = stdio_client(None).await;
    for (name, args) in [
        ("get_secure_summary", json!({})),
        ("search_secure_data", json!({ "query": "anything" })),
    ] {
        let result = call(&client, name, args).await;
        assert_eq!(
            result.is_error,
            Some(true),
            "{name} must fail without a key"
        );
        let text = first_text(&result);
        assert!(text.contains(name), "{text}");
        assert!(text.contains("API_KEY"), "{text}");
    }
}

#[tokio::test]
async fn an_empty_api_key_counts_as_none() {
    let (client, _stderr) = stdio_client(Some("")).await;
    let result = call(&client, "get_secure_summary", json!({})).await;
    assert_eq!(result.is_error, Some(true));
}

#[tokio::test]
async fn the_authenticated_tools_succeed_once_the_api_key_is_set() {
    let (client, _stderr) = stdio_client(Some(TEST_KEY)).await;
    let summary = text_of(&call(&client, "get_secure_summary", json!({})).await);
    assert!(summary.contains("Authenticated"), "{summary}");
    let search = text_of(&call(&client, "search_secure_data", json!({ "query": "widgets" })).await);
    assert!(search.contains("widgets"), "{search}");
}

#[tokio::test]
async fn the_api_key_is_never_echoed_back_to_the_caller() {
    let (client, _stderr) = stdio_client(Some(TEST_KEY)).await;
    for (name, args) in [
        ("get_secure_summary", json!({})),
        ("search_secure_data", json!({ "query": "widgets" })),
    ] {
        let result = call(&client, name, args).await;
        let wire = serde_json::to_string(&result).unwrap();
        assert!(!wire.contains(TEST_KEY), "{name} echoed the key");
    }
}

#[test]
fn the_cli_prints_the_version_and_the_tools() {
    let cli = env!("CARGO_BIN_EXE_template");
    let version = std::process::Command::new(cli)
        .arg("--version")
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&version.stdout), "0.0.0\n");

    let tools = std::process::Command::new(cli)
        .arg("tools")
        .output()
        .unwrap();
    let listed: Vec<String> = String::from_utf8_lossy(&tools.stdout)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap_or_default()
                .to_owned()
        })
        .collect();
    let expected: Vec<String> = template::routes::server::list_tools()
        .into_iter()
        .map(|tool| tool.name)
        .collect();
    assert_eq!(listed, expected);

    let unknown = std::process::Command::new(cli)
        .arg("nope")
        .output()
        .unwrap();
    assert_eq!(unknown.status.code(), Some(2));
    assert!(unknown.stdout.is_empty());
}
