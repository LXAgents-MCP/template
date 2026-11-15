//! The shape of a tool module.
//!
//! Every file under `src/controllers/` is one tool. It declares `NAME` and `DESCRIPTION`, an
//! `async fn handler` that returns a [`ToolOutcome`], and a `route()` that pairs them as an rmcp
//! [`ToolRoute`]. `src/routes/server.rs` registers whatever `TOOL_MODULES` lists, so a tool that is
//! not in that array is invisible to the MCP surface and to the CLI alike.
//!
//! Nothing here knows about a sample tool, so scaffolding can delete all four cleanly.

use std::sync::Arc;

use rmcp::handler::server::common::{schema_for_empty_input, schema_for_input};
use rmcp::handler::server::router::tool::ToolRoute;
use rmcp::model::{JsonObject, Tool};
use schemars::JsonSchema;

/// A tool module: a function that builds the tool's route for the server type `S`.
///
/// Generic over `S` because a handler never needs the server itself; that keeps
/// `src/controllers/` below `src/routes/` in the import order.
pub type ToolModule<S> = fn() -> ToolRoute<S>;

/// What a handler returns: the text of a successful answer, or the text of a failure.
///
/// `Err` reaches the caller as a tool result with `isError: true`, never as ordinary text: a
/// caller must be able to tell a failure from an answer.
pub type ToolOutcome = Result<String, String>;

/// The advertised input schema of a tool that takes no arguments: `{"type":"object","properties":{}}`.
pub fn no_parameters() -> Arc<JsonObject> {
    schema_for_empty_input()
}

/// The advertised input schema of a tool whose arguments deserialize into `T`.
///
/// Each field's `///` doc comment becomes its `description`, which is what the calling model
/// reads as the parameter's documentation.
///
/// # Panics
///
/// When `T` does not describe a JSON object. That is a defect in the tool file, found the first
/// time the server starts, and `cargo test` starts it.
pub fn parameters<T: JsonSchema + 'static>() -> Arc<JsonObject> {
    schema_for_input::<T>().unwrap_or_else(|error| {
        panic!(
            "invalid input schema for {}: {error}",
            std::any::type_name::<T>()
        )
    })
}

/// The tool declaration rmcp lists: name, description and input schema.
pub fn declaration(
    name: &'static str,
    description: &'static str,
    input_schema: Arc<JsonObject>,
) -> Tool {
    Tool::new(name, description, input_schema)
}
