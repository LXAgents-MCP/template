---
name: memory-tasks-typescript-migration
description: Migrating template from JavaScript to strict TypeScript compiled to dist/, with every dependency on its latest version and a clean npm audit; the version stays 0.0.0.
---

# TypeScript migration — template

## 2026-11-11 — in progress

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
| 1 | Task record | `plan/typescript` | |
| 2 | Latest dependencies and a clean audit | `build/dependencies` | |
| 3 | Convert `src/` and `test/` to TypeScript | `refactor/typescript` | |
| 4 | Docs, rules, `PROMPT.md` and `Dockerfile` for the build step | `docs/typescript` | |
| 5 | Release | `release/0.0.0` | |

Branches stacked: task 1 from `master`, task `k` from task `k-1`. Only this repository changes.

## Decisions

| # | Decision |
|---|---|
| D1 | The owner's request sets aside the local rule *What must not be introduced: a build step* in `.agents/rules/repository.md`. The rule and every page that says "no build step" are corrected in task 4. |
| D2 | `tsconfig.json` compiles `src/` only (`rootDir` `./src`). The tests are type-checked by a second config, `tsconfig.test.json` (`noEmit`, `src` + `test`), run as `npm run typecheck`, so they are held to `strict` without being emitted into `dist/`. |
| D3 | The HTTP and stdio tests spawn the compiled `dist/index.js`, not the source, so they test what ships. `pretest` runs `npm run build` so `npm test` never runs against a stale `dist/`. The in-memory tests import the TypeScript source through `tsx`. |
| D4 | Imports keep their `.js` extension in the TypeScript source, as `NodeNext` requires; `tsx` maps them to the `.ts` files. |
| D5 | `dist/` is ignored by git and is what the package ships (`files`); the `Dockerfile` gains a build stage. |

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
