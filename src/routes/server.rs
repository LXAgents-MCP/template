//! The MCP server.
//!
//! A fresh [`TemplateServer`] is created per connection - per request on HTTP - because an rmcp
//! service holds per-connection state.
//!
//! Every tool lives in its own file under `src/controllers/`. Adding one means two edits: the new
//! file, and an entry in [`TOOL_MODULES`] below. Nothing else registers tools - [`list_tools`] and
//! the CLI both read this array, so a tool registered outside it would be invisible to both.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, tool_handler};

use crate::config::version::VERSION;
use crate::controllers::{calculate_sum, get_secure_summary, get_server_time, search_secure_data};
use crate::models::tool::ToolModule;

pub const SERVER_ID: &str = "template";
pub const SERVER_TITLE: &str = "Template";

/// Every tool this server registers, in the order it lists them.
pub const TOOL_MODULES: &[ToolModule<TemplateServer>] = &[
    get_server_time::route,
    get_secure_summary::route,
    calculate_sum::route,
    search_secure_data::route,
];

/// A registered tool as the CLI lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolListing {
    pub name: String,
    pub description: String,
}

/// The MCP server: server info, instructions, and the tools in [`TOOL_MODULES`].
#[derive(Debug, Clone)]
pub struct TemplateServer {
    tool_router: ToolRouter<TemplateServer>,
}

impl Default for TemplateServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateServer {
    /// A fresh server with every tool in [`TOOL_MODULES`] registered on it.
    pub fn new() -> Self {
        Self {
            tool_router: tool_router(),
        }
    }
}

/// The router built from [`TOOL_MODULES`]: the one source of truth for the tool list.
fn tool_router() -> ToolRouter<TemplateServer> {
    TOOL_MODULES
        .iter()
        .fold(ToolRouter::new(), |router, module| {
            router.with_route(module())
        })
}

/// The registered tools, as name/description pairs, in registration order.
///
/// The CLI prints this rather than keeping a list of its own, so the two surfaces cannot drift
/// apart.
pub fn list_tools() -> Vec<ToolListing> {
    let router = tool_router();
    TOOL_MODULES
        .iter()
        .map(|module| module().name().to_owned())
        .filter_map(|name| router.get(&name).cloned())
        .map(|tool| ToolListing {
            name: tool.name.into_owned(),
            description: tool.description.map(|d| d.into_owned()).unwrap_or_default(),
        })
        .collect()
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for TemplateServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(SERVER_ID, VERSION).with_title(SERVER_TITLE))
            .with_instructions(format!("{SERVER_TITLE} - call the tools listed below."))
    }
}
