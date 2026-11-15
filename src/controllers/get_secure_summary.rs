//! Sample tool: read-only, takes no arguments, requires the server API key.
//!
//! The key is read inside the handler, at call time, so a process that sets it after startup
//! still works. Deleted at scaffold time - see `PROMPT.md`.

use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::service::MaybeSend;

use crate::models::tool::{ToolOutcome, declaration, no_parameters};

pub const NAME: &str = "get_secure_summary";
pub const DESCRIPTION: &str = "Return a short authenticated status summary. Takes no arguments. Requires the API_KEY environment variable.";

pub async fn handler() -> ToolOutcome {
    if std::env::var("API_KEY").map_or(true, |key| key.is_empty()) {
        return Err(
            "get_secure_summary requires an API key. Set the API_KEY environment variable before starting the server."
                .to_owned(),
        );
    }

    Ok("Authenticated. 3 records available, last updated 2026-01-01T00:00:00.000Z.".to_owned())
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(declaration(NAME, DESCRIPTION, no_parameters()), handler)
}
