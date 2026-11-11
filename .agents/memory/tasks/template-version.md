---
name: memory-tasks-template-version
description: Pinning the template's own version at 0.0.0 for good, because every project scaffolded from it starts from this package.json; the version a project ships is set by PROMPT.md at scaffold time.
---

# Template version — template

## 2026-11-11 — in progress

**Goal.** The owner: "Template repo must always stay at version 0.0.0, since it will be reused by
template." The template is not a product with releases of its own. Whatever version it carries is
inherited by every scaffolded project until `PROMPT.md` overwrites it, and `0.2.2` (cut earlier
today) reads as a real release.

**Objective.**

- `package.json` and both lockfile entries say `0.0.0`; `template --version` and `/healthz` report it.
- A local rule says the version stays `0.0.0` while this is a template, so no later task bumps it.
- The docs that show a version in sample output show `0.0.0`.
- `PROMPT.md` Step 3 resets the release history when scaffolding, so a new project does not inherit
  logs for versions it never shipped.
- `npm test` passes, 37 tests.

**Detail.** Released logs for `0.1.0`–`0.2.2` are history and are not rewritten or deleted
(the shared changelog rule forbids it). No tag exists for any of them, and none is made.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/template-version` | |
| 2 | Pin the version at 0.0.0 and write the rule | `chore/template-version` | |
| 3 | Release | `release/0.0.0` | |

Branches stacked: task 1 from `master`, task `k` from task `k-1`. Only this repository changes.

## Decisions

| # | Decision |
|---|---|
| D1 | The rule lives in `.agents/rules/template-mode.md`, because it holds only while `PROMPT.md` exists; scaffolding deletes that file and the new project versions itself. `AGENTS.md`'s version rule links to it. |
| D2 | The project instructions require a final `release/{version}` branch. With the version fixed, that branch is `release/0.0.0` for every template change, and its notes go in one rolling log, `wiki/logs/0/0/0/CHANGELOG.md`, with a dated section per change. |
| D3 | No test pins `0.0.0`: a scaffolded project changes the version on purpose, and a test it must delete is a scaffold step that can be missed. The rule is enforced by review. |

## Task entries

### Task 1 — plan/template-version

Landed: this record and its row in `memory-index.md`. Nothing outside `.agents/` changes.
