# Release Pipeline Standards

Consistent CI: check → build → release → pages.

## Why

Each tool's CI should work the same way. When you fix a CI issue in one tool, you should be able to apply the same fix across all tools without re-understanding each pipeline. Consistent pipelines mean consistent behavior: version bump enforcement, debounced runs, correct asset naming, automatic releases.

## What compliance looks like

- Single `.github/workflows/ci.yml` with chained jobs: Check → Build → Release → Pages
- `concurrency: group/cancel-in-progress: true` (debounced — new pushes cancel old runs)
- Version bump enforcement: CI fails if `Cargo.toml` version already has a tag on a different commit
- Build produces bare binaries with naming: `<binary>-<arch>-<os>` (no tarballs/zips)
- Release: recreate-on-push pattern (delete old release for this version, create fresh)
- Pages: deploy `docs/` + release binaries to GitHub Pages

## How to bring a tool into compliance

1. Compare the tool's current `ci.yml` against the canonical pattern (check oc or trunc for reference)
2. Align job structure, concurrency settings, version enforcement
3. Ensure asset naming follows the convention
4. Verify the recreate-release logic works (push twice with same version bumped → only one release exists)

## How to maintain

When improving the CI pattern: fix it in one tool, verify it works, then roll the same change across all tools. The workspace makes this visible — all tools' CI files are right there.
