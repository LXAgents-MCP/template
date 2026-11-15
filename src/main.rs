//! `template-server`: the MCP server entry point. It chooses the transport and does nothing
//! else.
//!
//! Nothing here may write to stdout: on stdio, stdout is the JSON-RPC channel.

use std::process::ExitCode;

use template::config::settings::Settings;

fn main() -> ExitCode {
    template::routes::run(Settings::from_env())
}
