# Could a fresh engineer change each tool safely next month, and would its tests catch a mistake?

## Why this matters here

Most change to these tools is made by agents that arrive cold. What protects the tools is whether their structure makes the next change obvious, and whether their tests would catch a wrong one quickly. That means black-box tests of real behaviour, IO that tests can control, and a fast loop for the common edit. The retired suite looked at this through file-shape proxies (a `tests/` directory exists, a 5-second second run) and four review attestations, none of them current.

This applies strongly to dotsync, which holds users' dotfiles and gets the most change, and to tdd-ratchet, whose value is being trusted. It applies moderately to trunc and tb. It does not apply to oc. Better means that for a plausible change, the place to make it is clear, the edit-to-feedback loop takes seconds, and deliberately breaking the behaviour turns a test red.

## How to look

- **Deterministic, cheap:**
  - The fast loop: `time cargo test --lib` or the tool's documented fast check. Run it twice, and record the second, warm run.
  - Where recent change has gone: `git -C tools/<t> log --since='3 months ago' --format= --name-only | sort | uniq -c | sort -rn | head`.
- **Judgment:** for one tool per refresh, pick a small realistic change from its `TODO.md` or issues. Without making it, work out:
  - Where would it go, and what would you have to understand first?
  - Which tests would tell you it works?
  - Which tests would tell you that you broke something else?

  Then deliberately break one behaviour in a scratch copy (`cp -r`, never the checkout) and see whether the suite notices, and how fast.
- Things to notice on the way: hidden global state, tests that depend on the host (tb's shared tmux server), and code that exists only for compatibility with something gone.

## Current view — 2026-09-28

Structure and test speed are good in dotsync, tdd-ratchet and trunc: whole suites run warm in seconds, and most deliberate breakages turn a test red. But one dotsync breakage passed all 151 green tests, and "How to look" measures the fast loop with a command that runs almost nothing.

Applies strongly to dotsync (300 commits in 3 months) and tdd-ratchet (158). It applies moderately and quietly to trunc and tb (29 and 28 commits, mostly version, CI and docs churn).

- **Fast loop (observed, high confidence).** Second, warm runs of the whole suite under `TDD_RATCHET=1 cargo test --no-fail-fast`, in scratch copies:
  - trunc: 102 tests, about 0s;
  - tdd-ratchet: 109 tests, 5s;
  - dotsync: 152 tests, 7s, with 1 red. The red one is `read_only_commands_leave_the_scope_bookmarks_where_they_found_them`, which is `pending` in the ledger and explained in PLAN.md as red on purpose.

  Rebuilding after a one-line edit took 8–17s, and once 53s in dotsync. Plain `cargo test` stops at the gatekeeper test on purpose, so `TDD_RATCHET=1` or `cargo ratchet` is the real loop.
- **dotsync, a worked change (judgment).** The change: make convergence merge ids reproducible (PLAN.md, "Smaller, unowned").
  - **Where it goes:** it is obvious where to make it. `src/machine.rs:30` `machine_signature` calls `Timestamp::now()`, and its callers are converge, commit, bootstrap and pause.
  - **What to understand first:** the convergence module doc and DESIGN.md.
  - **Tests:** the two-machine harness (`tests/harness`, `two_synced_machines`) could state the property black-box. `tests/convergence.rs` and `conflicts.rs` would catch collateral damage. Test files are split by area, so "which tests" has a clear answer.
  - **Hidden state:** machine identity is test-controllable (`DOTSYNC_HOSTNAME` and `DOTSYNC_OS`), but the clock is not.
- **Deliberate breakages (observed).**
  - **dotsync, caught:** continuing the pass past a conflicted scope instead of stopping turned 2 green tests red.
  - **dotsync, not caught:** no longer recording that a fast-forward moved a bookmark means a pass that only fast-forwards drops its transaction. No green test noticed; only the already-pending one went red. `a_scope_published_without_its_cascade_still_reaches_the_other_machines` still passed.
  - Why it went unnoticed is unknown (unresolved in this refresh). Either jj's import of fetched refs makes that branch redundant, which would make the flag dead weight, or a pure fast-forward cascade has no test.
  - **tdd-ratchet, caught:** removing the regression rule (a passing test that fails) turned 3 tests red in 8s.
- **tdd-ratchet observation.** `(Passing, Ignored)` is accepted without complaint (`src/ratchet.rs:420`), and `state_transitions.rs` tests this as deliberate. So adding `#[ignore]` to a test that has regressed keeps it "passing" in the ledger. Whether that is acceptable for the trust root is a question for Max.
- **tb: unknown.** Its integration tests create prefixed sessions on the user's default tmux server rather than an isolated `-L` socket. I did not run them, to avoid touching the live server, so tb's loop time and sensitivity to breakage are unmeasured. All of tb is one 1208-line `src/main.rs`.
- **trunc:** not examined beyond timing. oc: out of scope.
- **Worth considering:**
  - Replace `cargo test --lib` in "How to look" with the ratchet or area-test loop (a reconsider step).
  - Settle whether the dotsync fast-forward branch is dead or has no test. Either answer is cheap and useful.
  - Isolated tmux sockets would make tb's tests safe to run anywhere, including headless refreshes.
- **Since last view:** first view.
- **Noticed along the way:** `.test-status.json` is the most-changed file in dotsync (140 of 300 commits) and in tdd-ratchet. It is bot-written, so it inflates the churn numbers from "How to look" without meaning anything.
