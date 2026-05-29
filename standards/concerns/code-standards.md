# Code Standards

Shared clippy and rustfmt config. All tools pass workspace-level linting.

## Why

Inconsistent formatting and lint rules between tools means context-switching when moving between them. Agents make different style choices in each tool, creating unnecessary diffs and divergent idioms. Shared config means one set of rules, one muscle memory, and no "cleanup formatting" commits polluting history.

The workspace-level check is the enforcement: if a tool doesn't pass `cargo clippy --all -- -D warnings` from the workspace root, it's non-compliant.

## What compliance looks like

- No tool-local `rustfmt.toml` overriding the workspace config
- `cargo fmt --check -p <tool>` passes from the workspace root
- `cargo clippy -p <tool> -- -D warnings` passes from the workspace root
- No `#[allow(...)]` attributes without a comment explaining why the exception is necessary

## How to bring a tool into compliance

1. Remove any tool-local `rustfmt.toml` or `clippy.toml`
2. Run `cargo fmt -p <tool>` from workspace root to reformat
3. Fix all clippy warnings (usually `uninlined_format_args` and similar)
4. Commit the formatting fixes separately from logic changes

## How to maintain

The workspace `rustfmt.toml` and `rust-toolchain.toml` are the source of truth. When upgrading the toolchain, expect new lints — fix them across all tools in one pass.
