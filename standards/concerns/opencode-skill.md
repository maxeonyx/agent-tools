# OpenCode Skill Published

Each tool is usable by agents via its OpenCode skill.

## Why

These tools are built for agents. An agent that can load a skill for `trunc` or `tmux-bridge` gets usage patterns, gotchas, and examples injected into its context at the right moment. Without a skill, agents have to figure out the tool from `--help` alone or ask the user.

Skills make tools discoverable and usable without the user explaining them every time.

## What compliance looks like

- `docs/SKILL.md` exists in the tool repo
- Frontmatter has `name` matching the tool
- Frontmatter `description` is trigger-focused ("WHEN to load this skill" not "what it contains")
- Content is actionable: common commands, examples, gotchas, installation
- Content does NOT duplicate AGENTS.md (skills are for users of the tool, AGENTS.md is for developers)

## How to bring a tool into compliance

1. Write `docs/SKILL.md` with proper frontmatter
2. Focus on: when would an agent need this? What would it need to know?
3. Include concrete commands and examples
4. Install locally: `cp docs/SKILL.md ~/.config/opencode/skills/<tool>/SKILL.md`
5. Test: does an agent correctly identify when to load this skill?

## How to maintain

When adding new features or changing behavior, update the skill. When the skill's trigger conditions change, update the frontmatter description. The skill should always match the current release, not a future version.
