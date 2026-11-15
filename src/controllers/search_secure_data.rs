//! Sample tool: takes typed parameters and requires the server API key.
//!
//! Deleted at scaffold time - see `PROMPT.md`.

use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::MaybeSend;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::models::tool::{ToolOutcome, declaration, parameters};

pub const NAME: &str = "search_secure_data";
pub const DESCRIPTION: &str = "Search the protected dataset and return matching records. Requires the API_KEY environment variable.";

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchSecureDataArgs {
    /// The search term to match records against.
    #[schemars(length(min = 1))]
    pub query: String,
}

pub async fn handler(
    Parameters(SearchSecureDataArgs { query }): Parameters<SearchSecureDataArgs>,
) -> ToolOutcome {
    // The schema advertises `minLength: 1`, but deserializing does not enforce it, so the
    // argument is checked here, before the key, as the schema would have been.
    if query.is_empty() {
        return Err("search_secure_data needs a non-empty query.".to_owned());
    }

    if std::env::var("API_KEY").map_or(true, |key| key.is_empty()) {
        return Err(
            "search_secure_data requires an API key. Set the API_KEY environment variable before starting the server."
                .to_owned(),
        );
    }

    Ok(format!("No records matched \"{query}\"."))
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(
        declaration(NAME, DESCRIPTION, parameters::<SearchSecureDataArgs>()),
        handler,
    )
}
