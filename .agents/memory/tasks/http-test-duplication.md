---
name: memory-tasks-http-test-duplication
description: Removing the duplicated blocks Sonar flagged in test/http.test.js (6.7% duplicated lines on new code) without changing what the suite covers; released as 0.2.2.
---

# HTTP test duplication — template

## 2026-11-11 — complete

**Goal.** Sonar reports 6.7% duplicated lines on new code in `test/http.test.js`. The owner asked
to check whether that is real and, if it is, to remove it. The file is the one a scaffolded
project edits first (`PROMPT.md` step 2e), so a copy-pasted block here is copied again downstream.

**Objective.** The duplicated blocks are replaced by small helpers in the same file. `npm test`
runs the same 37 tests, all passing, with the same names, skips and assertions. `jscpd` finds no
clone in the file at its default thresholds.

**Status:** complete. Released as `0.2.2`; the pull requests, merged in order by rebase, are in the table.

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
| 1 | Task record | `plan/http-test-duplication` | [#16](https://github.com/LXAgents-MCP/template/pull/16) |
| 2 | Remove duplication from the HTTP tests | `test/http-test-duplication` | [#17](https://github.com/LXAgents-MCP/template/pull/17) |
| 3 | Release | `release/0.2.2` | [#18](https://github.com/LXAgents-MCP/template/pull/18) |

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

### Task 2 — test/http-test-duplication

Landed. `test/http.test.js` gains eight helpers, and each duplicated block now calls one:

| Helper | Replaces |
|---|---|
| `NEEDS_SIGNALS` | The Windows `skip` option written out on four signal tests. |
| `waitUntil(check, ms)` | The two polling loops; `waitForOutput` and `waitForCount` are one line each. |
| `exitOf(child)` | Four hand-built exit promises, `startServer`'s included. |
| `interrupt(child, times)` | Send `SIGINT` and await the exit, in the three drain tests. |
| `withCluster(run)` | Start two workers and assert the fork, in five tests. The orphan test now uses it too, so its primary is killed by `withServer`'s `finally` as before. |
| `withClient(url, run)` | Connect, run, close in `finally`, in three tests. |
| `assertIsolatedSums(url, sums, message)` | The 13-line concurrent `calculate_sum` block in the single-process and cluster tests. Each keeps its own sums and its own failure message. |
| `postMcp(url, body)` | The raw `POST /mcp` in the oversized and malformed body tests. |

Checked: `npm test` runs 37 passing and none failing, the same as before. The 37 test names are
identical. Every assertion still runs in every test that ran it; the count in the file drops from
75 to 70 only because the fork check and the sums check are now written once. `jscpd` on the file
finds 0 clones at its defaults and at `--min-lines 5 --min-tokens 50` (was 3 clones, 26 lines).
Nothing under `src/` changed.

### Task 3 — release/0.2.2

Landed. `package.json` and the lockfile are at `0.2.2`, the release log is
`wiki/logs/0/2/2/CHANGELOG.md`, and the logs index leads with its row. This commit fills the `PR`
column and closes the record. No tag is made and nothing is published.
