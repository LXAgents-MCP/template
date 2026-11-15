//! Transports and the server they serve.
//!
//! `server` builds the MCP server and owns the tool list; `app` builds the axum application and
//! never binds; `http` binds it and drains it; `stdio` serves the process's own pipe.

pub mod app;
pub mod http;
pub mod server;
pub mod stdio;

use std::process::ExitCode;

use crate::config::settings::{Settings, Transport};
use crate::config::version::VERSION;
use crate::routes::server::SERVER_ID;

/// Run the server with `settings` until it is told to stop, on a runtime sized for the transport.
///
/// stdio always runs on one thread: there is one client and one pipe. HTTP runs on
/// `MCP_CLUSTER_WORKERS` threads, and `1` means a single-threaded runtime.
pub fn run(settings: Settings) -> ExitCode {
    let threads = match settings.transport {
        Transport::Stdio => 1,
        Transport::Http => {
            eprintln!("{}", http::workers_line(&settings));
            settings.workers
        }
    };

    let runtime = if threads <= 1 {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(threads)
            .enable_all()
            .build()
    };
    let runtime = match runtime {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("{SERVER_ID} {VERSION} could not start its runtime: {error}");
            return ExitCode::FAILURE;
        }
    };

    let outcome = runtime.block_on(async {
        match settings.transport {
            Transport::Stdio => stdio::serve().await.map_err(|error| error.to_string()),
            Transport::Http => http::serve(&settings)
                .await
                .map_err(|error| error.to_string()),
        }
    });

    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{SERVER_ID} {VERSION} stopped: {error}");
            ExitCode::FAILURE
        }
    }
}
