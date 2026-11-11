import cluster from "node:cluster";
import type { Worker } from "node:cluster";
import { httpPort, workerCount } from "../config/settings.js";
import { version } from "../config/version.js";
import { SERVER_ID } from "./server.js";
import { startWorker } from "./worker.js";

/**
 * The primary: the process that forks workers and does not serve.
 *
 * It binds nothing, so the startup lines in a container's log describe ports that are
 * genuinely open — one per worker, from the processes that opened them. A primary that
 * logged a listening line of its own would be claiming a port it does not hold.
 */
export function startPrimary(): void {
  const count = workerCount();
  const port = httpPort();

  // Read by the respawn handler below, and set by the signal handler further down, so
  // it is declared before either can run rather than beside the one that reads it
  // first.
  let draining = false;

  if (count <= 1) {
    // No fork. The worker path is the whole server, and the two lines below are what a
    // single-process run prints; saying so is worth a line of its own.
    process.stderr.write(
      `${SERVER_ID} ${version} MCP_CLUSTER_WORKERS is ${count}, so no worker is forked.\n`
    );
    startWorker();
    return;
  }

  process.stderr.write(
    `${SERVER_ID} ${version} forking ${count} HTTP workers on :${port}/mcp\n`
  );

  // A worker that exits unexpectedly is replaced, but not forever: a server that cannot
  // start its workers is a server that should say so and stop, not one that respawns
  // into a crash loop nobody is watching.
  let starts = 0;

  cluster.on("exit", (worker, code, signal) => {
    if (draining) return;
    if (starts > count * 10) {
      process.stderr.write(
        `${SERVER_ID} ${version} a worker exited ${code ?? signal} ${starts} times, not restarting it.\n`
      );
      process.exit(1);
      return;
    }
    process.stderr.write(
      `${SERVER_ID} ${version} worker ${worker.process.pid} exited ${code ?? signal}, restarting it\n`
    );
    forkWorker();
  });

  function forkWorker(): void {
    starts += 1;
    cluster.fork();
  }

  for (let i = 0; i < count; i += 1) forkWorker();

  /*
   * Relay the signal, then wait.
   *
   * The signal goes to the workers rather than being handled here alone, because the
   * workers hold the requests and the listener. The primary exits when the last worker
   * is gone, so the port is genuinely closed before the process that started it is —
   * a test that stops the server and then checks the port is refused must not race a
   * primary that exits while its workers are still answering.
   */
  const shutdown = (signal: NodeJS.Signals): void => {
    if (draining) {
      // A second signal means the operator has stopped waiting.
      process.exit(0);
    }
    draining = true;

    const workers = Object.values(cluster.workers ?? {}).filter(
      (worker): worker is Worker => Boolean(worker)
    );
    process.stderr.write(`${SERVER_ID} ${version} ${signal}, draining ${workers.length} worker(s)\n`);

    // Unref'd: this timer is a backstop for a wedged worker, not a reason to keep the
    // process alive when every worker has already gone.
    const forced = setTimeout(() => process.exit(0), 10_000);
    forced.unref();

    let remaining = workers.length;
    for (const worker of workers) {
      worker.once("exit", () => {
        remaining -= 1;
        if (remaining === 0) {
          clearTimeout(forced);
          process.exit(0);
        }
      });
    }

    for (const worker of workers) worker.kill(signal);
  };

  process.on("SIGTERM", () => shutdown("SIGTERM"));
  process.on("SIGINT", () => shutdown("SIGINT"));
}
