---
name: memory-tasks-typescript-migration
description: Migrating template from JavaScript to strict TypeScript compiled to dist/, with every dependency on its latest version and a clean npm audit; the version stays 0.0.0.
---

# TypeScript migration — template

## 2026-11-11 — complete

**Goal.** The owner asked for every Node.js repository to move to TypeScript, with dependencies on
their latest versions and no known vulnerabilities. This is the template, so every project
scaffolded from it starts typed.

**Objective.**

- Every dependency in `package.json` is on its latest version; `typescript`, `@types/node`, `tsx`
  and `@types/express` are devDependencies on their latest versions.
- `tsconfig.json` targets Node ESM: `target` `ES2022`, `module` and `moduleResolution` `NodeNext`,
  `rootDir` `./src`, `outDir` `./dist`, `strict` on.
- Every `.js` file in `src/` and `test/` is `.ts`, typed, with no implicit `any`, and behaves
  exactly as before.
- `package.json`: `main` and both `bin` entries point into `dist/`, `build` is `tsc`, `start`
  runs `dist/`, `test` runs `node --import tsx --test test/*.test.ts`.
- `npm run build` compiles with no errors, `npm test` passes the same 37 tests over stdio and HTTP,
  and `npm audit` reports 0 vulnerabilities.

**Status:** complete. The pull requests, merged in order by rebase, are in the table.

**Detail.** The version stays `0.0.0` (template-mode rule); the change ends on `release/0.0.0` with a
dated section in `wiki/logs/0/0/0/CHANGELOG.md`, no tag. The tool surface — four tools, their
names, descriptions and schemas — does not change. Only this repository is in scope; the other
three have their own tasks.

**Baseline, before any change:** `npm test` 37 passing; `npm audit` 9 vulnerabilities (4 moderate,
4 high, 1 critical); outdated: `@modelcontextprotocol/sdk` 1.30.0 → 1.32.2, `express` 5.2.1 → 5.3.0,
`zod` 4.5.4 → 4.6.7.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/typescript` | [#23](https://github.com/LXAgents-MCP/template/pull/23) |
| 2 | Latest dependencies and a clean audit | `build/dependencies` | [#24](https://github.com/LXAgents-MCP/template/pull/24) |
| 3 | Convert `src/` and `test/` to TypeScript | `refactor/typescript` | [#25](https://github.com/LXAgents-MCP/template/pull/25) |
| 4 | Docs, rules, `PROMPT.md` and `Dockerfile` for the build step | `docs/typescript` | [#26](https://github.com/LXAgents-MCP/template/pull/26) |
| 5 | Release | `release/0.0.0` | [#27](https://github.com/LXAgents-MCP/template/pull/27) |

Branches stacked: task 1 from `master`, task `k` from task `k-1`. Only this repository changes.

## Decisions

| # | Decision |
|---|---|
| D1 | The owner's request sets aside the local rule *What must not be introduced: a build step* in `.agents/rules/repository.md`. The rule and every page that says "no build step" are corrected in task 4. |
| D2 | `tsconfig.json` compiles `src/` only (`rootDir` `./src`). The tests are type-checked by a second config, `tsconfig.test.json` (`noEmit`, `src` + `test`), run as `npm run typecheck`, so they are held to `strict` without being emitted into `dist/`. |
| D3 | The HTTP and stdio tests spawn the compiled `dist/index.js`, not the source, so they test what ships. `pretest` runs `npm run build` so `npm test` never runs against a stale `dist/`. The in-memory tests import the TypeScript source through `tsx`. |
| D4 | Imports keep their `.js` extension in the TypeScript source, as `NodeNext` requires; `tsx` maps them to the `.ts` files. |
| D5 | `dist/` is ignored by git and is what the package ships (`files`); the `Dockerfile` gains a build stage. |
| D6 | A `prepare` script runs `npm run build`, so `npm install` in a checkout (and `npm link`, and a git dependency) leaves a working `dist/` behind the bins. The Docker stages pass `--ignore-scripts` and build explicitly. |

## Task entries

### Task 1 — plan/typescript

Landed: this record and its row in `memory-index.md`. Nothing outside `.agents/` changes.

### Task 2 — build/dependencies

Landed. Only `package.json` and `package-lock.json` change; nothing under `src/` or `test/`.

- Runtime, to latest: `@modelcontextprotocol/sdk` `^1.30.0` → `^1.32.2`, `express` `^5.2.1` →
  `^5.3.0`, `zod` `^4.5.4` → `^4.6.7`.
- New devDependencies, at latest: `typescript` `^7.0.2`, `@types/node` `^26.6.6`, `tsx` `^4.23.15`,
  `@types/express` `^5.0.6`.
- `npm audit` after the upgrade still found 5 (3 moderate, 2 high) in transitive packages —
  `fast-uri`, `hono`, `ip-address` among them. `npm audit fix` resolved all of them inside the
  existing ranges: `npm audit` now reports 0. `npm outdated` reports nothing.
- Lockfile version stays `0.0.0`.

Checked: `npm test` 37 passing, none failing, on the JavaScript source.

### Task 3 — refactor/typescript

Landed. Every `.js` file in `src/` and `test/` is renamed to `.ts` (git records each as a rename)
and typed under `strict`; behaviour is unchanged.

- `tsconfig.json`: `target` `ES2022`, `module`/`moduleResolution` `NodeNext`, `rootDir` `./src`,
  `outDir` `./dist`, `strict`, `noImplicitAny`, `noImplicitReturns`, `types: ["node"]` (TypeScript
  7 no longer loads every `@types` package by default), source maps. `tsconfig.test.json` extends
  it with `rootDir` `.` and `noEmit` over `src` + `test` (D2).
- `package.json`: `main` `dist/index.js`; `bin` `template` → `dist/cli.js`, `template-server` →
  `dist/index.js`; `files` `dist` + `README.md`; scripts `build` (`tsc`), `typecheck`
  (`tsc -p tsconfig.test.json`), `start`/`start:http`/`cli`/`inspect` run `dist/`, `pretest`
  runs build then typecheck, `test` is `node --import tsx --test test/*.test.ts`.
- `.gitignore`: `dist/`.
- New `src/models/tool.ts`, types only: `ToolConfig`, `ParameterlessTool`, `ParameterisedTool<Shape>`
  and the `ToolModule` union — the tool-authoring contract written as types. Each controller's
  default export is checked against it with `satisfies`; `calculate_sum` and `search_secure_data`
  export an args interface (`CalculateSumArgs`, `SearchSecureDataArgs`) and keep their zod shape in
  a `schema` constant.
- `src/routes/server.ts`: `ToolListing`, `ServerOptions`; `TOOL_MODULES` is `readonly ToolModule[]`;
  a `hasSchema` type guard (`Boolean(config.schema)`, the old truthiness test) picks the
  four-argument or three-argument `server.tool` form, as before.
- `src/routes/app.ts`: express `Request`/`Response`/`NextFunction`/`Express` types and an
  `HttpError` interface for the error handler. `primary.ts`/`worker.ts`: `NodeJS.Signals`, and a
  `Worker` type predicate in place of `filter(Boolean)`. `settings.ts`: return types replace the
  JSDoc `@returns`. `version.ts`: a `PackageManifest` interface; it still resolves
  `../../package.json`, which is the root from both `src/config/` and `dist/config/`.
- Tests: typed helpers and interfaces (`RunningServer`, `ExitStatus`, `RpcBody`, `HealthBody`, …).
  The HTTP and stdio tests spawn `dist/index.js` (D3). The one change beyond types: reading a tool
  result's text now asserts the block is `text` first, which the narrowing needs.

Checked: `npm run build` clean; `npm run typecheck` clean; `npm test` 37 passing, none failing;
`node dist/cli.js --version` prints `0.0.0` and `tools` lists the four samples; `npm audit` 0.

### Task 4 — docs/typescript

Landed. No code under `src/` or `test/` changes; `package.json` gains the `prepare` script (D6).

- `Dockerfile`: two stages. `build` runs `npm ci --ignore-scripts` and `npm run build`; the runtime
  stage installs production dependencies only and copies `dist/`. `CMD` is `node dist/index.js`.
  No Docker daemon was available here, so the two stages were replayed by hand in a scratch
  directory: the runtime tree had no `typescript`, `dist/cli.js tools` listed four tools, and
  `/healthz` answered `{"status":"ok","server":"template","version":"0.0.0"}`.
- `.agents/rules/repository.md`: *What must not be introduced* no longer bans a build step (D1); it
  bans JavaScript source and a bundler or second compiler. Commands and the *Where things go* table
  (with `src/models/`) updated.
- `.agents/rules/tool-authoring.md`: the sample tool is TypeScript with `satisfies`, and the
  `ParameterlessTool` / `ParameterisedTool` contract is named.
- `.agents/rules/template-mode.md`, `PROMPT.md`: paths are `.ts`; the scaffold's `ping` is
  TypeScript, `TOOL_MODULES` is typed, `src/models/tool.ts` is kept as the contract, and Step 3f
  says `npm test` builds and type-checks first.
- `.agents/wiki/context/repository-map.md`: layout tree rewritten (it still showed the pre-0.2.1
  `src/app.js` and `src/tools/`), commands, and two gotchas: `version.ts`'s path and tests running
  `dist/`.
- `README.md`, `AGENTS.md`, `wiki/information/{overview,architecture}.md`,
  `wiki/environments/{setup,env}.md`, `.agents/memory/state/repository-state.md`: "no build step"
  replaced, `.js` paths now `.ts`, run commands use `dist/`.
- Not changed: earlier task records, decisions and released changelogs, which keep their paths as
  history.

Checked: `npm ci` builds `dist/` through `prepare`; `npm test` 37 passing; `npm audit` 0.

### Task 5 — release/0.0.0

Landed. No version change and no tag (template-mode rule). `wiki/logs/0/0/0/CHANGELOG.md` gains the
dated *2026-11-11 — TypeScript* section, and the `0.0.0` row in `logs-index.md` names it. This
commit fills the `PR` column and closes the record.

Final state checked on this branch: `npm run build` clean, `npm run typecheck` clean, `npm test` 37
passing, `npm audit` 0 vulnerabilities.
