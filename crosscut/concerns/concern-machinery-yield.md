# Is the umbrella's concern machinery earning its keep?

## Why this matters here

The umbrella once spent about a quarter of its commits and most of its CI time on concern machinery: a ratcheted standards suite, attestations and a ledger. That machinery answered fewer questions than it generated, and it was retired on 2026-09-28. Its replacement is this `crosscut/` directory, and the same failure can happen again, only more slowly: concerns nobody reads, views that grow into reports, helper scripts that turn into a second standards crate, or process that creeps back into `AGENTS.md`.

This applies strongly while the umbrella is new to CrossCut. Better means every concern here produces a view that someone reads, and sometimes acts on, at a cost that is obviously smaller than what it shows. The whole directory should stay smaller than the problems it looks at.

## How to look

1. **Cost, deterministic:**
   - `git log --since='3 months ago' --format=%s -- crosscut/ | wc -l` against all umbrella commits over the same period.
   - Size: `wc -w crosscut/concerns/*.md`, and any helper scripts under `crosscut/concerns/*/`.
   - Refresh cost, if a refresh log or CI run exists.
2. **Yield, judgment:** for each concern, read its last two or three views (`git log -p -- crosscut/concerns/<slug>.md`), and ask:
   - Did anything change between views? Did anyone act on one, as shown by commits, PRs or issues that cite it?
   - Is the view the length of its conclusions, or of the investigation behind them?
   - Is the question still live? Has part of it become structural, or answered by a tool, so that it could shrink?
3. **Residue of the old machinery:** look for anything still serving the retired ratchet.
   - `grep -rn -E 'standards::|state\.json|review-attest|NOT_APPLICABLE' --include=*.md --include=*.yml --include=*.py . tools/*/` (excluding `target/`).
   - Published `tools/*/docs/reviews/*.json`.
   - Process text in `AGENTS.md` that polices concerns rather than explaining the work.
4. **Drift towards enforcement:** does any file here now read as a standard, a gate or a score? Does any carrier of the CrossCut doctrine drop its requirement to carry the doctrine forward?

Not in scope: each tool's own TDD ratchet on its own tests. That is the tool's business.

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
