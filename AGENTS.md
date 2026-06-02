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

Update this file, the standards, or the compliance checks IMMEDIATELY when you notice a gap. Process fixes are high leverage, and the entire point of this project. They compound. Implementation fixes are local. They are needed but don't compound.

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
2. Define compliance as an objective predicate where possible.
3. Add a Rust concern module in `crates/standards/src/concerns/<concern>.rs` with the concern definition in the module docs, any review instructions it exports, and the checker in a `#[cfg(test)]` module.
4. Keep `crates/standards/src/concerns/mod.rs` in sync so the concern registry and agentic concern list match the modules.
5. Land the enforcement. Red tests are the visible work queue.
6. Bring tools into compliance one by one via the generalize loop.

The concern is not real until enforcement exists. Prose in AGENTS.md is not enforcement.

### Bringing a tool into compliance

Two tracks run in parallel: mechanical fixes and agentic review.

**Mechanical fixes** (workspace-routing, tdd-ratchet, black-box-tests, etc.):
Pick one concern, fix it, run the test, see it pass. Simple loop.

**Agentic review pass** (code-review, error-messages, help-text, injectable-io):
These are batched per tool — one review pass covers ALL applicable agentic concerns.

Review loop:
1. Pick a tool. Derive applicable agentic concerns from NOT_APPLICABLE lists.
2. Fresh review session: read source, run the tool, trigger errors, read help.
   Evaluate against ALL applicable REVIEW_INSTRUCTIONS. Produce findings by concern.
3. Shape findings into an implementation backlog (group by design seam, not concern).
4. Implement in stages. Each stage verified before the next.
5. Fresh re-review of the whole tool (not just what you fixed).
6. If findings remain → back to step 3.
7. Exit: zero findings. Write attestation files, tests go green.

Rules:
- Review and implementation are different agent sessions (independence).
- injectable-io applicability is precomputed from NOT_APPLICABLE, not debated.
- Mechanical failures can join the implementation backlog if the same change fixes them.
- Attestation files: `docs/reviews/<concern>.json` with `reviewed_commit` field.

Tool order: trunc → tdd-ratchet → dotsync → tb → oc (simplest first).

### Adding a new tool

1. **Improve process first.** Does the onboarding process need updating?
2. Create the tool repo (follow existing patterns — MIT license, AGENTS.md, docs/, .github/workflows/)
3. Add it as a submodule under `tools/`
4. Add it to `standards::TOOLS`
5. Run `cargo test -p standards` and use the failing concern tests as the compliance backlog
6. Bring it into compliance concern by concern via the generalize loop
7. Update the umbrella site (`docs/index.html`) and cross-references in sibling tools

---

## Enforcement

A concern is not enforced until two things exist:

1. **Definition** — what compliance means, precisely
2. **Checker** — a Rust test that can verify compliance mechanically

Without both, it's aspiration. Aspiration does not prevent drift.

### Current standards

Defined in `crates/standards/src/concerns/*.rs`.

Run the standards suite:
```bash
cargo test -p standards
```

Passing tests are the compliance state. Failing tests are the TODO list. `crates/standards/src/concerns/mod.rs` tracks the concern registry and which concerns are agentic.

---

## Commands

```bash
# Fast checks (lint, format, build, tests — immediate feedback)
cargo test -p standards              # standards fail until every applicable tool complies
cargo fmt --check --all              # formatting
cargo clippy --all -- -D warnings    # linting
cargo test -p trunc                  # tool tests (fast — spawns binary, checks output)

# Slow checks (black-box tests — spawn binaries, real filesystem)
cargo test --test '*' -p trunc       # example: trunc integration tests

# Per-tool verification (TDD ratchet)
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
| Cross-cutting standards definitions and enforcement | `crates/standards/src/concerns/*.rs` |
| Concern registry / agentic concern visibility | `crates/standards/src/concerns/mod.rs` |
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
