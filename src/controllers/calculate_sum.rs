//! Sample tool: takes typed parameters, needs no API key.
//!
//! The arguments are a `serde` + `schemars` struct: rmcp deserializes the call's arguments into
//! it, and a call with the wrong types is answered with an error result before the handler runs.
//!
//! Deleted at scaffold time - see `PROMPT.md`.

use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::service::MaybeSend;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::models::tool::{ToolOutcome, declaration, parameters};

pub const NAME: &str = "calculate_sum";
pub const DESCRIPTION: &str = "Add two numbers and return the sum. Requires no API key.";

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculateSumArgs {
    /// The first addend.
    pub a: f64,
    /// The second addend.
    pub b: f64,
}

/// The sum, written the way `f64` displays it: `42`, not `42.0`.
pub async fn handler(
    Parameters(CalculateSumArgs { a, b }): Parameters<CalculateSumArgs>,
) -> ToolOutcome {
    Ok((a + b).to_string())
}

pub fn route<S: MaybeSend + 'static>() -> ToolRoute<S> {
    ToolRoute::new(
        declaration(NAME, DESCRIPTION, parameters::<CalculateSumArgs>()),
        handler,
    )
}
