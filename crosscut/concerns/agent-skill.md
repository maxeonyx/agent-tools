# Agent skill

An agent can learn to use each tool from a skill: when to reach for it, how, and its gotchas.

## User stories

- **An agent mid-task:** it should know the tool exists and how to use it well, without Max explaining.
- **Max on a new machine:** installing the skills should be part of installing the tools.

## What it looks like here

- Each tool: `docs/SKILL.md`, published on its site, with frontmatter naming when to use it.
- crosscut: a multi-file skill in `skill/`, installed by `crosscut install-skill`.

## How to look

- `ls tools/*/docs/SKILL.md tools/*/skill/SKILL.md 2>/dev/null`, and `curl -sI https://<site>/SKILL.md`.
- How does the skill reach a machine? Look for install instructions, or dotsync-managed skill directories.
- Judgment: read each description. Would it trigger at the right moments, and would the body let an agent succeed?
