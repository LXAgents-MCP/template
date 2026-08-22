/*
 * The MCP server.
 *
 * A fresh instance is created per connection because McpServer holds
 * per-connection state.
 */

import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";

export const SERVER_ID = "template";
export const SERVER_TITLE = "Template";

export const TOOLS = Object.freeze([
  {
    name: "ping",
    title: "Ping",
    description: "Return pong, to prove the server is reachable. Takes no arguments.",
    run: async () => "pong",
  },
]);

/**
 * @param { version: string } options
 * @returns { McpServer }
 */
export function createServer({ version }) {
  const server = new McpServer(
    { name: SERVER_ID, title: SERVER_TITLE, version },
    { instructions: `${SERVER_TITLE} - call the tools listed below.` }
  );

  for (const tool of TOOLS) {
    server.registerTool(
      tool.name,
      {
        title: tool.title,
        description: tool.description,
        annotations: {
          readOnlyHint: true,
          destructiveHint: false,
          idempotentHint: true,
          openWorldHint: false,
        },
      },
      async () => ({
        content: [{ type: "text", text: await tool.run() }],
      })
    );
  }

  return server;
}
