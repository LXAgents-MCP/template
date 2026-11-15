//! The HTTP listener: binds `HOST`:`PORT`, serves the application from `src/routes/app.rs`, and
//! drains on SIGINT or SIGTERM.
//!
//! How many threads serve it is decided by the runtime `src/main.rs` builds from
//! `MCP_CLUSTER_WORKERS`; tokio spreads connections across them, so no process is forked.

use std::time::Duration;

use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::config::settings::Settings;
use crate::config::version::VERSION;
use crate::routes::app::create_app;
use crate::routes::server::SERVER_ID;

/// How long a drain waits for requests still in flight before it stops waiting.
pub const DRAIN_GRACE: Duration = Duration::from_secs(5);

/// The line written before the server starts: how many threads will serve it.
pub fn workers_line(settings: &Settings) -> String {
    if settings.workers <= 1 {
        format!("{SERVER_ID} {VERSION} MCP_CLUSTER_WORKERS is 1, so the server runs on one thread.")
    } else {
        format!(
            "{SERVER_ID} {VERSION} starting {} HTTP worker threads on :{}/mcp",
            settings.workers, settings.port
        )
    }
}

/// Bind, serve until a signal arrives, then drain.
pub async fn serve(settings: &Settings) -> std::io::Result<()> {
    let listener = TcpListener::bind((settings.host.as_str(), settings.port)).await?;
    let port = listener.local_addr()?.port();
    let shutdown = CancellationToken::new();
    let app = create_app(settings.allowed_hosts.clone(), shutdown.clone());

    let where_ = if settings.host == "0.0.0.0" {
        "all interfaces".to_owned()
    } else {
        settings.host.clone()
    };
    eprintln!("{SERVER_ID} {VERSION} serving over http on :{port}/mcp ({where_})");

    // Said out loud, because the default is the unguarded one, and a control that is off
    // silently reads as present.
    if settings.allowed_hosts.is_empty() {
        eprintln!(
            "{SERVER_ID} {VERSION} MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied."
        );
    }

    axum::serve(listener, app)
        .with_graceful_shutdown(drain_on_signal(shutdown))
        .await
}

/// Resolve when the first SIGINT or SIGTERM arrives, after arranging the rest of the drain.
///
/// Once it resolves the listener stops accepting and axum waits for the requests in flight. A
/// second signal means the operator has stopped waiting, and [`DRAIN_GRACE`] caps the wait for a
/// request that never finishes; either ends the process at once, with status 0.
async fn drain_on_signal(shutdown: CancellationToken) {
    let signal = next_signal().await;
    eprintln!("{SERVER_ID} {VERSION} {signal}, draining");
    shutdown.cancel();

    tokio::spawn(async {
        tokio::select! {
            _ = next_signal() => {}
            () = tokio::time::sleep(DRAIN_GRACE) => {}
        }
        std::process::exit(0);
    });
}

/// The name of the next SIGINT or SIGTERM to arrive.
#[cfg(unix)]
async fn next_signal() -> &'static str {
    use tokio::signal::unix::{SignalKind, signal};

    let mut interrupt = signal(SignalKind::interrupt()).expect("SIGINT handler");
    let mut terminate = signal(SignalKind::terminate()).expect("SIGTERM handler");
    tokio::select! {
        _ = interrupt.recv() => "SIGINT",
        _ = terminate.recv() => "SIGTERM",
    }
}

/// The name of the next Ctrl-C to arrive. Windows has no SIGTERM.
#[cfg(not(unix))]
async fn next_signal() -> &'static str {
    let _ = tokio::signal::ctrl_c().await;
    "SIGINT"
}
