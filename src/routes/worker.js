import { allowedHosts, httpHost, httpPort } from "../config/settings.js";
import { version } from "../config/version.js";
import { createApp } from "./app.js";
import { SERVER_ID } from "./server.js";

/**
 * The worker: the process that actually answers.
 *
 * Every worker binds the same `port`. The kernel's shared handle and the round-robin
 * scheduler do the distribution — no `SO_REUSEPORT` is set by hand and no sticky
 * session logic is written, because the scheduler already has the information
 * (which connection is next) that a sticky-session scheme would have to reconstruct.
 */
export function startWorker() {
  const port = httpPort();
  const host = httpHost();

  // A worker whose primary is gone holds the port for whoever starts next. The test
  // harness kills the child process directly, so this is the difference between a
  // suite that passes and a suite that fails on its second run with EADDRINUSE.
  process.on("disconnect", () => process.exit(0));

  const httpServer = createApp().listen(port, host, () => {
    const where = host === "0.0.0.0" ? "all interfaces" : host;
    process.stderr.write(
      `${SERVER_ID} ${version} serving over http on :${port}/mcp (${where})\n`
    );

    // Said out loud, because the default is the unguarded one. Someone reading a
    // container's startup log is the only person who can act on it, and a control
    // that is off silently is worse than no control at all — it reads as present.
    if (allowedHosts().length === 0) {
      process.stderr.write(
        `${SERVER_ID} ${version} MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied.\n`
      );
    }
  });

  /*
   * Shutdown, in three steps, in this order.
   *
   * `close()` first, so nothing new arrives — a request accepted during the drain gets
   * an answer rather than a refused connection. Then idle keep-alive sockets are
   * closed, because `close()` waits on them and a client that opened one and went quiet
   * would hold the process open indefinitely for a request that no longer exists. Then
   * a short grace period, after which whatever is genuinely still in flight is cut off
   * rather than waited on forever.
   *
   * Each request is self-contained — a fresh McpServer, closed when its response
   * closes — so there is no session state to drain. What drains is the requests.
   *
   * **The drain line is written here, by the worker**, because the worker is the
   * process actually draining. The primary writes its own line about the workers it is
   * relaying the signal to; two lines, from two processes, describing two different
   * jobs.
   */
  let shuttingDown = false;

  const shutdown = (signal) => {
    if (shuttingDown) return;
    shuttingDown = true;
    process.stderr.write(`${SERVER_ID} ${version} ${signal}, draining\n`);

    const forced = setTimeout(() => httpServer.closeAllConnections(), 5000);
    forced.unref();

    httpServer.close(() => {
      clearTimeout(forced);
      process.exit(0);
    });
    httpServer.closeIdleConnections();
  };

  process.on("SIGTERM", () => shutdown("SIGTERM"));
  process.on("SIGINT", () => shutdown("SIGINT"));
}
