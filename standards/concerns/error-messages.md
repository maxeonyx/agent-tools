---
title: Error Message Standards
type: agentic
applies_to: all
checker: "agent review: trigger error paths, verify messages include intent + actual state + remediation"
---

Every error message answers three questions: what was attempted, what actually happened, what to do about it.

Agents are the primary user of these tools. An agent that gets `"Error: invalid input"` will thrash — trying random variations, reading source code, asking the user. An agent that gets `"Error: expected a tmux session name matching /^[a-z0-9-]+$/, got 'My Session' — use lowercase with hyphens"` fixes it immediately.

This isn't about being verbose. It's about being actionable. Sometimes the remediation is one word ("use lowercase"). Sometimes it's a command to run. But it must always be there.

## The test

For each error path: trigger it. Read the message. Can you fix the problem from the message alone, without reading source code? If not, the message is wrong.

## Implementation

- `thiserror` for structured error types with context
- No bare `.unwrap()` on user-facing paths
- Error chains preserve context: "failed to read config at ~/.config/oc/oc.db: permission denied — check file permissions"
- Parse errors include what was expected and what was found
- File errors include the path
- Network errors include the URL and suggest checking connectivity
