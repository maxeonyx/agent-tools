# Agent guidance

A fresh agent entering a repository can do good, safe work from its AGENTS.md, and nothing there is false.

## User stories

- **A fresh agent's first task in a repo:** it follows the commands and rules it is given. Stale or damaging ones cost real damage, like the old tb cleanup command that killed live sessions.
- **Max:** the guidance should carry what the repository cannot say for itself, and not much else.

## What it looks like here

- Every repository: `AGENTS.md`, and a `CLAUDE.md` containing exactly `@AGENTS.md`.
- The umbrella's `AGENTS.md` holds cross-tool process; each tool's holds its own facts.
- Markdown prose is not hard-wrapped (Max's rule).

## How to look

- CLAUDE.md alias: `find . -name AGENTS.md -not -path '*/target/*' -not -path '*/fixtures/*' -execdir sh -c 'grep -qx "@AGENTS.md" CLAUDE.md 2>/dev/null || pwd' \;`
- Documented commands: for each repo, pick the commands in its AGENTS.md and check that they exist (`--help`, `devenv` tasks, scripts). Do not run anything destructive.
- Judgment: for one repo per refresh, read AGENTS.md as a fresh agent. What is false, what is missing, and which paragraphs only work around something that could be fixed?
