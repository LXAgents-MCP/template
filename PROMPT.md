# Project Scaffolding Prompt

You are acting as a scaffolding agent. This repository is a template for a dual-purpose MCP server and CLI. When the user initiates the scaffolding process, follow these steps strictly.

## Step 1: Gather Requirements
Ask the user for the following information in a single, clear message. Do not proceed to modify files until you have gathered all necessary details (or the user explicitly tells you to use defaults).

1. **Project Name** (e.g., `@my-org/my-mcp-server`)
2. **Project Description** (A short, one-line description)
3. **Repository URL** (e.g., `https://github.com/my-org/my-mcp-server` - note: always assume github.com and gitlab.com refer to the cloud-hosted providers)
4. **Organization / Author Name**
5. **CLI Command Name** (e.g., `my-cli` - defaults to the project name without scope)
6. **MCP Server ID** (e.g., `my-mcp-server` - defaults to the CLI command name)
7. **License** (e.g., MIT, Apache-2.0 - defaults to MIT)
8. **Initial Version** (e.g., `0.1.0`)

## Step 2: Confirmation
Once the user provides the details, summarize the values you will use and ask for a final confirmation before modifying any files.

## Step 3: Execute Scaffolding
Upon confirmation, perform the following actions:
1. **Update `package.json`**:
   - Update `name`, `version`, `description`, `author`, `license`.
   - Update `repository.url`.
   - Update the `bin` object keys to match the new **CLI Command Name** and **MCP Server ID** (e.g., `<cli-name>-server`).
2. **Update `README.md`**:
   - Replace the title, description, and repository details.
   - Update the quick start and installation commands to use the new package name and CLI command.
   - Remove the "Sample tools" table and the "Scaffolding a new project" section. Replace
     the table with a one-line mention of the `ping` tool created in step 3.
   - Keep the features list, but replace the template's `Host` allow-list bullet with one
     that describes **this** project's guard. Do not delete the bullet: a project that
     inherits the transport inherits the guard, and an inherited control nobody
     mentioned is a control nobody turned on.
3. **Reset the tool layer** (`src/tools/` and `src/server.js`):

   The template ships four sample tools that exist only to demonstrate what the tool
   layer can do. A new project must not inherit them.

   a. **Delete all four sample tool files:**

   ```bash
   rm src/tools/get_server_time.js \
      src/tools/get_secure_summary.js \
      src/tools/calculate_sum.js \
      src/tools/search_secure_data.js
   ```

   b. **Create a single basic `src/tools/ping.js`** as the new project's starting point.
   It takes no arguments and requires no API key:

   ```js
   /*
    * The starting tool. Replace it with the project's own.
    */

   export const config = {
     name: "ping",
     description: "Return pong, to prove the server is reachable. Takes no arguments.",
   };

   export async function handler() {
     return {
       content: [{ type: "text", text: "pong" }],
     };
   }

   export default { config, handler };
   ```

   c. **Modify `src/server.js` to import and register only `ping`.** Delete the four
   sample imports and replace the `TOOL_MODULES` array so it holds `ping` alone:

   ```js
   import ping from "./tools/ping.js";

   const TOOL_MODULES = Object.freeze([ping]);
   ```

   Leave `listTools()` and `createServer()` as they are. The registration loop already
   handles both the schema and the no-schema form, so a later tool that declares a zod
   schema needs no change here — only a new file and a new entry in the array.

   d. **Update `test/server.test.js`** so the suite passes against the single `ping`
   tool. The template's tests assert the four sample names, their zod schemas, and the
   `API_KEY` behaviour; all of that is gone. Keep the surface-parity test — it is what
   holds the CLI and the MCP server together — and keep a test that calls `ping` and
   asserts it returns `pong`. Delete the rest.

   e. **Update `test/http.test.js`**, which is the new project's only coverage of the
   transport it inherits. It asserts four tools, calls `calculate_sum` and
   `get_server_time` by name, and uses `template.example.test` as the allow-listed host.
   Retarget the tool assertions at `ping`, fix the count, and rename the host to
   something belonging to the new project. **Do not delete the file and do not delete
   the guard or worker tests**: the `Host` allow-list, the body limit, the 404 and 405,
   the error mapping, and the fact that N workers can share one port and none of them
   survives its primary, are exactly the properties a new project has no reason to write
   tests for twice, and they are what keeps the inherited transport from regressing.

   f. **Run `npm test`** and confirm it passes before continuing. A scaffolded project
   whose suite is red on the first commit is the failure this step exists to prevent.
4. **Update `src/server.js` identity**:
   - Update `SERVER_ID` and `SERVER_TITLE` to match the new project.
5. **Update `src/cli.js`**:
   - Update the `HELP` text to use the new CLI command name and description.
   - Drop the `API_KEY` line from the `Environment` block unless the new project actually
     uses a server-wide key.
   - Keep the `HOST`, `MCP_ALLOWED_HOSTS` and `MCP_CLUSTER_WORKERS` lines. They are the
     only place the guard and the worker count are named on the surface a new user reads
     first, and step 8 decides what they say.
6. **Update `wiki/environments/setup.md`**:
   - Replace all references of the old template names with the new project names.
   - Replace the sample `tools` output with the `ping` row.
   - Remove the "Authentication" section unless the new project uses `API_KEY`.
   - Keep the HTTP run forms, the `/healthz` check, and the note that `/healthz` is
     covered by `MCP_ALLOWED_HOSTS`, updating the sample values to the new project's
     names. The new project inherits both the transport and the guard; a probe that uses
     a `Host` outside the allow-list gets a 403, and the reader of this page is the one
     who has to know that before they deploy it.
7. **Update the rest of the documentation**:
   - `wiki/information/overview.md`: remove the sample tool table and the template
     framing; describe the new project.
   - `wiki/information/architecture.md`: keep the structure, but replace the
     `calculate_sum` example with `ping` and drop the authentication section if unused.
     Keep the `app.js` / `index.js` split, the transport paragraphs, the `Host` guard
     section, the body-limit section and the shutdown section — they describe the
     transport the new project inherits, and `src/app.js` survives scaffolding.
   - `wiki/environments/env.md`: remove `API_KEY` unless the new project uses it. Keep
     the `HOST`, `MCP_ALLOWED_HOSTS` and `MCP_CLUSTER_WORKERS` rows and the two sections
     that go with them, and fix the "six variables" count to whatever the new project
     actually has.
8. **Decide the `Host` guard.** The new project inherits `MCP_ALLOWED_HOSTS` in
   `src/app.js`, and it is **off unless the variable is set** — the middleware is not
   mounted at all, every request is served, and the startup line says
   `MCP_ALLOWED_HOSTS is unset, so no Host header allow-list is applied.` That is a
   deliberate default, matching the other LXAgents MCP servers, and an allow-list that
   silently refuses every request is a worse failure than an absent one.

   **Ask the user where the server will be reachable from, and record the answer in the
   project, not only in this conversation.** If it will be reachable from anywhere but
   the user's own machine, set `MCP_ALLOWED_HOSTS` in the project's deployment
   documentation and say plainly in the README that the server is unguarded until it is
   set. Do not change the default to refuse-by-default and do not invent a default
   allow-list: a host list that guesses wrong takes down a working deployment, and the
   four sibling repositories all keep the same off-by-default semantics for this reason.

   Also decide `HOST`. It defaults to `0.0.0.0` — every IPv4 interface — and the startup
   line names what was bound. A project that should only be reachable locally wants
   `HOST=127.0.0.1` in its setup instructions.

   And decide `MCP_CLUSTER_WORKERS`, which the new project inherits: it forks one worker
   per CPU onto the one `PORT`, and `1` means no fork at all. A container that is given
   more CPUs than it needs, or a project that wants one process for a simpler log, can
   set it; nothing else has to change. Keep the variable and its `env.md` section — it is
   the same in every LXAgents MCP repository, and dropping it would make a generated
   project the odd one out.
9. **Update `AGENTS.md`**:
   - **Keep the "Shared Instruction Set" section and the trigger table exactly as they
     are.** Every consuming repository carries that bootstrap block verbatim, and the
     table is mirrored row-for-row from `agents://rules/auto-activation.md` — this is
     the mechanism by which the new project resolves the shared set at all. Deleting
     either leaves a repository whose conventions never activate.
   - Remove the "Project Scaffolding (Template Mode)" section, and the
     `Scaffold a new project from this template` row from the trigger table, as the
     repository is no longer a template.
   - Remove the `Change the sample tools, PROMPT.md, or anything a scaffolded project
     inherits` row — the local rule it points at is deleted in step 10. Keep the
     `tool-authoring` and `secrets` rows if those rules are kept.
   - Append rows to the trigger table for any local instruction the new project adds
     under `.agents/`. Never remove, reorder, or repoint a mirrored row — repointing is
     an override and is declared in `.agents/index/root-index.md`.
   - Update the description in the frontmatter.
10. **Update the local instruction set** (`.agents/`):
   - Delete `.agents/rules/template-mode.md`. It describes template mode, which is over,
     and a rule that can now only mislead.
   - Remove its row from `.agents/index/agents-index.md`.
   - Update `.agents/wiki/context/repository-map.md`: replace the sample tool listing
     with `ping`, and drop the "the four tools are samples" gotcha. **Keep** the
     `src/app.js` row, the `test/http.test.js` row, the `HOST` and `MCP_ALLOWED_HOSTS`
     environment rows, and the "an unset `MCP_ALLOWED_HOSTS` is the guard being off"
     gotcha — all of it describes the transport the new project inherits, and the new
     project's agent is the reader.
   - Rewrite `.agents/memory/state/repository-state.md` for the new project's starting
     state, and delete `.agents/memory/tasks/mcp-tools-refactor.md` and
     `.agents/memory/tasks/express-cluster-migration.md`, which record work done on the
     template rather than on this project. Keep the substance of
     `.agents/memory/decisions/express-for-http-transport.md` only if the new project
     still speaks of the guard as *new*; once it is inherited, rewrite it as the
     decision it now is, or delete it with the rest of the template's history.
   - In `.agents/memory/decisions/harness-branch-naming.md`, delete the `History`
     section: the decision itself carries over to the new project, but that section
     narrates work done on the template. Delete the whole file instead if the shared set
     has since absorbed the rule.
   - Update every row in `.agents/index/memory-index.md` to match what survives.
   - Keep `.agents/rules/secrets.md` only if the new project uses an API key; delete it
     and its index row otherwise.
11. **Delete `PROMPT.md`** (Self-destruct):
   - This file is only for the template. Remove it once scaffolding is complete.

## Step 4: Finalize
Provide the user with a summary of the changed files and ask if they would like you to commit these initial scaffolding changes following the standard LXAgents commit conventions (e.g., `chore(setup): ...`).
