# Can each repository be cloned, built and tested on its own, and does anything quietly depend on the umbrella?

## Why this matters here

Each tool is meant to be a complete, standalone development, CI and release context. The umbrella only pins them. Coupling creeps in anyway: a relative path into a sibling checkout, a CI job that clones the umbrella, a config file that only exists in a parent directory, a devenv that assumes the workspace layout. A standalone clone, or a contributor arriving from GitHub, then fails in a way nobody sees from inside the umbrella.

This applies to every tool and library, with most weight on the ones others build on (help-test, tdd-ratchet). Better means that a fresh `git clone` of any one repository, with only its declared environment, builds and runs its own tests.

## How to look

- **Structural evidence first:** each tool's integration run already builds from a standalone checkout. A recent green `integrated-ci` status on the pinned commit (see `release-coherence`) is the strongest cheap signal.
- **Deterministic greps:**
  - `grep -rn 'path = "\.\./' tools/*/Cargo.toml libraries/*/Cargo.toml`
  - `grep -rln 'agent-tools' tools/*/.github/workflows/`
  - Configuration inherited from a parent directory: `rustfmt.toml`, `.cargo/config.toml`, `.editorconfig` or `devenv.*` above a repository root but not inside it.
- **Occasionally, the real test:** clone one repository into a scratch directory outside the workspace, enter its devenv, and run its documented check. It is slow and costs network time, so rotate through the repositories, and say which one this view covers.

## Current view — 2026-09-28

All six active repositories look standalone: they have no path dependencies, and their CI only checks out their own repository. oc is the only one tied to the umbrella, and it is archived. The main gap is that `cargo ratchet` comes from the machine, not the repository's environment.

- **Structural signal (high confidence, observed):** `integrated-ci` is `success` on every pinned commit:
  - trunc v0.4.13, 2026-09-08
  - tb v0.1.32, 2026-09-28
  - dotsync v0.9.1, 2026-09-09
  - tdd-ratchet v1.1.7, 2026-09-28
  - agent-harness v0.1.20, 2026-09-26
  - help-test v0.1.2, 2026-08-30

  In tb's workflow (read in full; the others are assumed to share its shape), every job uses `actions/checkout` of that repository alone on a GitHub runner. A green status therefore means it built from a standalone checkout.
- **Deterministic greps (observed):**
  - No `path = "../..."` dependencies, except in oc.
  - The only workflow that mentions `agent-tools` is oc's `ci.yml`. It clones the umbrella and copies `crates/agent-tools-updater` and `crates/help-test` to `../../crates`.
  - Everything else reaches its siblings by public git URL: help-test by tag, and tdd-ratchet as a clone of main.
- **Inherited configuration:**
  - The umbrella root has `devenv.*` and `.envrc`, and has no `rustfmt.toml`, `.cargo/` or `.editorconfig`.
  - Each tool has its own `devenv.nix`, `devenv.lock`, `devenv.yaml` and `.envrc`. tb also has `.cargo/config.toml`, and oc has `rust-toolchain.toml`.
  - `~/.cargo/config*` is absent.
  - **help-test has no `.envrc` and no `devenv.yaml`.** Standalone, `devenv shell` should still work, since devenv 1.4 defaults the inputs and the lock is present (inferred, not run). Inside the umbrella, direnv would load the *umbrella's* `.envrc` in `libraries/help-test`, which has no Rust. That is umbrella-side confusion rather than a standalone break, and direnv is not installed on this machine today.
- **The devenv path is not what CI tests.** CI uses `dtolnay/rust-toolchain`, not devenv. Every devenv's `enterTest` runs `cargo ratchet`, but no devenv provides it. Here it resolves to `~/.cargo/bin/cargo-ratchet`. A contributor with only the declared environment would fail at the ratchet step. That dependency is on the machine, not the umbrella, but it breaks the "only its declared environment" bar.
- **oc:** it is archived, and it does not build from source standalone or in the umbrella, because `crates/` no longer exists there. This is already known and deliberately accepted, so it carries little weight.
- **Unknowns:**
  - The real clone-and-check was not done. The repository that a later refresh should start with is help-test, because others build on it and because of the env-file gap above.
  - I did not open the other repositories' workflows, only their integrated-ci statuses and the grep results.
- **Worth considering:**
  - Add `cargo-ratchet` to each devenv, as a package or a pinned install. That would make "declared environment" true, and would close the one gap CI can't see.
  - Give help-test the same `.envrc` and `devenv.yaml` as its siblings.
- **Since last view:** first view.
- **Noticed along the way:**
  - Every consumer pins `help-test` at tag `v0.1.0`, while the umbrella pins v0.1.2. CI installs tdd-ratchet from its unpinned main, so a ratchet change could reach every sibling's gate unannounced. This may belong in `release-coherence`, or deserve its own concern about how siblings pin each other.
  - `tools/crosscut` has no devenv. It is out of scope here.
