//! Sample tool: read-only, takes no arguments, needs no API key.
//!
//! Deleted at scaffold time - see `PROMPT.md`.

use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::service::MaybeSend;

use crate::models::tool::{ToolOutcome, declaration, no_parameters};

pub const NAME: &str = "get_server_time";
pub const DESCRIPTION: &str = "Return the server's current time as an ISO 8601 timestamp in UTC. Takes no arguments and requires no API key.";

/// The current time in UTC, with milliseconds: `2026-01-01T00:00:00.000Z`.
pub async fn handler() -> ToolOutcome {
    Ok(chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string())
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(declaration(NAME, DESCRIPTION, no_parameters()), handler)
}
