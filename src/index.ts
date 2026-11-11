#!/usr/bin/env node

/*
 * Server entry point: chooses the transport, and does nothing else.
 *
 * HTTP runs as a cluster primary that forks workers (`src/routes/primary.ts`), or as one of those
 * workers (`src/routes/worker.ts`). stdio serves this process's own pipe.
 *
 * Nothing here may write to stdout: on stdio, stdout is the JSON-RPC channel.
 */

import cluster from "node:cluster";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { transportName } from "./config/settings.js";
import { version } from "./config/version.js";
import { startPrimary } from "./routes/primary.js";
import { SERVER_ID, createServer } from "./routes/server.js";
import { startWorker } from "./routes/worker.js";

const transport = transportName();

if (transport === "http" || transport === "streamable-http") {
  if (cluster.isPrimary) {
    startPrimary();
  } else {
    startWorker();
  }
} else {
  // stdio never forks. stdout is the JSON-RPC channel here, and a worker's copy of it
  // would corrupt the stream.
  const server = createServer({ version });
  await server.connect(new StdioServerTransport());
  process.stderr.write(`${SERVER_ID} ${version} serving over stdio\n`);
}
