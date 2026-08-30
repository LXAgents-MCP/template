---
name: template-mode
description: Rules that apply only while this repository is still a template - PROMPT.md is the scaffolding authority and the template names are placeholders.
---

# Template Mode

This repository is a template until `PROMPT.md` is deleted. While that file exists,
these rules apply on top of everything else.

## PROMPT.md is the scaffolding authority

A request to initialize, scaffold, or set up a new project from this template is
answered by reading [`../../PROMPT.md`](../../PROMPT.md) and following it, not by
improvising an equivalent. Its steps are ordered and its Step 3 is exhaustive.

Changing what a scaffolded project starts with means editing `PROMPT.md`. A change to
the template's own structure that Step 3 does not know about produces projects with
leftovers.

## Names in the template are placeholders

`template`, `Template`, and `@mcagents-mcp/template` are scaffolding targets, not
names to preserve. They appear in `package.json`, `src/server.js`, `src/cli.js`,
`README.md`, and `wiki/environments/setup.md`, and Step 3 of `PROMPT.md` replaces each
one. A new occurrence added anywhere else needs a matching line in Step 3.

## Anything shipped here is inherited

Every file in this repository becomes the starting point of some other project. A
convenience added for the template's own sake is a convenience every scaffolded project
carries whether it wants it or not.

Before adding anything: either it is worth inheriting, or `PROMPT.md` Step 3 removes it.
There is no third option, and a file that is neither is how a template accumulates
things nobody chose.

## Leaving template mode

Scaffolding removes `PROMPT.md`, the "Project Scaffolding (Template Mode)" section of
`AGENTS.md`, and that section's trigger row. This file goes with them - a scaffolded
project is not a template, and a rule about template mode left behind is a rule that
can only mislead.
