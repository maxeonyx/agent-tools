# What does the umbrella's concern machinery cost, and what does it actually let us see?

## Why this matters here

agent-tools exists to make the ecosystem's quality visible (`VISION.md`). Its owner intends to evolve the current machinery in place until it is an application of CrossCut. That machinery is `crates/standards`: 35 concern modules and about 6.6k lines of Rust, run as tests under the TDD ratchet, with `NOT_APPLICABLE` lists, review attestations in `state.json`, a dispatch-only trusted ledger workflow, and the process prose in the root `AGENTS.md`.

The machinery is itself the largest recurring cost in the umbrella. Whether it pays for itself is the question that decides what the migration keeps, what it changes, and what it deletes. This is CrossCut applying itself to itself, so the same question applies to this `crosscut/` directory as it grows.

The concern applies strongly to the umbrella, and to each tool's copy of the ledger and CI template. "Better" means every mechanism produces a view that someone reads and acts on, at a cost that is proportionate. Anything that does not should be named as a candidate for deletion or redesign.

## How to look

1. **Could parts disappear?** For each concern module, ask three things:
   - Would a redesign remove the question?
   - Is the property already structural? For example, a CLAUDE.md alias could simply be generated.
   - Is it dead? `auto_update` is an empty test recorded as `passing`, and it is kept as a "historical registry alias".
2. **Cost, deterministic:**
   - Share of umbrella commits spent on the machinery: `git log --since=<3 months ago> --format=%s | grep -ciE 'ledger|ratchet|tdd|pending|status'`, pointer bumps (`grep -ciE '^point|^pin'`), and `git log --format= --name-only | sort | uniq -c | sort -rn | head`.
   - CI wall time per workflow: `gh api "repos/maxeonyx/<repo>/actions/runs?per_page=100&created=>YYYY-MM-DD"`, then sum `updated_at - run_started_at` by workflow name.
   - The size of `AGENTS.md`, and how much of it is ledger mechanics.
3. **Yield, deterministic:**
   - Run `cargo nextest run -p standards --no-fail-fast` in the umbrella devenv. This skips the ratchet, so it does not write the umbrella ledger. It still has side effects, so budget for them. It takes about 10 minutes. It rewrites `devenv.lock` in the umbrella and in five submodules, and `tools/tb/.test-status.json` as well; restore them afterwards. It can also leave `tb-help-*` sessions in the tmux server of whoever runs it. Compare the output with `.test-status.json`:
     - Which concern tests are `pending`, and have been for how long?
     - Which ones recorded as `passing` are red today?
     - Why is each one red: code, or external state that moved?
   - Count current attestations in `state.json` against the applicable (target, agentic concern) pairs.
4. **Judgment:** read three or four concern modules end to end, and ask the following.
   - Does the result tell a human where each tool sits, or only give one bit for the whole ecosystem?
   - Does the mechanism measure the property, or a source-text proxy for it? agent-tools#31 already asks this question.
   - Would the question survive as a CrossCut concern file? That means one question, a "How to look", and a dated view.
   - What would be lost if the module were deleted?

Not in scope: whether tdd-ratchet is a good product. That is tdd-ratchet's own business.

## Current view — 2026-09-28

The concern applies strongly. The cost is high and visible. The yield is real for the mechanical checks. The judgment half produces almost no views, and the ratchet model distorts several of the others.

- **Cost (observed):**
  - 242 umbrella commits since 2026-06-01. About 64 of them are about the ledger, ratchet or status, 56 are pointer or pin bumps, and 30 edit `AGENTS.md`.
  - In the last month the umbrella's ledger dispatches took 25 runs and about 366 minutes of wall time. The rest of the umbrella's CI took 7 minutes, and each tool's CI took 19 to 141 minutes.
  - The repositories are public, so the cost is latency and agent waiting, not money (inferred).
  - `AGENTS.md` is 23 KB. Its first section, on ratchet and ledger mechanics and history repair, runs to about 4.4 KB of dense procedure before "The goal".
- **The judgment rung is effectively empty (observed):**
  - There are 9 agentic concerns across 6 targets.
  - `state.json` holds 4 attestations, all for tdd-ratchet and all for commit `06d8fc99` (2026-09-02). All 4 are now stale, because 19 commits have landed since, one of them a CLAUDE.md alias.
  - Freshness is defined as "the attestation names the latest commit that touches anything except `docs/reviews` and `state.json`". The ledger bot's own commits, which touch `.test-status.json`, therefore invalidate every attestation. So does any one-line change.
  - As defined, the attestation set can never all be current at once (inferred).
  - agent-tools#28, to complete the attestations, has been open since 2026-09-02.
  - The review prompts point to skills (`thermonuclear-review`, `error-handling`) that exist only in `~/.config/opencode/skills` on Max's machine. No repository contains them, so a headless reviewer has no access to them.
  - `review-attest` refuses `help-test` as a target, yet `integration-policy` and `merge-policy` expect help-test attestations.
- **Results are one bit per concern (observed).** Each concern is a single test, and it panics with a list of findings. The ledger therefore records `pending` for "tb is missing one attestation" and `pending` for "every tool is missing everything", with no difference between them. The finding lists in the panic output are the actual map, and nothing persists them.
- **External state inside a ratchet (observed):**
  - `release_freshness`, `pinned_main_parity` and `version_artifacts` are recorded `passing`. All three fail locally today, for one reason: dotsync released v0.10.0 and v0.10.1, and the umbrella still pins v0.9.1.
  - The ratchet treats passing → failing as a regression. So a child release makes the umbrella's ratchet red, with no change in the umbrella.
  - The same class of problem explains why the umbrella's own `tdd_ratchet` concern "stays pending" in CI: tb's tests share host tmux state (`TODO.md`, tmux-bridge#11). This run reproduced it locally. No tmux server was running beforehand, so the first test started one, and the two tb prefix tests then failed as "previously passing test now fails". `TODO.md` describes this as a CI-only failure. That is true only when a developer's server predates the run.
  - Time-dependent observations fit a dated view better than they fit a ratchet (inferred).
- **Applicability is barely used (observed).** 31 of the 35 `NOT_APPLICABLE` lists are empty. `merge_policy` lists `wmux`, which is not in the inventory. The doctrine's "not applicable is not good" is not violated here. It is simply not being exercised.
- **Divergent sources of truth (observed):**
  - `AGENTS.md` says attestations live only in `state.json`, while tools still carry `docs/reviews/*.json`. Those files are published to Pages, and tb's names a commit that does not exist on GitHub.
  - `docs/version.json` lists oc 0.3.23, which was never released. oc's newest release is v0.3.20.
  - `README.md` omits agent-harness from "Maintained tools", although `MAINTAINED_TOOLS` includes it.
- **What works (observed):**
  - The mechanical, fixture-backed checks are clear and quick, and they find real things. Examples are `claude-md-alias` (help-test lacks CLAUDE.md), `trusted-tdd-ledger` (5 tool ledgers lack the skip-when-unchanged step), and `pinned-main-parity`.
  - The "red is information" framing in `VISION.md` is close to CrossCut's doctrine already.
- **Worth considering:**
  - Keep the fixture-backed mechanical checks as the deterministic rung beneath CrossCut concern files, and stop ratcheting their results.
  - Move time-dependent checks, such as freshness, parity and live sites, into dated views.
  - Replace attestations with the dated "Current view" of a concern file, which already records its date and evidence.
  - Delete dead modules such as `auto_update`.
  - The largest single simplification may be letting the umbrella stop ratcheting concern outcomes at all. That would remove most of the ledger dispatches and much of `AGENTS.md`. That is Max's call.
- **Local run, 2026-09-28:** `cargo nextest run -p standards` took 610 s. 105 tests ran: 86 passed and 19 failed. 15 of the failures match the ledger's `pending` entries, and the rest are the three freshness tests above plus `tdd_ratchet_gatekeeper`, which refuses to run outside the ratchet by design.
- **Since last view:** first view.
