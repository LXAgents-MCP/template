/*
 * Sample tool: read-only, takes no arguments, needs no API key.
 *
 * Deleted at scaffold time - see PROMPT.md.
 */

import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js";
import type { ParameterlessTool } from "../models/tool.js";

export const config = {
  name: "get_server_time",
  description:
    "Return the server's current time as an ISO 8601 timestamp in UTC. Takes no arguments and requires no API key.",
};

export async function handler(): Promise<CallToolResult> {
  return {
    content: [{ type: "text", text: new Date().toISOString() }],
  };
}

export default { config, handler } satisfies ParameterlessTool;
