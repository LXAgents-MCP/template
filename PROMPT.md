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
3. **Update `src/server.js`**:
   - Update `SERVER_ID` and `SERVER_TITLE` to match the new project.
4. **Update `src/cli.js`**:
   - Update the `HELP` text to use the new CLI command name and description.
5. **Update `wiki/environments/setup.md`**:
   - Replace all references of the old template names with the new project names.
6. **Update `AGENTS.md`**:
   - **Keep the "Shared Instruction Set" section and the trigger table exactly as they
     are.** Every consuming repository carries that bootstrap block verbatim, and the
     table is mirrored row-for-row from `agents://rules/auto-activation.md` — this is
     the mechanism by which the new project resolves the shared set at all. Deleting
     either leaves a repository whose conventions never activate.
   - Remove the "Project Scaffolding (Template Mode)" section, and the
     `Scaffold a new project from this template` row from the trigger table, as the
     repository is no longer a template.
   - Append rows to the trigger table for any local instruction the new project adds
     under `.agents/`. Never remove, reorder, or repoint a mirrored row — repointing is
     an override and is declared in `.agents/index/root-index.md`.
   - Update the description in the frontmatter.
7. **Delete `PROMPT.md`** (Self-destruct):
   - This file is only for the template. Remove it once scaffolding is complete.

## Step 4: Finalize
Provide the user with a summary of the changed files and ask if they would like you to commit these initial scaffolding changes following the standard LXAgents commit conventions (e.g., `chore(setup): ...`).
