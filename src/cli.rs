//! `template`: the CLI. `help`, `--version`, `tools`, and `serve`, which runs the same server as
//! `template-server`.
//!
//! Only CLI commands write to stdout; `serve` prints nothing of its own.

use std::process::ExitCode;

use template::config::settings::{Settings, Transport};
use template::config::version::VERSION;
use template::routes::server::list_tools;

const HELP: &str = "template - MCP server and CLI for template

Usage
  template <command>

Commands
  serve            Run as an MCP server (--stdio, --http, --port N)
  tools            List the tools this server exposes

Options
  -h, --help       Show this help
  -v, --version    Show the version

Environment
  API_KEY          Unified key for the tools that require authentication
  MCP_TRANSPORT    stdio (default) or http
  PORT             HTTP port, default 3000
  HOST             HTTP interface, default 0.0.0.0
  MCP_ALLOWED_HOSTS  comma-separated Host allow-list; unset means none is applied
  MCP_CLUSTER_WORKERS  HTTP worker threads, default one per CPU; 1 runs one thread";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str);
    let rest = args.get(1..).unwrap_or_default();

    match command {
        Some("-v" | "--version") => {
            println!("{VERSION}");
            ExitCode::SUCCESS
        }
        None => {
            println!("{HELP}");
            ExitCode::from(2)
        }
        Some("-h" | "--help" | "help") => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Some("serve") => template::routes::run(serve_settings(rest)),
        Some("tools") => {
            let tools = list_tools();
            let width = tools.iter().map(|tool| tool.name.len()).max().unwrap_or(0);
            for tool in tools {
                println!("{:<width$}  {}", tool.name, tool.description);
            }
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("Unknown command \"{other}\". Try --help.");
            ExitCode::from(2)
        }
    }
}

/// The environment's settings, with `serve`'s flags applied on top.
fn serve_settings(flags: &[String]) -> Settings {
    let mut settings = Settings::from_env();
    if flags.iter().any(|flag| flag == "--http") {
        settings.transport = Transport::Http;
    }
    if flags.iter().any(|flag| flag == "--stdio") {
        settings.transport = Transport::Stdio;
    }
    if let Some(port) = flags
        .iter()
        .position(|flag| flag == "--port")
        .and_then(|at| flags.get(at + 1))
        .and_then(|value| value.parse().ok())
    {
        settings.port = port;
    }
    settings
}
