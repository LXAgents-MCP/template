# 0.0.0 — the template's rolling log

The template's version is always `0.0.0` (see `.agents/rules/template-mode.md`). It has no releases
of its own, so every change to it is logged here, newest first, one dated section each. The logs for
`0.1.0` to `0.2.2` are history from before the version was pinned.

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
