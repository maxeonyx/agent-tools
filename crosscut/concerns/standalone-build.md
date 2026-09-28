# Standalone build

A fresh clone of each repository builds and tests with only what it declares.

## User stories

- **A contributor from GitHub, or CI:** they have only this repository.
- **Max:** the umbrella must stay optional, never a hidden dependency.

## What it looks like here

- Rust repos: no `path = "../` dependencies outside the repository, no umbrella clones in CI, and no configuration inherited from a parent directory.
- The integration run of each tool builds from its own checkout.

## How to look

- `grep -rn 'path = "\.\./' tools/*/Cargo.toml libraries/*/Cargo.toml`; `grep -rln 'agent-tools' tools/*/.github/workflows/`.
- The latest `integrated-ci` status on each pinned commit: `gh api repos/maxeonyx/<repo>/commits/$(git -C tools/<t> rev-parse HEAD)/statuses --jq '.[].context' | sort -u`.
