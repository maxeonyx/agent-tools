---
title: Auto-Update Mechanism
type: auto
applies_to: all
checker: "depends on agent-tools-updater crate AND calls update check at startup"
---

Users install these tools once and forget. Without auto-update, they run stale versions indefinitely.

The mechanism: at startup, check if a previously-downloaded update exists. If yes, replace self. In background, check GitHub Releases for a newer version. If found, download it. Next invocation picks it up.

No user interaction. No prompts. Silent and correct. If the update check fails (offline, rate-limited), the tool works normally — update is best-effort, never blocking.

All these tools are public with GitHub Releases. The infrastructure exists. The missing piece is the client-side code, which lives in the shared `agent-tools-updater` crate.

## Not yet implemented

This concern is aspirational — the `agent-tools-updater` crate is a skeleton. Implementing it is a single piece of work that benefits all tools simultaneously. Once the crate works, bringing each tool into compliance is trivial (add dependency, call at startup).
