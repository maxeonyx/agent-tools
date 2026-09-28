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

The concern applies strongly. The ledger's trust boundary has been built carefully. The release gate beside it is much weaker, and the result is unverified binaries.

- **The release gate runs unpinned code from another repository (observed).** The `ci.yml` of trunc, tb, dotsync, agent-harness and help-test runs `git clone --depth=1 https://github.com/maxeonyx/tdd-ratchet-rs` at whatever `main` is, then `cargo install --path` on it, inside the job that posts `Ready` and goes on to merge and release.
  - The ledger workflow pins tdd-ratchet `v1.1.6`. The gate and the ledger can therefore run different ratchets.
  - A push to tdd-ratchet `main` changes every sibling's gate on its next run, without any sibling commit.
  - oc, the archived tool, is the only one that pins a rev.
- **Actions are pinned by tag (observed).** 38 `actions/checkout@v6`, 17 `dtolnay/rust-toolchain@stable`, and `taiki-e/install-action`, `Swatinem/rust-cache`, `raven-actions/actionlint` and others are all pinned by tag. Only the umbrella's two cachix actions are pinned by SHA. trunc's and agent-harness's `pages.yml` use older major versions than their siblings.
- **The required check is self-posted, and its binding differs (observed).** Each workflow posts `Ready` through the statuses API with `GITHUB_TOKEN`. trunc binds the check to app 15368 (GitHub Actions). tb, dotsync, tdd-ratchet, agent-harness and help-test have `app_id: null`, so any token with `statuses: write` can satisfy it. `enforce_admins` is on everywhere.
- **The ledger's rules exist twice (observed, from the survey):** once in Rust inside tdd-ratchet, and once as about 60 lines of jq in each sibling's `ledger.yml`. All five tool ledgers (trunc, tb, dotsync, tdd-ratchet, agent-harness) lack the skip-when-unchanged step that the umbrella has, per `trusted-tdd-ledger` findings; `TODO.md` says four.
- **The user's end of the chain (observed):**
  - Install is `curl -Lo ~/.local/bin/<bin> https://<tool>.maxeonyx.com/releases/...`, with no checksum and no signature.
  - trunc's `pages.yml` runs `gh release download ... || true`. A failed download would therefore deploy a site without binaries, and installs would 404. That is inferred from the code, and no such deploy has been observed.
  - Skills are also curl-installed into `~/.config/opencode/skills/`. A skill is a prompt that agents follow, so it is part of the chain too.
- **Secrets (observed):** only `secrets.GITHUB_TOKEN` is referenced, 37 times. There are no personal access tokens (PATs) and no deploy keys in the workflows. That is good. The blast radius is the account itself, not a secret that could leak.
- **Unknown:**
  - Actions settings at repository and account level, such as whether the default token permissions are read-only and whether fork PR workflows need approval. I did not read `actions/permissions`.
  - Whether 2FA is on the account, which only Max can say.
  - Whether the single-account posture is deliberate. It probably is.
- **Worth considering:**
  - Pin the gate's ratchet to the same tag as the ledger. This is one line in five files, and it removes the gate/ledger mismatch.
  - Publish `SHA256SUMS` with each release.
  - Move to one reusable workflow before the next fix has to be applied six times.
  - Pin actions by SHA with a Dependabot or Renovate bump. This matters most once auto-update ships.
- **Since last view:** first view.
