---
name: memory-tasks-template-version
description: Pinning the template's own version at 0.0.0 for good, because every project scaffolded from it starts from this package.json; the version a project ships is set by PROMPT.md at scaffold time.
---

# Template version — template

## 2026-11-11 — complete

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

**Status:** complete. The pull requests, merged in order by rebase, are in the table.

**Detail.** Released logs for `0.1.0`–`0.2.2` are history and are not rewritten or deleted
(the shared changelog rule forbids it). No tag exists for any of them, and none is made.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/template-version` | [#19](https://github.com/LXAgents-MCP/template/pull/19) |
| 2 | Pin the version at 0.0.0 and write the rule | `chore/template-version` | [#20](https://github.com/LXAgents-MCP/template/pull/20) |
| 3 | Release | `release/0.0.0` | [#21](https://github.com/LXAgents-MCP/template/pull/21) |

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

### Task 2 — chore/template-version

Landed.

- `package.json` and both `package-lock.json` entries: `0.2.2` → `0.0.0`. `template --version` prints
  `0.0.0`.
- `.agents/rules/template-mode.md`: new section *The version is always 0.0.0* (the rule, why, the
  `release/0.0.0` + rolling-log convention, no tags, old logs left as history); description updated,
  and its row in `agents-index.md`.
- `AGENTS.md` *Version rule*: one sentence linking the template-mode rule.
- `PROMPT.md`: Step 1 item 8 says the template has no version to carry over; Step 3 item 1 says to set
  the Initial Version in the lockfile too; Step 3 item 7 updates the version in the sample output and
  resets the release history (delete `wiki/logs/` version directories and the logs-index rows, create
  the Initial Version's log).
- `wiki/environments/setup.md` and `env.md`: sample output showed `0.1.0`, now `0.0.0`.
- `.agents/memory/state/repository-state.md`: said `0.1.0` and `src/tools/`; now `0.0.0` and
  `src/controllers/`.

Checked: `npm test` 37 passing, none failing. Nothing under `src/` changed.

### Task 3 — release/0.0.0

Landed. No version change. `wiki/logs/0/0/0/CHANGELOG.md` starts the template's rolling log with this
change's section, and the logs index carries its row at the top. This commit fills the `PR` column
and closes the record. No tag.
