# agent-tools — Development Workspace

This is the control plane for coordinating the maxeonyx agent-tool suite. Cross-cutting work is easiest here, while every child repository remains a complete standalone development, CI, and release context in its own right.

## The goal

Every tool in this suite should benefit from every improvement made to any tool. When you add auto-update to one tool, all tools get it. When you improve help text patterns, all tools get it. When you fix a CI problem, all tools get the fix. The workspace enforces this by making cross-cutting work the natural path and tool-specific work the exception.

## Cross-cutting concerns: CrossCut

This umbrella is an application of [CrossCut](tools/crosscut/skill/SKILL.md). The questions worth asking again across the suite live in `crosscut/concerns/`, one Markdown file each: the question, why it matters here, how to look, and the latest dated view. `crosscut list` shows them; `crosscut refresh` updates the views headless. To add, change or delete a concern, use the CrossCut skill (`establish` and `reconsider` modes). Read its doctrine before touching `crosscut/`.

Concerns are visibility and food for thought, not gates: nothing here ratchets, scores or attests them, and a view that finds bad news has done its job. Anything that passes this on (this file, a delegated agent's instructions, a rewrite) must carry the CrossCut doctrine forward together with its requirement to carry both the doctrine and that requirement forward again.

When you finish a piece of work here, spend a moment on CrossCut's `generalize` question: does this reveal something about the other tools, or a concern nobody is watching? Usually the answer is no; say so and move on.

## Concurrent development

Mutable work belongs in an exclusive full clone under the sibling `agent-tools-workspace` repository. Name clones `at-<feature-branch>` directly under that repository and use slash-free feature branch names. Never edit, stash, reset, or commit another session's clone.

After creating a clone, run every branch, push, and PR command with the new clone as the explicit working directory. A shell stays in the parent directory after `git clone`; chaining `git switch` or `git push` without changing directories can mutate the workspace-wrapper repository instead of the new clone.

Initialize only the tool and library submodules the task needs, and run component commands from their own repository directory. The umbrella has no Cargo workspace of its own.

When child and umbrella branches both move a submodule pointer, never resolve the gitlink conflict by choosing one side. Merge the child histories first and pin their common descendant. Tool commits and PRs land before the umbrella pointer PR. Prefer merge commits; do not rebase or force-push by default. Rewrite history only when it is genuinely unusable; if replacement is necessary, manually rebuild the branch and explicitly swap it rather than using a brittle routine rebase workflow.

## IMPROVE PROCESS FIRST

**Before doing ANY work — before investigating, before designing, before implementing — ask: does the process need to change?**

Agents ignore process. They barrel past it into implementation. This rule exists because of that. The process is the first job. Not the second job. Not "also important." THE FIRST JOB.

If a mistake happened, the process should have prevented it. Fix the process. If a step was confusing, the process should have been clearer. Fix the process. If something was skipped, the process should have enforced it. Fix the process. If you're about to do work and the process doesn't describe how, STOP. Write the process first. Then follow it.

Update this file, or the relevant CrossCut concern, IMMEDIATELY when you notice a gap. Process fixes are high leverage, and the entire point of this project. They compound. Implementation fixes are local. They are needed but don't compound.

---

## LEAVING TESTS RED IS A SUPERPOWER

Red tests and red CI are **expected and good** here. They are the honest, visible record of where things sit. Red is a resting state, not a debt. Do not be uncomfortable with it.

The wrong instinct — the one to fight — is making red go green by papering over it: grandfathering a failing test, adding a carve-out, moving a ratchet baseline to swallow a violation, or marking something passing that isn't. That hides the work and corrupts the signal. **Never make red green except by genuinely doing the work.**

**You do not need to make anything green. Prefer to defer over fixing hackily: green is earned the proper way only, there is time, and it is fine to leave something red and say why.** If a test should fail, let it fail loudly. If you fix a violation, fix it the real way (e.g. rewrite history so a test genuinely goes `pending` → `passing`), never by relaxing the gate. Honest red beats fake green every time.

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

**For ratcheted tools, never grandfather a new passing test. If a new test would already pass, introduce the test in the same commit as a deliberately corrupted implementation that proves the test fails, commit that red state, then fix the implementation in the subsequent commit.**

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
3. If it's a pattern: can it be made structural (a shared library, one reusable workflow) so tools cannot drift? If not, and it is worth re-asking, is there a CrossCut concern that keeps it visible?
4. Apply the pattern to the next tool
5. Repeat until every tool has it, or until the remaining ones need work you are deferring

Exit: every tool has the improvement, or the ones that don't are named in the relevant concern's view with the reason.

---

## Workflows

### Maintaining one tool

1. **Improve process first.** Does this task reveal a process gap? Fix the process.
2. Work in `tools/<name>/` within this workspace
3. Follow the loops: investigate → design → test → implement → review
4. After review: does this change represent a pattern other tools should follow? If yes → generalize loop
5. Run the child repository's `devenv test`; its actionlint check is the local proof that GitHub can parse the workflow. Commit and push the tool branch, open a PR, and merge current child `main` into that branch.
   - **Run `git status --short` after every commit** and confirm nothing you meant to commit is left behind. A 2026-08-12 dotsync session corrupted three commits' worth of TDD history because `git add -u <pathspec>` _restricts_ staging to that pathspec (it does not mean "everything tracked plus this") — the ratchet flips landed in commits that didn't contain their work, and only a reviewer's history audit caught it. If commit messages describe work, the diff must contain that work.
6. Each tool's own TDD ratchet and trusted ledger are described in its AGENTS.md. A push to a tool PR triggers that tool's ledger workflow, which may add a bot commit to the branch: wait for it, pull it, and only then dispatch integration, or `Ready` lands on a commit that is no longer the head. Integration also refuses a version that is already released, so bump `package.version` (and `docs/version.json`) in any PR, even a docs-only one.
7. Explicitly dispatch the source workflow with `gh workflow run ci.yml --ref <feature-branch> -f pr_number=<number>`. It serializes integration, records the required Ready check, auto-merges, publishes the artifacts it already built, and records `integrated-ci` on the exact merge commit. It merges with `--delete-branch`, and GitHub closes any pull request whose base branch is deleted, so retarget every `gh pr list --base <feature-branch>` to `main` before dispatching.
8. **Observe the whole dispatched run and reconcile it against your intent — do NOT chase green.** The bar is not "CI is green"; the bar is "CI is in the state I intended, and I understand every red." Red CI is often the correct desired state. Confirm the PR merge, release, Pages deployment, and exact-commit `integrated-ci` status before updating the umbrella pointer. Never relax a gate merely to turn a red green.

Standard release targets are `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc` only. Do not add musl, macOS, or aarch64 release targets unless the user explicitly reopens that support. `oc` is intentionally Linux-only for now.

### Adding a new tool

1. **Improve process first.** Does the onboarding process need updating?
2. If the tool starts from external design/process notes, import those notes into the tool repo as source material and create a process handoff before product implementation. The handoff records the active loop, first verification target, user-gated decisions, and what is explicitly experimental.
3. Create the tool repo (follow existing patterns — MIT license, AGENTS.md, docs/, .github/workflows/)
4. Add it as a submodule under `tools/`
5. Add it to `TOOLS` in `scripts/generate-version-json.py` once it publishes a `docs/version.json`
6. Refresh the concerns in `crosscut/concerns/` that should now cover it (`crosscut refresh`), and note in each view what the new tool changes
7. Update the umbrella site (`docs/index.html`) and cross-references in sibling tools

### Archiving a tool

Archiving is a reversible lifecycle transition, not deletion. Preserve the source, final releases, and historical explanation while removing the tool from active maintenance obligations.

1. Change the tool README, skill, and site from active installation guidance to a historical showcase explaining why development ended.
2. Fix any known presentation defect that would undermine the preserved showcase.
3. Merge and deploy the tool's historical site while the repository is still writable.
4. Move the tool from maintained to archived inventory, retain its submodule pin and final version metadata, and move it to the umbrella site's old-tools section.
5. Verify the source, final release, historical site, and umbrella links.
6. Archive the GitHub repository last, then verify those public surfaces again.

If archiving interrupts a public surface, unarchive the repository, relocate or repair the preserved material, verify it, and archive again. To revive a tool, explicitly unarchive it, move it back to maintained inventory, restore active documentation and CI, and refresh the concerns that cover it before publishing new work.

---

## Commands

```bash
devenv test                               # umbrella: actionlint over the Pages workflow
crosscut list                             # the concerns and how old each view is
crosscut refresh [slug...]                # refresh views headless (six at a time)
python3 scripts/generate-version-json.py  # after any pointer or version change
(cd tools/<name> && devenv test)          # a tool's own checks, in its own directory
```

---

## Submodule workflow

1. Make changes in `tools/<name>/`
2. Commit and push to the tool's own repo/branch
3. Open the child PR, merge current child `main`, and explicitly dispatch its serialized integration workflow
4. Wait for that workflow to merge, release, deploy, and record `integrated-ci` on its merge commit
5. Fast-forward the child checkout to the merged `main`
6. From workspace root: `git add tools/<name>`, regenerate `docs/version.json` with `python3 scripts/generate-version-json.py` — never by hand — and commit both together
7. Push, and let the Pages workflow deploy the site — it runs on any `docs/` push and takes about twenty seconds

Do not defer step 7: until it runs, the live umbrella site names versions that `main` no longer pins. Land a pointer bump before starting other umbrella work, and carry unrelated umbrella changes with it if a branch is already open.

---

## What belongs where

| Content | Location |
| --- | --- |
| Development process, loops, discipline | This file |
| Cross-cutting concerns (questions, how to look, dated views) | `crosscut/concerns/*.md` |
| The projects those concerns look at | `crosscut/projects.md` |
| CrossCut itself (skill, CLI, doctrine) | `tools/crosscut/` |
| Shared Rust libraries | Independent repos pinned under `libraries/` |
| Tool-specific product/architecture facts | `tools/<name>/AGENTS.md` |
| Tool CI, releases, Pages | Tool's own repo |
| Umbrella site | `docs/` |

---

## Git identity

Personal repo. Use:

```
user.name = Max Clarke
user.email = maxeonyx@gmail.com
```

Pushing is safe — remote preservation. Commit and push frequently.
