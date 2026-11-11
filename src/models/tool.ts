/*
 * The shape of a tool module, as types only.
 *
 * Every file under src/controllers/ exports a `config`, a `handler`, and a default that pairs
 * them. These types are that contract written down: `src/routes/server.ts` registers whatever
 * satisfies them, and a tool file that drifts from the contract fails to compile instead of
 * registering a tool with no parameters.
 *
 * Types only - nothing here exists at runtime, so no sample tool depends on code outside
 * src/controllers/ and scaffolding can still delete all four cleanly.
 */

import type { ToolCallback } from "@modelcontextprotocol/sdk/server/mcp.js";
import type { ShapeOutput, ZodRawShapeCompat } from "@modelcontextprotocol/sdk/server/zod-compat.js";
import type { CallToolResult } from "@modelcontextprotocol/sdk/types.js";

/** What every tool declares: the registered name and the one line an agent routes on. */
export interface ToolConfig {
  readonly name: string;
  readonly description: string;
}

/** A tool that takes no arguments: no `schema`, and a handler that is given none. */
export interface ParameterlessTool {
  readonly config: ToolConfig & { readonly schema?: undefined };
  readonly handler: ToolCallback;
}

/**
 * A tool that takes arguments: a zod raw shape, and a handler given the parsed arguments.
 *
 * `handler` is declared with method syntax on purpose. Method parameters are compared
 * bivariantly, which is what lets a module typed for its own shape - `{ a: number, b: number }`
 * - sit in the one `TOOL_MODULES` array next to a module typed for another.
 */
export interface ParameterisedTool<Shape extends ZodRawShapeCompat = ZodRawShapeCompat> {
  readonly config: ToolConfig & { readonly schema: Shape };
  handler(args: ShapeOutput<Shape>): Promise<CallToolResult>;
}

/** Any tool module `src/routes/server.ts` can register. */
export type ToolModule = ParameterlessTool | ParameterisedTool;

