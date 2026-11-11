# 0.0.0 — the template's rolling log

The template's version is always `0.0.0` (see `.agents/rules/template-mode.md`). It has no releases
of its own, so every change to it is logged here, newest first, one dated section each. The logs for
`0.1.0` to `0.2.2` are history from before the version was pinned.

## 2026-11-11 — TypeScript

The template is now strict TypeScript compiled to `dist/`, and every dependency is on its latest
version with a clean `npm audit`. The tool surface — four sample tools, their names, descriptions
and schemas — and both transports behave exactly as before. A project scaffolded from here starts
typed.

### Changed

- `src/` and `test/`: every `.js` file is now `.ts`, typed under `strict`. New `src/models/tool.ts`
  holds the tool module contract as types; each tool's default export is checked against it with
  `satisfies`.
- `package.json`: `main` is `dist/index.js`; the bins are `dist/cli.js` and `dist/index.js`; the
  package ships `dist/`. New scripts `build` (`tsc`), `typecheck`, and `prepare` (builds on
  install); `start`, `start:http`, `cli` and `inspect` run `dist/`; `test` builds and type-checks
  first, then runs `node --import tsx --test test/*.test.ts`.
- Dependencies: `@modelcontextprotocol/sdk` 1.32.2, `express` 5.3.0, `zod` 4.6.7. New
  devDependencies `typescript`, `@types/node`, `tsx`, `@types/express`. `npm audit` 9 → 0.
- `Dockerfile`: a build stage compiles `dist/`; the image ships `dist/` and production
  dependencies only, and runs `node dist/index.js`.
- `PROMPT.md`, the rules and the wiki describe the TypeScript layout; the scaffold's `ping` tool
  is TypeScript.

### Added

- `tsconfig.json` (ES2022, NodeNext, `rootDir` `src`, `outDir` `dist`, `strict`) and
  `tsconfig.test.json` (type-checks the tests, emits nothing).

What a reader has to do: run `npm install` (which builds) or `npm run build` before running from a
checkout; anything that ran `node src/index.js` now runs `node dist/index.js`. An MCP client
config with `"args": ["src/index.js"]` must change to `dist/index.js`.

## 2026-11-11 — the version is pinned at 0.0.0

The template now says `0.0.0` and keeps saying it. A project scaffolded from it gets its own version
from `PROMPT.md` and no longer inherits one that reads as a release.

### Changed

- `package.json` and `package-lock.json`: `0.2.2` → `0.0.0`. `template --version`, `/healthz` and
  the startup lines report `0.0.0`.
- `PROMPT.md`: scaffolding sets the Initial Version in the lockfile as well as `package.json`, shows
  it in the sample output, and resets the release history (`wiki/logs/` and the logs index) to the
  new project's first version.
- `.agents/rules/template-mode.md`: the version stays `0.0.0` while this is a template; a template
  change ends on `release/0.0.0` with a section in this log; no tags.
- `wiki/environments/setup.md`, `wiki/environments/env.md`: sample output shows `0.0.0`.

What a reader has to do: nothing. Anyone who pinned the template's `0.2.x` should know those
versions were never published and no tag exists for them.
