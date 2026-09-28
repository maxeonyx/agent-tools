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
