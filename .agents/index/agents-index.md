---
name: agents-index
description: Index of this repository's own instruction files - the repository rules and template mode.
---

# Agents Index

**Scope:** `.agents/rules/`
**Parent:** [`root-index.md`](root-index.md)

## Rules

| File | Purpose |
|---|---|
| [`../rules/repository.md`](../rules/repository.md) | The dual-surface contract, the stdout ban, where things go, and what must not be introduced. |
| [`../rules/template-mode.md`](../rules/template-mode.md) | Rules that apply only while `PROMPT.md` exists and everything here is inherited by scaffolded projects. |

This repository holds no `git/`, `planning/`, `prompts/`, or `creators/` folder. Those
are served by the `lxagents-agents-base` connector — route to
`agents://index/root-index.md` for them.

## Maintenance

Any file added to or removed from `.agents/rules/` is reflected here **in the same
commit**.
