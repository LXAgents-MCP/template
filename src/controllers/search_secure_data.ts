/*
 * Sample tool: takes zod-validated parameters and requires the server API key.
 *
 * Deleted at scaffold time - see PROMPT.md.
 */

import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js";
import { z } from "zod";
import type { ParameterisedTool } from "../models/tool.js";

const schema = {
  query: z.string().min(1).describe("The search term to match records against."),
};

/** The parsed arguments, as `schema` guarantees them. */
export interface SearchSecureDataArgs {
  query: string;
}

export const config = {
  name: "search_secure_data",
  description:
    "Search the protected dataset and return matching records. Requires the API_KEY environment variable.",
  schema,
};

export async function handler({ query }: SearchSecureDataArgs): Promise<CallToolResult> {
  const apiKey = process.env.API_KEY;
  if (!apiKey) {
    throw new Error(
      "search_secure_data requires an API key. Set the API_KEY environment variable before starting the server."
    );
  }

  return {
    content: [
      { type: "text", text: `No records matched "${query}".` },
    ],
  };
}

export default { config, handler } satisfies ParameterisedTool<typeof schema>;
