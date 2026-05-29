# Vision and Process Docs

Each tool has VISION.md and proper AGENTS.md.

## Why

A tool without a vision statement drifts. Agents (and humans) make feature decisions without knowing what the tool IS — they add things that don't fit, or miss things that do. VISION.md is the anchor: what problem does this tool solve, for whom, and where is it going?

AGENTS.md is the process: how to develop this tool, what the architecture is, how tests work, what to watch out for. Without it, every agent starts from zero, re-discovers the same things, and makes the same mistakes.

## What compliance looks like

**VISION.md:**
- Exists
- States what the tool is in one sentence
- States who it's for and what problem it solves
- States design principles (what to optimize for, what to avoid)
- States direction (what's next, what's explicitly out of scope)

**AGENTS.md:**
- Exists
- Routes development to the agent-tools workspace (see workspace-routing concern)
- Describes the architecture (how the code is organized, key abstractions)
- Describes the test strategy (what kinds of tests exist, how to run them)
- Documents non-obvious decisions and constraints

## How to bring a tool into compliance

1. Write VISION.md: what is this tool? Why does it exist? What are its principles? Where is it going?
2. Review AGENTS.md: does it actually help a new agent work on this tool? Does it explain the architecture? The test patterns?
3. Remove stale information. Update anything that's wrong.

## How to maintain

When making significant changes to a tool's direction or architecture, update VISION.md and AGENTS.md in the same commit. These documents should always reflect current reality, not historical aspirations.
