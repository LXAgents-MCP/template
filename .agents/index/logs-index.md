---
name: logs-index
description: Release history of template, newest version first - what changed in each version and where its changelog lives.
---

# Logs Index

**Scope:** `wiki/logs/`
**Parent:** [`project-wiki-index.md`](project-wiki-index.md)

## Versions

| Version | Changelog | Summary |
|---|---|---|
| `0.2.2` | [`../../wiki/logs/0/2/2/CHANGELOG.md`](../../wiki/logs/0/2/2/CHANGELOG.md) | Duplicated blocks in `test/http.test.js` replaced by helpers; same 37 tests. Nothing in the package changes. |
| `0.2.1` | [`../../wiki/logs/0/2/1/CHANGELOG.md`](../../wiki/logs/0/2/1/CHANGELOG.md) | `src/` split by role, one job per file, in the sibling servers' folders; `src/index.js` split into settings, worker and primary. Nothing a caller sees changes. |
| `0.2.0` | [`../../wiki/logs/0/2/0/CHANGELOG.md`](../../wiki/logs/0/2/0/CHANGELOG.md) | Express transport at `POST /mcp`, cluster workers on one `PORT`, and the `MCP_ALLOWED_HOSTS` guard and `HOST` support. |
| `0.1.0` | [`../../wiki/logs/0/1/0/CHANGELOG.md`](../../wiki/logs/0/1/0/CHANGELOG.md) | Per-file tool layer with zod schemas and an optional unified API key; agent instruction system adopted. |

## Maintenance

A new version directory is added here **in the same commit** that creates it. Newest
version first. Version directories are `wiki/logs/{Major}/{Minor}/{Patch}/`, and a
version is never bumped without explicit user approval — see
`agents://rules/versioning.md`.
