# What code runs with release authority in our CI, who can change it, and what does a user's machine end up trusting?

## Why this matters here

Every maintained tool ships through one dispatched workflow. That workflow validates, posts its own required `Ready` status, merges, releases, and copies the binaries to Pages. Users then `curl` those binaries into `~/.local/bin` with no checksum. So whatever code runs inside that workflow can, in effect, choose what lands on every user's `PATH`, and that includes the agents Max runs. When auto-update arrives (agent-tools#31, `auto-update-integration`), the channel will also push to machines on its own. The reservoir example "the update channel is the most powerful code path you ship" then becomes literal.

The concern applies strongly to trunc, tb, dotsync, tdd-ratchet, agent-harness and help-test, which share one copied CI template. It applies to the umbrella ledger. It applies weakly to archived oc, whose binary is still served. All of it sits behind one GitHub account, `maxeonyx`, and that is probably a deliberate trade-off for a one-person ecosystem.

Better does not mean enterprise supply-chain hygiene. It means three things:
- each workflow runs code that someone deliberately chose, at a known version;
- the copies of the template differ only where they mean to;
- a user can tell that the binary they fetched is the one that was built.

## How to look

1. **Could it disappear or become structural?** One reusable workflow, called with `uses: maxeonyx/<repo>/.github/workflows/x.yml@<sha>`, would replace six copies, and drift would become impossible. Serving a `SHA256SUMS` alongside the binaries, and checking it in the install snippet, makes integrity visible at almost no cost.
2. **Existing tools:**
   - `zizmor` (a GitHub Actions security linter) or `actionlint` (already in every devenv), run over `.github/workflows/`.
   - `gh api repos/maxeonyx/<repo>/branches/main/protection --jq '.required_status_checks.checks'` for each repository, to see whether `Ready` is bound to an app.
   - `gh api repos/maxeonyx/<repo>/actions/permissions` for each repository.
3. **Cheap deterministic checks:**
   - `grep -ohE 'uses: [^ ]+' */.github/workflows/*.yml | sort | uniq -c` shows unpinned or drifted actions.
   - `grep -n 'tdd-ratchet-rs' tools/*/.github/workflows/ci.yml` shows how each gate obtains its ratchet.
   - `diff` each tool's `ci.yml` and `ledger.yml` against trunc's copy.
4. **Judgment:**
   - Given one of these workflows and the permissions it has, what is the easiest way for a change nobody reviewed to reach a released binary?
   - Which of the defences here are real, and which are ritual?
   - For each weakness, is it a deliberate trade-off? Ask Max once, and record the answer here.

Not in scope: vulnerabilities in Rust dependencies. That is `cargo audit` territory, and no concern asks it yet.

## Current view — 2026-09-28

The concern applies strongly. The real release boundary is the `maxeonyx` account and anything that holds its credentials. `Ready` and branch protection protect `main`'s history, but not what users download.

- **A pushed branch can publish without a merge (inferred from settings, high confidence, not tried).** Each tool's `ci.yml` grants `contents`, `statuses`, `pages` and `id-token: write` at workflow level. Each tool's `github-pages` environment allows deploys from any branch (`*`); only the umbrella's is limited to `main`. A branch carrying an edited `ci.yml` can be dispatched and create a release and Pages deploy. Even unedited, the PR's own code (tests, `build.rs`) runs in `ready`/`build` with that write token, left in `.git/config` because `persist-credentials` is on by default, so it could post its own `Ready`.
  - Against accidents the gate is real. Against deliberately bad code it is ritual.
  - The ledger is the exception: it has `permissions: {}`, credential-less checkouts and same-repo heads only.
- **The gate's ratchet is unpinned, and the mismatch has grown (observed).** Five `ci.yml` still `git clone --depth=1` tdd-ratchet `main`, now v1.1.7. The ledgers pin `v1.1.6`, and the umbrella pins the v1.1.7 submodule. Only oc pins a rev. `TODO.md` item 3 tracks this.
- **No action is pinned by SHA anywhere now (observed).** The umbrella's two SHA-pinned cachix actions went with its ledger (950b649). Every repository has `allowed_actions: all` and `sha_pinning_required: false`. `taiki-e/install-action@nextest` and `dtolnay/rust-toolchain@master` (x2) are moving refs by design.
- **Repository settings are good defaults (observed):** `default_workflow_permissions: read` and `can_approve_pull_request_reviews: false` everywhere. There are no tag rulesets, and `enforce_admins` is on wherever `main` is protected. oc's `main` is unprotected. The umbrella's `main` requires no checks, and it now has only Pages CI.
- **The `Ready` binding is unchanged (observed):** app 15368 on trunc, and `app_id: null` on tb, dotsync, tdd-ratchet, agent-harness and help-test.
- **Template drift is mostly intended (observed).** The `ci.yml` diffs are the binary names, tb's tmux install, dotsync's release guard, the ratchet self-install, and help-test being source-only. Four `ledger.yml` files are identical; tdd-ratchet's intentionally uses the base SHA. trunc's and agent-harness's `pages.yml` still use `checkout@v4`, `configure-pages@v5` and `deploy-pages@v4`.
- **The user's end of the chain:**
  - The served binaries match their releases (observed today). For trunc, tb, dotsync, agent-harness and cargo-ratchet, the served file's SHA-256 equals GitHub's asset `digest` for the release.
  - There is still no `SHA256SUMS` (it 404s), no provenance attestation, and no check in the install snippet.
  - All six `pages.yml` files have `gh release download ... || true`, not only trunc's.
- **This machine (observed):**
  - The installed trunc 0.2.0, tb 0.1.12 and `~/.cargo/bin/cargo-ratchet` 1.0.3 are far behind their releases (0.4.13, 0.1.32, 1.1.7). Their origin can no longer be checked.
  - dotsync matches its release.
  - The local `gh` holds an OAuth token for the account, so any agent run here holds release authority too.
- **Unknown:**
  - `zizmor` and `actionlint` were not run. Neither is on `PATH` outside the devenv.
  - Whether 2FA is on the account (only Max knows).
  - Which agents or machines hold account tokens.
  - Whether the branch-can-publish path is an accepted trade-off.
- **Worth considering:**
  - Deploy Pages only from a `main`-triggered workflow, so the environment can be `main`-only. This is the design rung, and it closes the headline path.
  - Scope write permissions per job, and give the `ready` and `build` jobs `persist-credentials: false`.
  - Turn on `sha_pinning_required` and add Dependabot. A structural switch beats a grep.
  - Pin the gate's ratchet.
  - Check the published digest in install snippets, and consider `actions/attest-build-provenance`.
- **Since last view:**
  - The umbrella ledger is retired, and the SHA pins went with it.
  - Actions settings and environments are now observed.
  - Served binaries are verified against their digests.
  - `|| true` turns out to be everywhere.
  - The ratchet now spans three versions.
- **Noticed along the way:** stale installed binaries belong to `release-coherence` and auto-update. The ledger-import hole in `TODO.md` item 4 belongs to `trusted-tdd-ledger`.
