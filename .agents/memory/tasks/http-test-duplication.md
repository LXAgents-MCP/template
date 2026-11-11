---
name: memory-tasks-http-test-duplication
description: Removing the duplicated blocks Sonar flagged in test/http.test.js (6.7% duplicated lines on new code) without changing what the suite covers.
---

# HTTP test duplication — template

## 2026-11-11 — in progress

**Goal.** Sonar reports 6.7% duplicated lines on new code in `test/http.test.js`. The owner asked
to check whether that is real and, if it is, to remove it. The file is the one a scaffolded
project edits first (`PROMPT.md` step 2e), so a copy-pasted block here is copied again downstream.

**Objective.** The duplicated blocks are replaced by small helpers in the same file. `npm test`
runs the same 37 tests, all passing, with the same names, skips and assertions. `jscpd` finds no
clone in the file at its default thresholds.

**Detail.** Test-only: nothing under `src/` changes, so no client sees a difference. No test is
removed, renamed, merged or weakened. The helpers stay in `test/http.test.js`: it is the only file
that starts a real server, so a shared module would have one user.

**Is it real?** Yes. Checked with `jscpd` (`--min-lines 5 --min-tokens 50`) on `0605a30`:

| Clone | Lines | Where |
|---|---|---|
| Three concurrent `calculate_sum` calls, each checked against its own arguments | 13 | `concurrent requests do not share state`, `concurrent tool calls stay isolated…` |
| Waiting for the child's exit, then asserting code 0 | 6 | the single-process drain and the cluster drain |
| `withServer` with two workers, waiting for the fork, then the exit promise | 7 | the cluster drain and the second-signal test |

Also near-identical, below the tool's token threshold but the same shape: the polling loops in
`waitForOutput` and `waitForCount`, the `{ skip: … "no signal delivery on Windows" }` option on
four tests, and the connect / try / close pattern around an MCP client.

## Tasks

| # | Title | Branch | PR |
|---|---|---|---|
| 1 | Task record | `plan/http-test-duplication` | |
| 2 | Remove duplication from the HTTP tests | `test/http-test-duplication` | |
| 3 | Release | `release/0.2.2` | |

Branches stacked: task 1 from `master`, task `k` from task `k-1`. Only this repository changes.

## Decisions

| # | Decision |
|---|---|
| D1 | No documentation task: no page names the helpers being changed, and the test count does not move. |
| D2 | The release is `0.2.2`, a patch. The shipped package (`src`, `README.md`) is byte-identical; the project instructions require a release branch for every task. The owner is away and asked for the recommended option. No tag, nothing published. |

## Task entries

### Task 1 — plan/http-test-duplication

Landed: this record and its row in `memory-index.md`. Nothing outside `.agents/` changes. The `PR`
column is filled by the release task.
