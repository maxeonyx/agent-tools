# Would a fresh agent do good, safe work from the AGENTS.md files, and are they true?

## Why this matters here

Agents are the main operators of this ecosystem. They write most of the commits, dispatch CI and bump pointers. The tools are built for agents too. So the `AGENTS.md` files work as the ecosystem's control surface: when they are wrong, an agent follows them into the wrong action, confidently, and it may happen from several sessions at once. The files total about 1,200 lines. The umbrella's alone is 314 lines and 23 KB, and it has been edited 30 times since June, which shows that incidents get turned into prose.

The concern applies strongly to the umbrella and to tb, dotsync, trunc and tdd-ratchet. It applies weakly to archived oc. Better would mean each file is:
- short enough to be read in full;
- true when it is checked against the repository;
- free of commands that do damage when they are followed literally;
- free of rules that exist only because a setup step is strange, where that step could instead be fixed.

## How to look

1. **Could parts disappear?** Many paragraphs describe a trap and how to step around it. Examples are the dispatch-after-ledger race, deleting `.tdd-ratchet.json` in the next commit, and marking a PR ready before dispatching. For each one, ask whether the tool or the workflow could remove the trap instead. When that happens, delete the paragraph.
2. **Deterministic, cheap:**
   - Run each fenced command that the guidance tells agents to run, in a scratch clone. For example, `cargo test -p trunc` from the umbrella root fails today with "did not match any packages".
   - Check that every path and URL it names exists.
   - Compare the numbers it quotes with reality. For example, tb's `AGENTS.md` says "96 tests", while its ledger has 99.
   - Compare each rule with what history shows. For example, compare the "Git identity" rule with `git log --since=<date> --format='%an <%ae>' | sort | uniq -c`.
3. **Judgment (the main mechanism):** start a fresh agent with only the repository and one realistic task, such as "fix tb's session leak" or "bump the dotsync pointer". Have it read the guidance and say, before it acts, what it would do. Then ask the following:
   - Would that plan be correct and safe?
   - Where did the guidance mislead it?
   - Which paragraphs did it need, and which were history presented as architecture?
   - Do the umbrella and a child disagree?
   - Check `.codex/config.toml` and any harness settings too: what may an agent do here without asking?

Not in scope: product documentation for end users. The `installed-reality` concern covers install instructions.

## Current view — 2026-09-28

The concern applies strongly. The guidance is detailed and mostly earnest, and it contains at least one command that does damage when followed, several untrue statements, and a large share of procedure that exists only to work around the ledger and CI design.

- **Damaging when followed (observed):** `tools/tb/AGENTS.md` lines 96–106 tell agents, after any `cargo ratchet` or `cargo nextest` run, to sweep leaked sessions with `tmux ls | grep -oE '^tb-[^:]*' | … kill-session`. Real tb sessions are also named `tb-{id}` (the same file, "Session naming"). Test mode uses `tbtest-` (`src/main.rs`). So the sweep kills the user's live tb sessions, which may include the session the agent itself is working through. In the other direction, running the umbrella suite on 2026-09-28 left two `tb-help-run-*` / `tb-help-launch-*` sessions in the live tmux server. That is exactly the leak the paragraph describes, and the sessions carry the real `tb-` prefix. They were killed by exact name. A test-only prefix already exists, so the sweep could target `tbtest-` and the leaking `tb-help-*` prefixes, or the harness could reap its own sessions (tmux-bridge#11).
- **Untrue or drifted (observed):**
  - The umbrella's "Commands" section says `cargo test -p trunc` and `cargo test --test '*' -p trunc`. These fail, because the root workspace has only `standards` as a member.
  - The "Git identity" rule names the personal identity, yet 76 of 242 umbrella commits since June, and 72 in dotsync, use a work-domain address. This may be deliberate. That is a question for Max.
  - The rule that attestation state lives only in `state.json` sits alongside `docs/reviews/*.json` files that remain in every tool.
  - tb says "96 tests" and points the skill URL at `maxeonyx.github.io`, while every other page uses the canonical domain.
  - The README omits agent-harness from the maintained tools.
- **Procedure that exists because of the design (observed):** 21 lines of the umbrella file are about the ledger or ratchet. Its first section is about 4.4 KB, covering ledger dispatch, timing and history repair. dotsync and agent-harness carry the same dispatch-race paragraph word for word. These traps are timing rules that an agent must remember, and nothing enforces them.
- **Tone and pull (observed):** the umbrella file has "IMPROVE PROCESS FIRST … THE FIRST JOB" and "LEAVING TESTS RED IS A SUPERPOWER" in capitals, plus "Pushing is safe … Commit and push frequently". The last of these is repeated in trunc, tdd-ratchet and oc, and many coding harnesses default to the opposite. How capitals like these affect current models is unknown, but they may make agents rewrite process when they should be doing the task (inferred).
- **Permission posture (observed):** the umbrella's checked-in `.codex/config.toml` sets `approval_policy = "never"` and `sandbox_mode = "danger-full-access"`. So any Codex session opened in any clone of this public repository runs without approvals. Combined with guidance that tells agents to push, dispatch workflows and rewrite history, that session can reach everything the `gh` login can reach. This may well be deliberate for Max's machine. It is worth seeing because it travels with the repository.
- **Good:** most rules carry their reason, and the incident history often names the commit, as in dotsync's `39f73c6` example. That makes a claim checkable.
- **Worth considering:**
  - Fix the tb sweep now. This is the highest leverage, and it takes one line.
  - Remove the commands that no longer work.
  - When a trap is designed away, delete its paragraph along with it.
  - Treat a fresh-agent dry run as this concern's refresh.
- **Since last view:** first view.
