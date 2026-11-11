/**
 * Every environment variable the server reads, in one place.
 *
 * Read when called and not at import, so a test or a wrapper that sets a variable before it starts
 * the server is seen. Nothing here writes anywhere.
 */
import { availableParallelism } from "node:os";

/**
 * The transport this process speaks: `MCP_TRANSPORT`, lowercased, `stdio` when unset.
 *
 * @returns {string}
 */
export function transportName() {
  return (process.env.MCP_TRANSPORT ?? "stdio").toLowerCase();
}

/**
 * The HTTP port: `PORT`, 3000 when unset.
 *
 * @returns {number}
 */
export function httpPort() {
  return Number.parseInt(process.env.PORT ?? "3000", 10);
}

/**
 * The bind address, named rather than implied.
 *
 * `listen(port)` with no host binds every interface, which is what a published port needs
 * and what a container gets — but it is a decision nobody made, so it is made here,
 * visibly, and the startup line reports it. `HOST=127.0.0.1` is the way to take it back.
 *
 * @returns {string}
 */
export function httpHost() {
  return process.env.HOST || "0.0.0.0";
}

/**
 * How many HTTP workers to run.
 *
 * `MCP_CLUSTER_WORKERS` overrides the count. **A value of 1 means no forking at all** —
 * one process, one listener, the pre-cluster behaviour — which is what makes the
 * cluster change bisectable: the same code answers, with and without workers, and a
 * difference between them is a difference in the fork rather than in the transport.
 *
 * Unset, the count is the number of CPUs the process was actually given, not a
 * constant: a container with two CPUs gets two workers and a laptop does not get
 * eight.
 *
 * Only a positive integer is honoured. A count of zero would be a server that binds
 * nothing and answers nothing, which reads like a configuration and behaves like an
 * outage, so `0`, a negative number and anything unparseable all fall through to the
 * default rather than being taken at face value.
 *
 * @returns {number}
 */
export function workerCount() {
  const configured = Number.parseInt(process.env.MCP_CLUSTER_WORKERS ?? "", 10);
  if (Number.isInteger(configured) && configured >= 1) return configured;
  return Math.max(1, availableParallelism());
}

/**
 * The `Host` header allow-list, when one is configured.
 *
 * **This guard is new in this repository** — the four sibling LXAgents MCP servers have
 * had it for some time, and this template had none, so a project scaffolded from it
 * would have started unguarded. The task record for the migration says so in the same
 * words; see `.agents/memory/tasks/express-cluster-migration.md`. The matching code is
 * deliberately identical to the siblings', because a template that ships a *different*
 * guard is a template that ships a guard nobody has reviewed.
 *
 * Unset — or set to nothing but commas and spaces — means the check is skipped rather
 * than guessed at. An allow-list that silently refuses every request is a worse failure
 * than an absent one, and a control that is off silently reads as present, so the
 * absence is reported on startup rather than papered over with a default list.
 *
 * @returns {string[]}
 */
export function allowedHosts() {
  const raw = process.env.MCP_ALLOWED_HOSTS;
  if (!raw) return [];
  return raw
    .split(",")
    .map((host) => host.trim())
    .filter(Boolean);
}
