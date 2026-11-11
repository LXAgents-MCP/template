/*
 * The MCP server.
 *
 * A fresh instance is created per connection because McpServer holds
 * per-connection state.
 *
 * Every tool lives in its own file under src/controllers/. Adding one means two edits:
 * the new file, and an import plus an entry in TOOL_MODULES below. Nothing else
 * registers tools - listTools() and the CLI both read this array, so a tool
 * registered outside it would be invisible to both.
 */

import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";

import type { ParameterisedTool, ToolModule } from "../models/tool.js";
import calculateSum from "../controllers/calculate_sum.js";
import getSecureSummary from "../controllers/get_secure_summary.js";
import getServerTime from "../controllers/get_server_time.js";
import searchSecureData from "../controllers/search_secure_data.js";

export const SERVER_ID = "template";
export const SERVER_TITLE = "Template";

/** A registered tool as the CLI lists it. */
export interface ToolListing {
  name: string;
  description: string;
}

/** What `createServer` needs to know about the process it serves. */
export interface ServerOptions {
  version: string;
}

const TOOL_MODULES: readonly ToolModule[] = Object.freeze([
  getServerTime,
  getSecureSummary,
  calculateSum,
  searchSecureData,
]);

/**
 * The registered tools, as name/description pairs.
 *
 * The CLI prints this rather than keeping a list of its own, so the two
 * surfaces cannot drift apart.
 *
 */
export function listTools(): ToolListing[] {
  return TOOL_MODULES.map(({ config }) => ({
    name: config.name,
    description: config.description,
  }));
}

/**
 * Whether `tool` declares a schema, and so is registered with the four-argument form.
 */
function hasSchema(tool: ToolModule): tool is ParameterisedTool {
  return Boolean(tool.config.schema);
}

/**
 * A fresh MCP server with every tool in `TOOL_MODULES` registered on it.
 */
export function createServer({ version }: ServerOptions): McpServer {
  const server = new McpServer(
    { name: SERVER_ID, title: SERVER_TITLE, version },
    { instructions: `${SERVER_TITLE} - call the tools listed below.` }
  );

  for (const tool of TOOL_MODULES) {
    if (hasSchema(tool)) {
      server.tool(tool.config.name, tool.config.description, tool.config.schema, tool.handler);
    } else {
      server.tool(tool.config.name, tool.config.description, tool.handler);
    }
  }

  return server;
}
