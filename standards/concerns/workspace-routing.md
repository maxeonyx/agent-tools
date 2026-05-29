---
title: Develop from workspace
type: auto
applies_to: all
checker: "tool AGENTS.md contains canonical workspace-routing directive"
---

All development happens from the `agent-tools` workspace, not from individual tool clones.

This is the foundation concern. When agents clone individual repos, they lose visibility into sibling tools and shared standards. They make decisions that diverge from the ecosystem. The workspace gives them all tools at once, shared enforcement, and the compliance matrix.

Nothing else in this standards system works if agents develop outside the workspace.

## Compliance

The tool's `AGENTS.md` has a clear, prominent directive — one of the first things an agent reads — telling it to develop from `maxeonyx/agent-tools`.
