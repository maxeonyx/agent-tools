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

- **Mechanical rungs** (cheap, run first):
  - `CLAUDE.md` should be the one line `@AGENTS.md` next to every `AGENTS.md`: `find . -name AGENTS.md -not -path '*/target/*' -not -path '*/fixtures/*' -execdir sh -c 'grep -qx "@AGENTS.md" CLAUDE.md 2>/dev/null || pwd' \;`
  - Markdown prose is not hard-wrapped (Max's rule). Judge by eye in the files you read; a paragraph broken at a steady column is the tell. The retired `markdown_no_hard_wrap` checker (450 lines, in git history before 2026-09-28) is the fallback if this ever needs to be mechanical again.

## Current view — 2026-09-28

**Better than this morning, but the procedure isn't fully written down.** Today's rewrite fixed the damaging tb sweep, the broken umbrella commands and the Codex config. What remains: the umbrella and child files don't agree on when to wait for the ledger bot, when to merge `main` and when to dispatch. They also never say what to do with the `.test-status.json` that local runs change.

This view is a second pass on the same day. The umbrella was rewritten in `950b649`, and `52190c1` and `8548e03` also landed.

- **Fixed since the earlier view (observed):**
  - tb's sweep (`tools/tb/AGENTS.md:96–106`, at v0.1.32) now matches only `tbtest-|tb-help-|tb-test-runner-`, which are the prefixes the tests actually generate. It also explains why a `^tb-` sweep would be dangerous.
  - The umbrella "Commands" section no longer uses `cargo test -p trunc`. It says the umbrella has no Cargo workspace, and every command it lists points at something that exists: `devenv test` runs actionlint, and `generate-version-json.py` has a `TOOLS` list.
  - `.codex/config.toml` is gone, and no child checks in a `.codex/` or `.claude/settings` directory.
  - "Pushing is safe" now appears only in the umbrella.
  - The umbrella's ledger/ratchet share dropped from 21 matching lines to 5.
- **Still damaging or risky (observed):**
  - `tests/common/mod.rs:582–594` `cleanup_session` also kills `tb-{id}`, the real-session name. The harness does the thing the guidance warns against, although only for a specific id.
  - The four children's ledger workflows pin tdd-ratchet `v1.1.6`, while the umbrella pins `v1.1.7`.
- **Still untrue or drifted (observed):**
  - tb says "96 tests", but `.test-status.json` has 99.
  - tb's skill URL still points at `maxeonyx.github.io/tmux-bridge`.
  - Umbrella commits since June: 165 personal, 76 work-domain. dotsync: 276 personal, 76 work-domain. Everything committed today uses the personal address, so this may already be settled. Whether the older work-domain commits were deliberate is Max's call.
  - `docs/reviews/*.json` remains in dotsync, oc and trunc. They are no longer contradicted by a `state.json` rule, but nothing explains them either.
  - `CLAUDE.md` is missing next to `AGENTS.md` in oc and help-test, and the mechanical check found both. oc is archived, so low stakes. For help-test, it matters only if Claude Code gets opened there.
- **Fresh-agent dry run (judgment, one agent, task "make tb's session reaping crash-proof and bump the pointer"):** its plan was broadly correct and safe. Where it got stuck:
  - Umbrella :19 says to use an exclusive clone, while :125 says "work in `tools/<name>/` within this workspace".
  - :183 says to "carry unrelated umbrella changes" along with a pointer bump, and it wasn't clear whose changes that means.
  - tb :7 says an expected red test "keeps CI green", while the umbrella celebrates red. It is the same word used in two senses.
  - No file says that `cargo ratchet` / `devenv test` rewrite `.test-status.json` locally and that the change must be discarded. A plain `git add -A` then fails the ledger check. The agent's claim rests on `cargo ratchet --help`. I checked the other claims I relied on, but not this one.
  - The agent had to work out from `ledger.yml` and `ci.yml:41` that pushing the merge of `main` needs its own bot wait before dispatch, and it couldn't tell whether a later bot commit can race the dispatch.
  - It found the dotsync 2026-08-12 story (:129) and "IMPROVE PROCESS FIRST" (:27–35) useless for acting.
- **Procedure that exists because of the design (observed):** the word-for-word "wait for the bot commit" paragraph now appears in tb, trunc, dotsync and agent-harness. tdd-ratchet has a variant and still carries the "delete `.tdd-ratchet.json` in the next commit" trap.
- **Tone:** the capitalised "IMPROVE PROCESS FIRST … THE FIRST JOB" and "LEAVING TESTS RED IS A SUPERPOWER" sections remain. Their effect on current agents is still unknown.
- **Good:** no prose is hard-wrapped. The umbrella now points concern work at CrossCut and carries its doctrine. The paths these files mention exist.
- **Worth considering:**
  - The ledger bot's wait/pull/dispatch sequence is the one trap left in every tool. Have the dispatched CI wait for, or itself run, the ledger step, so the paragraph can be deleted from all five files.
  - Say in one place that local ratchet runs change `.test-status.json` and that the change must be restored. Better still, make local runs not write it.
  - Reconcile umbrella :19 with :125.
  - Drop the `tb-{id}` kill from `cleanup_session`, or confirm that it's safe.
- **Since last view:** the umbrella went from 314 to 211 lines, the Codex config was removed, the tb sweep and the umbrella commands were fixed, and "Pushing is safe" was dropped from the children. The dispatch-race paragraph spread from two children to four.
- **Noticed along the way:** the children pin `v1.1.6` of the tdd-ratchet ledger while the umbrella pins `v1.1.7`. That version skew across repositories may belong in a pin-consistency concern, if one exists or gets created.
