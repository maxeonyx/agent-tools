# agent-tools - Agent Instructions

This repository is a lightweight umbrella for Max's AI agent CLI tools.

## Start here

- Read `README.md` for the human-facing project pointer.
- Read `docs/SKILL.md` when creating a new tool that should follow this pattern.
- Read `docs/index.html` when updating the landing page and tool catalog.

## Scope

- Keep this repo documentation-only.
- Do not add Rust or application code here.
- Keep cross-tool links and install snippets current.

## Required website content

- `docs/index.html` lists trunc, tmux-bridge, dotsync, tdd-ratchet, and oc.
- Each tool entry includes description, binary name, repo link, site link, and install command.
- `docs/CNAME` must contain `tools.maxeonyx.com`.

## Deployment

- Pages deploys from `docs/` via `.github/workflows/pages.yml`.
- Use Actions-based Pages deployment.

## Skill synchronization

- `docs/SKILL.md` is the canonical source for the tool-creation pattern.
- Keep `~/.config/opencode/skills/agent-tools/SKILL.md` in sync with this file.
