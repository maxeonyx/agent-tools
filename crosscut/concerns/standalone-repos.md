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
