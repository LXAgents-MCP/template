//! The HTTP transport, as an application.
//!
//! This module builds an axum [`Router`] and returns it. **It does not listen** -
//! `src/routes/http.rs` owns the port and the runtime. A file that both builds the app and binds a
//! port cannot be tested without binding one.
//!
//! The surface is deliberately narrow: `POST /mcp` and `GET /healthz`, a JSON-RPC 404 for
//! everything else, and a JSON-RPC 405 for any other method on `/mcp`. The MCP endpoint is rmcp's
//! [`StreamableHttpService`], stateless, with a fresh [`TemplateServer`] per request - the same
//! type `src/routes/stdio.rs` serves, so there is no second source of truth about the tool list.

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::{Request, State};
use axum::http::{Method, StatusCode, Uri, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post_service};
use rmcp::transport::streamable_http_server::session::never::NeverSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use serde_json::{Value, json};
use tokio_util::sync::CancellationToken;

use crate::config::version::VERSION;
use crate::routes::server::{SERVER_ID, TemplateServer};

/// The request body ceiling, in bytes: 4 MiB.
///
/// Pinned as a number by `tests/http.rs`, so a limit that quietly became 64 MB cannot pass every
/// boundary test in the suite.
pub const BODY_LIMIT_BYTES: usize = 4 * 1024 * 1024;

/// The answer to an oversized body and to a malformed one. Deliberately the same for both: the
/// transport has always answered them alike, and a client was never taught to tell them apart.
pub const PARSE_ERROR_MESSAGE: &str = "Parse error: request body is not valid JSON";

/// A JSON-RPC error response: the same envelope a client parses for a successful answer, so a
/// client never has to branch on content type to find out it was refused.
pub fn rpc_error(status: StatusCode, code: i64, message: impl Into<String>) -> Response {
    let body = json!({
        "jsonrpc": "2.0",
        "error": { "code": code, "message": message.into() },
        "id": null,
    });
    (status, Json(body)).into_response()
}

/// Build the HTTP application.
///
/// `allowed_hosts` is the `Host` allow-list; empty means none is applied. `shutdown` is cancelled
/// when the server drains, which ends any response rmcp is still streaming.
pub fn create_app(allowed_hosts: Vec<String>, shutdown: CancellationToken) -> Router {
    // rmcp's own Host check is turned off: its default list is loopback-only, which would refuse
    // every request to a server bound on 0.0.0.0. The guard below is the one this server applies,
    // over every route, `/healthz` included.
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_sse_keep_alive(None)
        .with_max_request_body_bytes(BODY_LIMIT_BYTES)
        .with_cancellation_token(shutdown)
        .disable_allowed_hosts();

    let mcp = StreamableHttpService::new(
        || Ok(TemplateServer::new()),
        Arc::new(NeverSessionManager::default()),
        config,
    );

    let app = Router::new()
        .route("/healthz", get(health))
        .route(
            "/mcp",
            post_service(mcp)
                .layer(middleware::from_fn(body_guard))
                .fallback(method_not_allowed),
        )
        .fallback(not_found);

    // Not mounted at all when there is no list, rather than mounted with an empty one: an empty
    // list would refuse every request, which is not the same answer as "no allow-list is
    // configured".
    if allowed_hosts.is_empty() {
        app
    } else {
        let hosts: Arc<Vec<String>> = Arc::new(
            allowed_hosts
                .into_iter()
                .map(|host| host.to_ascii_lowercase())
                .collect(),
        );
        app.layer(middleware::from_fn_with_state(hosts, host_guard))
    }
}

/// The health check. Answers without a session, a request, or a tool.
async fn health() -> Response {
    Json(json!({ "status": "ok", "server": SERVER_ID, "version": VERSION })).into_response()
}

/// Any other method on `/mcp`. A refusal and not a 404: the path exists.
async fn method_not_allowed(method: Method) -> Response {
    rpc_error(
        StatusCode::METHOD_NOT_ALLOWED,
        -32000,
        format!("{method} is not supported in stateless mode"),
    )
}

/// Anything else is not a route this server has.
async fn not_found(uri: Uri) -> Response {
    rpc_error(
        StatusCode::NOT_FOUND,
        -32601,
        format!("Not found: {}", uri.path()),
    )
}

/// Read the whole body under [`BODY_LIMIT_BYTES`] and check it is JSON before rmcp sees it.
///
/// An oversized body and a malformed one get the same 400 / `-32700`. rmcp alone would answer the
/// first with a 413.
async fn body_guard(request: Request, next: Next) -> Response {
    if request.method() != Method::POST {
        return next.run(request).await;
    }

    let (parts, body) = request.into_parts();
    let Ok(bytes) = to_bytes(body, BODY_LIMIT_BYTES).await else {
        return rpc_error(StatusCode::BAD_REQUEST, -32700, PARSE_ERROR_MESSAGE);
    };
    if serde_json::from_slice::<Value>(&bytes).is_err() {
        return rpc_error(StatusCode::BAD_REQUEST, -32700, PARSE_ERROR_MESSAGE);
    }

    next.run(Request::from_parts(parts, Body::from(bytes)))
        .await
}

/// Refuse a request whose `Host` is not on the list.
///
/// The port is not part of the match: a client that reaches the server through a proxy sends
/// `host:port`, and a list that matched the whole header would refuse it.
async fn host_guard(
    State(allowed): State<Arc<Vec<String>>>,
    request: Request,
    next: Next,
) -> Response {
    let Some(raw) = request
        .headers()
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
    else {
        return rpc_error(StatusCode::FORBIDDEN, -32000, "Missing Host header");
    };

    let hostname = hostname_of(raw);
    if allowed.contains(&hostname) {
        next.run(request).await
    } else {
        rpc_error(
            StatusCode::FORBIDDEN,
            -32000,
            format!("Invalid Host: {hostname}"),
        )
    }
}

/// The hostname part of a `Host` header, lowercased: `Example.test:8080` is `example.test`, and
/// `[::1]:3000` is `[::1]`.
pub fn hostname_of(raw: &str) -> String {
    let raw = raw.trim();
    let hostname = if raw.starts_with('[') {
        raw.find(']').map_or(raw, |end| &raw[..=end])
    } else {
        raw.split(':').next().unwrap_or(raw)
    };
    hostname.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::hostname_of;

    #[test]
    fn the_port_is_not_part_of_the_hostname() {
        assert_eq!(hostname_of("example.test"), "example.test");
        assert_eq!(hostname_of("Example.TEST:8080"), "example.test");
        assert_eq!(hostname_of("[::1]:3000"), "[::1]");
        assert_eq!(hostname_of("127.0.0.1:3000"), "127.0.0.1");
    }
}
