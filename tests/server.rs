//! The server in memory: the same code as every transport, with no listener.
//!
//! A client and a fresh `TemplateServer` are joined by an in-process pipe. The API-key cases are
//! in `tests/stdio.rs`, against the real binary, so that no test here touches the environment.

mod common;

use common::{Client, call, first_text, is_iso_millis, text_of};
use rmcp::ServiceExt;
use rmcp::model::Tool;
use rmcp::service::{RoleServer, RunningService};
use serde_json::json;
use template::routes::server::{SERVER_ID, TemplateServer, list_tools};

/// A client connected to a fresh server over an in-memory pipe.
async fn in_memory() -> (Client, RunningService<RoleServer, TemplateServer>) {
    let (client_io, server_io) = tokio::io::duplex(64 * 1024);
    let server = tokio::spawn(async move { TemplateServer::new().serve(server_io).await });
    let client = ().serve(client_io).await.expect("the client connects");
    let server = server.await.unwrap().expect("the server starts");
    (client, server)
}

async fn tools(client: &Client) -> Vec<Tool> {
    client.list_all_tools().await.expect("tools/list")
}

fn sorted(mut names: Vec<String>) -> Vec<String> {
    names.sort();
    names
}

/// The advertised property names of `tool`, sorted.
fn properties(tool: &Tool) -> Vec<String> {
    let schema = tool.schema_as_json_value();
    let mut names: Vec<String> = schema["properties"]
        .as_object()
        .map(|props| props.keys().cloned().collect())
        .unwrap_or_default();
    names.sort();
    names
}

fn by_name<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("{name} is registered"))
}

#[tokio::test]
async fn the_cli_list_and_the_mcp_tool_list_agree() {
    let (client, _server) = in_memory().await;
    assert_eq!(
        client.peer_info().map(|info| info
            .server_info
            .as_ref()
            .map(|server| server.name.clone())
            .unwrap_or_default()),
        Some(SERVER_ID.to_owned())
    );

    let listed = tools(&client).await;
    assert_eq!(
        sorted(listed.iter().map(|tool| tool.name.to_string()).collect()),
        sorted(list_tools().into_iter().map(|tool| tool.name).collect())
    );

    for tool in &listed {
        let description = tool.description.as_deref().unwrap_or_default();
        assert!(!description.is_empty(), "{} needs a description", tool.name);
        let cli = list_tools()
            .into_iter()
            .find(|listing| listing.name == tool.name)
            .expect("the CLI lists it");
        assert_eq!(description, cli.description);
    }
}

#[tokio::test]
async fn the_four_sample_tools_are_registered() {
    let (client, _server) = in_memory().await;
    assert_eq!(
        sorted(
            tools(&client)
                .await
                .iter()
                .map(|tool| tool.name.to_string())
                .collect()
        ),
        [
            "calculate_sum",
            "get_secure_summary",
            "get_server_time",
            "search_secure_data"
        ]
    );
}

#[tokio::test]
async fn tools_with_parameters_advertise_them() {
    let (client, _server) = in_memory().await;
    let listed = tools(&client).await;

    let sum = by_name(&listed, "calculate_sum");
    assert_eq!(properties(sum), ["a", "b"]);
    let schema = sum.schema_as_json_value();
    assert_eq!(schema["properties"]["a"]["type"], "number");
    assert_eq!(
        schema["properties"]["a"]["description"],
        "The first addend."
    );
    let mut required: Vec<&str> = schema["required"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    required.sort_unstable();
    assert_eq!(required, ["a", "b"]);

    let search = by_name(&listed, "search_secure_data");
    assert_eq!(properties(search), ["query"]);
    let schema = search.schema_as_json_value();
    assert_eq!(schema["properties"]["query"]["type"], "string");
    assert_eq!(schema["properties"]["query"]["minLength"], 1);
}

#[tokio::test]
async fn tools_without_parameters_advertise_none() {
    let (client, _server) = in_memory().await;
    let listed = tools(&client).await;
    for name in ["get_server_time", "get_secure_summary"] {
        assert!(
            properties(by_name(&listed, name)).is_empty(),
            "{name} takes no arguments"
        );
    }
}

#[tokio::test]
async fn get_server_time_returns_a_timestamp() {
    let (client, _server) = in_memory().await;
    let text = text_of(&call(&client, "get_server_time", json!({})).await);
    assert!(is_iso_millis(&text), "not an ISO 8601 timestamp: {text}");
}

#[tokio::test]
async fn calculate_sum_adds_its_arguments() {
    let (client, _server) = in_memory().await;
    assert_eq!(
        text_of(&call(&client, "calculate_sum", json!({ "a": 2, "b": 40 })).await),
        "42"
    );
    assert_eq!(
        text_of(&call(&client, "calculate_sum", json!({ "a": 1.5, "b": 1 })).await),
        "2.5"
    );
}

#[tokio::test]
async fn calculate_sum_rejects_arguments_of_the_wrong_type() {
    let (client, _server) = in_memory().await;
    let result = call(&client, "calculate_sum", json!({ "a": "two", "b": 40 })).await;
    assert_eq!(result.is_error, Some(true));
}

#[tokio::test]
async fn search_secure_data_rejects_an_empty_query() {
    let (client, _server) = in_memory().await;
    let result = call(&client, "search_secure_data", json!({ "query": "" })).await;
    assert_eq!(result.is_error, Some(true));
    assert!(first_text(&result).contains("non-empty query"));
}
