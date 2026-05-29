# agent-tools — Development Workspace

This is the control plane for developing the maxeonyx agent-tool suite. All development happens from this workspace. Individual tool repos exist for CI, releases, and Pages — not for development.

## The goal

Every tool in this suite should benefit from every improvement made to any tool. When you add auto-update to one tool, all tools get it. When you improve help text patterns, all tools get it. When you fix a CI problem, all tools get the fix. The workspace enforces this by making cross-cutting work the natural path and tool-specific work the exception.

## IMPROVE PROCESS FIRST

**Before doing ANY work — before investigating, before designing, before implementing — ask: does the process need to change?**

Agents ignore process. They barrel past it into implementation. This rule exists because of that. The process is the first job. Not the second job. Not "also important." THE FIRST JOB.

If a mistake happened, the process should have prevented it. Fix the process.
If a step was confusing, the process should have been clearer. Fix the process.
If something was skipped, the process should have enforced it. Fix the process.
If you're about to do work and the process doesn't describe how, STOP. Write the process first. Then follow it.

Update this file, the standards, or the compliance checks IMMEDIATELY when you notice a gap. Process fixes are high leverage. They compound. Implementation fixes are local. They don't.

---

## Development loops

All work follows loops. Not phases. Loops have exit conditions and go-back paths.

### Investigate loop

Understand the problem or goal before acting.

1. Reproduce. See the actual behavior. Run the thing. Observe what actually happens — not what you think happens from reading code.
2. Form a hypothesis about why
3. Design a test that distinguishes your hypothesis from alternatives
4. Run the test. Does it confirm or refute?
5. If refuted → new hypothesis → go to step 3
6. Exit: you can state the problem precisely, you've seen it with your own eyes, and your explanation predicts the observed behavior

### Design loop

Decide what to build before building it.

1. Sketch an approach
2. Trace its implications across all affected tools/concerns
3. Check: does it handle all known cases? Is it the simplest approach that works? Would you choose it fresh?
4. If no → revise or start over
5. Exit: you'd choose this design again if starting from scratch

### Test loop

Prove the requirement before satisfying it.

1. Write a test (or define a verification method) that captures the requirement
2. Run it — it must fail (if it passes, your test doesn't capture anything new)
3. Check: does the test failure clearly describe what's missing?
4. If no → fix the test
5. Exit: you have a failing test that will pass when and only when the requirement is met

### Implement loop

Satisfy the test.

1. Write the minimum code to make the test pass
2. Run the test
3. Check: does it pass?
4. If no → fix the implementation
5. Exit: test passes

### Review loop

Challenge what you built.

1. Read the code fresh — is this the design you'd choose if starting over?
2. Check for: unnecessary complexity, missing error handling, unclear names, untested paths
3. Check: would you approve this if someone else wrote it?
4. If no → go back to design or implement loop
5. Exit: you'd write it the same way from scratch

### Generalize loop

When you've done something for one tool, do it for all of them.

1. Look at what you just did for one tool
2. Identify what's tool-specific vs what's a pattern all tools should follow
3. If it's a pattern: is there enforcement that all tools must follow it?
4. If no enforcement exists → add enforcement first (update standards, add a check)
5. Apply the pattern to the next tool
6. Repeat until all tools comply

Exit: all tools have the improvement, and enforcement prevents regression.

---

## Workflows

### Maintaining one tool

1. **Improve process first.** Does this task reveal a process gap? Fix the process.
2. Work in `tools/<name>/` within this workspace
3. Follow the loops: investigate → design → test → implement → review
4. After review: does this change represent a pattern other tools should follow? If yes → generalize loop
5. Commit and push the tool repo, then update the submodule pointer here

### Adding a new cross-cutting concern

1. **Improve process first.** Write down what the concern IS and WHY it matters.
2. Define compliance as an objective predicate (a checker can verify it mechanically)
3. Write the checker. Add it to the workspace enforcement.
4. Add every tool to the compliance matrix — mark current state honestly (most will be non-compliant)
5. Land the enforcement. Now it's visible.
6. Bring tools into compliance one by one via the generalize loop.

The concern is not real until enforcement exists. Prose in AGENTS.md is not enforcement.

### Bringing a tool into compliance

1. **Improve process first.** Is the concern definition clear enough to implement against?
2. Pick ONE concern and ONE tool
3. Follow the loops: investigate (what's the gap?) → design (what's the minimal change?) → test → implement → review
4. Run the workspace compliance checker — does it pass for this tool + concern?
5. If no → iterate
6. If yes → commit, push tool, update submodule, re-run checker from workspace root
7. Exit: the tool is compliant and the checker proves it

### Adding a new tool

1. **Improve process first.** Does the onboarding process need updating?
2. Create the tool repo (follow existing patterns — MIT license, AGENTS.md, docs/, .github/workflows/)
3. Add it as a submodule under `tools/`
4. Add it to the compliance matrix for every existing concern (it will start non-compliant for most)
5. Bring it into compliance concern by concern via the generalize loop
6. Update the umbrella site (`docs/index.html`) and cross-references in sibling tools

---

## Enforcement

A concern is not enforced until three things exist:

1. **Definition** — what compliance means, precisely
2. **Checker** — a script that can verify compliance mechanically
3. **Status for every tool** — compliant, waived, or non-compliant

Without all three, it's aspiration. Aspiration does not prevent drift.

### Current standards

Defined in `standards/concerns.toml`. Compliance state in `standards/compliance.toml`.

Run the checker:
```bash
python3 scripts/check-compliance.py
```

### Compliance statuses

- **compliant** — meets the standard, verified by checker
- **waived** — explicitly excused, with documented reason and exit condition
- **non-compliant** — does not meet the standard, work needed
- **not-applicable** — standard does not apply to this tool

`unknown` is not a valid status. Every tool must have a status for every concern.

---

## Commands

```bash
# Workspace Rust checks
cargo fmt --check --all
cargo clippy --all -- -D warnings

# Compliance
python3 scripts/check-compliance.py

# Per-tool verification (from workspace root)
cd tools/<name> && cargo ratchet
```

---

## Submodule workflow

1. Make changes in `tools/<name>/`
2. Commit and push to the tool's own repo/branch
3. From workspace root: `git add tools/<name>` to update the submodule pointer
4. Commit and push the workspace

---

## What belongs where

| Content | Location |
|---------|----------|
| Development process, loops, discipline | This file |
| Cross-cutting standards definitions | `standards/concerns.toml` |
| Compliance state per tool | `standards/compliance.toml` |
| Compliance checker | `scripts/check-compliance.py` |
| Shared Rust libraries | `crates/` |
| Tool-specific product/architecture facts | `tools/<name>/AGENTS.md` |
| Tool CI, releases, Pages | Tool's own repo |
| Umbrella site | `docs/` |

---

## Git identity

Personal repo. Use:
```
user.name = Maxwell Clarke
user.email = maxeonyx@gmail.com
```

Pushing to main is safe — remote preservation. Commit and push frequently.
