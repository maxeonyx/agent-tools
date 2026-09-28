# Do the pins, releases, sites and version files all tell the same current story?

## Why this matters here

The umbrella pins each tool at a commit and publishes `docs/version.json`. Each tool has a latest GitHub release with assets, a site with its own `version.json`, and an `integrated-ci` status on its merge commit. These drift apart silently. A tool can release without the pin moving, a pin can move without the site deploying, and a `version.json` can name a release that was never made. When that happens, "what version is current?" has several answers, and install paths quietly serve the wrong one.

This applies strongly to maintained tools, and only to their final state for archived ones (oc). All of it is live external state, so it belongs in a dated view, not a test. Better means one command that shows every mismatch, and a pointer-bump routine that leaves none behind.

## How to look

- **Deterministic, per tool in `scripts/generate-version-json.py` `TOOLS`:**
  - The pinned version: `git -C tools/<t> describe --tags --exact-match` and `jq -r .version tools/<t>/docs/version.json`.
  - The latest release and its assets: `gh release view -R maxeonyx/<repo> --json tagName,assets`.
  - The live site: `curl -s https://<t>.maxeonyx.com/version.json`, and for the umbrella `curl -s https://tools.maxeonyx.com/version.json` against `docs/version.json`.
  - Parity with remote `main`: `git -C tools/<t> fetch -q && git -C tools/<t> rev-list --count HEAD..origin/main`.
  - Integration status on the pinned commit: `gh api repos/maxeonyx/<repo>/commits/<sha>/statuses --jq '.[].context'`.
- **Judgment:** for each mismatch, is it lag (a pointer bump waiting to land), or a real inconsistency (a version that was never released, a site that never deployed)? Only the second is interesting.
- If this settles into the same mechanical sweep every time, make it a script in `release-coherence/` next to this file.

## Current view — 2026-09-28

**Headline:** every maintained tool's release, site and binary agree, but the archived oc's site and both umbrella `version.json` files claim 0.3.23, which was never released. The `crosscut` branch and `main` each hold some newer pins than the other, and PR #48 conflicts on `docs/version.json`.

Applies strongly. What is left is mostly lag, plus one real inconsistency in oc's final state.

**Per tool** (observed today; high confidence unless noted):

| Tool | Pin on `crosscut` | Pin on `main` | Latest release | Site `version.json` | Served binary = release asset? |
|---|---|---|---|---|---|
| trunc | v0.4.13 | v0.4.13 | v0.4.13 | 0.4.13 | yes (sha256 match) |
| tb (tmux-bridge) | v0.1.32 | v0.1.31 | v0.1.32 | 0.1.32 | yes |
| dotsync | v0.9.1 | v0.11.0 | v0.11.0 | 0.11.0 | yes |
| tdd-ratchet | v1.1.7 | v1.1.6 | v1.1.7 | 1.1.7 | yes |
| agent-harness | v0.1.20 | v0.1.20 | v0.1.20 | 0.1.20 | yes |
| oc (archived) | v0.3.20-15-g0fa52e5 | same | v0.3.20 | **0.3.23** | yes, it is the v0.3.20 asset |

- **Real inconsistency: oc.** The `Cargo.toml` version was bumped to 0.3.23 in post-release commits (`e3588d7`). Its `docs/version.json` then published 0.3.23, but no v0.3.21–23 tag or release exists. oc.maxeonyx.com serves the v0.3.20 binary (sha256 matches the release digest) while its `version.json` says 0.3.23. The umbrella repeats 0.3.23. So "what version of oc is final?" has two answers. The repo is archived, so it would take an unarchive to fix the source, or a hand-edit of the umbrella and site to say 0.3.20.
- **Lag: the umbrella.** tools.maxeonyx.com serves `main`'s `docs/version.json` (commit 79a9382, deployed at 03:00Z, Pages green). It lists tb 0.1.31 and tdd-ratchet 1.1.6, while those tools already released and deployed 0.1.32 and 1.1.7. The `crosscut` branch (8548e03) pins the newer tb and tdd-ratchet but predates `main`'s dotsync 0.11.0 pin, so it still has dotsync 0.9.1, which is 28 commits behind remote `main`. PR #48 is `CONFLICTING` in `docs/version.json`. A careless resolution could regress the umbrella to dotsync 0.9.1. The right result is tb 0.1.32, dotsync 0.11.0 and tdd-ratchet 1.1.7.
- **Remote parity:** every other submodule pin on this branch equals its remote `main` (checked with `git ls-remote`, without fetching).
- **Integration status:** every maintained pin, including `main`'s dotsync pin, has `integrated-ci` pending followed by success. oc has no statuses, which is expected for an archived repo.

**Unknowns:** whether 0.3.23 in oc was meant to be released. Only Max knows, and nothing records it.

**Worth considering:**
- Resolve PR #48's `docs/version.json` by taking the newest pin of each tool. Merging `main` into `crosscut` soon keeps this from happening again.
- For oc, either set its final published version to 0.3.20 or accept the discrepancy knowingly. Either is cheap.
- The sweep was mechanical this time. A small `release-coherence/` script (pin, release, site, digest, status per tool) would now earn its keep. It should know each tool's repo, site domain and binary name.

**"How to look" is stale in three places:** tb's site is tmux-bridge.maxeonyx.com, not `tb.`; tdd-ratchet's binary is `cargo-ratchet`; and `jq` is not installed here (python works). Instead of `git fetch`, `git ls-remote` gives parity without changing state.

**Since last view:** first view.

**Noticed along the way:** the umbrella's long-lived `crosscut` branch diverging from `main` on pins is its own drift source. That may belong to whoever owns the branch strategy, rather than to this concern.
