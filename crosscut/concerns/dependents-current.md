# Dependents are current

Whatever depends on a tool runs the release you believe it runs.

## User stories

- **Max reading the umbrella site:** the versions it lists are the ones that are actually released.
- **Tools consuming help-test:** they pin the tag they think they pin.

## What it looks like here

- The umbrella's submodule pins and `docs/version.json` against each tool's latest release.
- help-test consumers pin by git tag in `Cargo.toml`.
- The `crosscut` branch of the umbrella pins tools that `main` may not.

## How to look

- Per tool: `git -C tools/<t> describe --tags --always` against `gh release view -R maxeonyx/<repo> --json tagName -q .tagName`.
- `jq . docs/version.json`, and `curl -s https://tools.maxeonyx.com/version.json`.
- help-test: `grep -n help-test tools/*/Cargo.toml` against `git -C libraries/help-test describe --tags`.
