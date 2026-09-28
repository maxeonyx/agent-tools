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

The last view was acted on the same day it was written: the ratchet machinery is gone, and `crosscut/` now costs very little. The open question has moved from cost to whether the views get read and stay short. Half the concerns have no view yet, and the three views that exist predate the retirement.

- **The previous view led to action (observed).** The retirement commit `950b649` deleted `crates/standards` (35 modules, about 7k lines), the umbrella ledger workflow, `.test-status.json` and `state.json`. It quotes Max: "machinery for the machinery… interpreted as compliance." The same first CrossCut pass produced tmux-bridge#12 and tdd-ratchet-rs#15, which are pinned in `8548e03`.
- **Cost now (observed):**
  - 2 of 204 umbrella commits in the last 3 months touch `crosscut/`, and both were made today.
  - No helper scripts.
  - Umbrella CI is `pages.yml` only, so the concerns cost nothing in CI.
  - `AGENTS.md` went from 23 KB to 14.6 KB. Its CrossCut section is three short paragraphs of explanation.
  - There is no refresh log, so the refresh cost is unknown. `crosscut` is not on `PATH` here, so `crosscut list` could not be run.
- **Size:**
  - The 8 concern files hold about 5,300 words, and about 2,300 of those are Current view text.
  - Four concerns (`changeability`, `cli-experience`, `release-coherence`, `standalone-repos`) have no view yet.
  - The three existing views are 592, 518 and 389 words. This concern's previous view was 807 words, the length of the investigation rather than its conclusions.
- **Views out of date after the retirement (observed):**
  - The views of `agent-guidance`, `ci-supply-chain` and `installed-reality` were written at `d76ac76`, before the retirement.
  - They still describe things that are gone. For example, `agent-guidance` still describes a 4.4 KB first section of `AGENTS.md` about ledger procedure, which no longer exists.
  - This is expected until they are refreshed, but a reader today would be misled.
- **Leftovers from the old machinery:**
  - The grep over md/yml/py/nix/toml, excluding `target/` and `crosscut/`, finds nothing.
  - `docs/reviews/*.json` files are still there in dotsync (4), trunc (3) and oc (3).
  - dotsync's and trunc's copies are still public: `/reviews/code-quality.json` returns 200 on both sites.
  - Removing them is already in `TODO.md` for the next PR in each of those tools. oc's are deliberately left, because oc is archived.
  - `target/standards-fixtures` remains locally. It is gitignored, so it does no harm.
- **Signs of drift towards enforcement (judgment):** none structural.
  - `crosscut/README.md` and `AGENTS.md:13` both carry the doctrine and the requirement to pass it on.
  - The concern files contain no pass/fail language.
  - Two lines in `AGENTS.md` read as mild obligations and are worth watching:
    - line 35, "Update… the relevant CrossCut concern, IMMEDIATELY";
    - line 116, "Exit: every tool has the improvement, or the ones that don't are named in the relevant concern's view".
  - The tools' ledger procedure that remains in `AGENTS.md` (line 130) belongs to each tool's own ratchet, which is out of scope here.
- **"How to look" needs updating (for reconsideration, not changed in this refresh):**
  - The 3-month commit ratio mostly measures history from before CrossCut, so a since-retirement window would say more.
  - The grep command fails under zsh with "no matches found" unless its `--include` globs are quoted.
- **Unknowns:**
  - Whether anyone reads the views. They are a day old, and only Max can say.
  - The refresh cost per concern (no log).
- **Worth considering:**
  - Refresh the three views that predate the retirement before they are relied on.
  - Keep views near their conclusions. Around 400 words looks like a reasonable ceiling here.
  - Delete the dotsync and trunc review JSON files, as `TODO.md` already plans.
- **Since last view:** the concern machinery described last time has been retired. This concern now asks about `crosscut/` itself, and its cost has fallen from a large share of CI and commits to almost nothing.
