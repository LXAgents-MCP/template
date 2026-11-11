# 0.2.2

**Released:** 2026-11-11

The HTTP test suite loses its copy-pasted blocks. Nothing a caller sees changes, and nothing in the
published package changes: `src/` and `README.md` are byte-identical to `0.2.1`. A project scaffolded
from this release starts with a shorter `test/http.test.js` whose helpers it can reuse for its own
tools.

## Changed

- `test/http.test.js` — Sonar reported 6.7% duplicated lines on new code. The repeated blocks are now
  helpers in the same file: `waitUntil` (one polling loop behind `waitForOutput` and `waitForCount`),
  `exitOf` and `interrupt` (waiting for the child to exit, and sending `SIGINT` first), `withCluster`
  (two workers, fork asserted), `withClient` (connect, run, close), `assertIsolatedSums` (concurrent
  `calculate_sum` calls each checked against their own arguments), `postMcp` (a raw `POST /mcp`), and
  `NEEDS_SIGNALS` (the Windows skip). Every test keeps its name, skip, inputs, failure messages and
  assertions.

## Verified

`npm test`: 37 passing, none failing, the same as before. `jscpd` finds no clone in
`test/http.test.js` (was 3 clones, 26 lines).

What a reader has to do: nothing.
