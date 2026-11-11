/*
 * Sample tool: takes zod-validated parameters, needs no API key.
 *
 * `schema` is a zod raw shape, not a z.object - that is what
 * server.tool(name, description, schema, handler) expects.
 *
 * Deleted at scaffold time - see PROMPT.md.
 */

import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js";
import { z } from "zod";
import type { ParameterisedTool } from "../models/tool.js";

const schema = {
  a: z.number().describe("The first addend."),
  b: z.number().describe("The second addend."),
};

/** The parsed arguments, as `schema` guarantees them. */
export interface CalculateSumArgs {
  a: number;
  b: number;
}

export const config = {
  name: "calculate_sum",
  description:
    "Add two numbers and return the sum. Requires no API key.",
  schema,
};

export async function handler({ a, b }: CalculateSumArgs): Promise<CallToolResult> {
  return {
    content: [{ type: "text", text: String(a + b) }],
  };
}

export default { config, handler } satisfies ParameterisedTool<typeof schema>;
