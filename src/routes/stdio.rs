//! The stdio transport.
//!
//! stdout **is** the JSON-RPC channel here, so nothing in this process may print to it. The
//! startup line goes to stderr.

use rmcp::ServiceExt;
use rmcp::transport::stdio;

use crate::config::version::VERSION;
use crate::routes::server::{SERVER_ID, TemplateServer};

/// Serve one client over stdin and stdout until it disconnects.
pub async fn serve() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = TemplateServer::new().serve(stdio()).await?;
    eprintln!("{SERVER_ID} {VERSION} serving over stdio");
    service.waiting().await?;
    Ok(())
}
