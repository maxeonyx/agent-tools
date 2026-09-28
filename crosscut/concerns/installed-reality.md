# When someone follows our install path, what do they end up running, and does it ever get newer?

## Why this matters here

The ecosystem's effort goes into the repositories: ratchets, releases, pointer bumps, sites. But the value of a CLI tool is delivered by the binary on someone's `PATH`. That binary might belong to Max, to one of his agents, or to a stranger who found a site. Nothing in the older concern suite looks at that binary. `release-freshness` compares a pin with a GitHub release. `auto-update-integration` checks for a library that does not exist yet. Neither one asks what is actually running.

This applies strongly to Max's own machines. The agents that use trunc, tb and cargo-ratchet every day run whatever was installed there, so an improvement that never reaches that machine has improved nothing. It applies moderately to public users of the sites and of crates.io, because the scale is unknown. It matters most for tdd-ratchet, because an old ratchet makes different judgments: the umbrella `AGENTS.md` records a stale 1.0.0 reporting green while the history checks were red.

Better would look like this. Every documented install path yields the current release. There is a cheap way to see which version a machine runs. Staleness either cannot persist, because an update mechanism exists, or it is at least visible.

## How to look

1. **Could it disappear?** An update mechanism in each tool would dissolve most of the machine-staleness part, and agent-tools#31 already plans for it through `auto-update-integration`. Deleting install paths that nobody maintains would dissolve the crates.io part. Once auto-update exists, this concern changes shape. The main question becomes "is the update channel trustworthy?" (see `ci-supply-chain`), not "is it stale?".
2. **Deterministic, with existing tools:**
   - On each machine you can reach, for each binary, compare `command -v <bin>` and `<bin> --version` with `curl -s https://<site>/version.json`. The sites are trunc.maxeonyx.com, tmux-bridge.maxeonyx.com, dotsync.maxeonyx.com, tdd-ratchet.maxeonyx.com and agent-harness.maxeonyx.com. Also check `ls -la --time-style=long-iso` on each binary for its install date.
   - Check crates.io: `curl -s -A crosscut https://crates.io/api/v1/crates/tdd-ratchet | jq .crate.max_version`. Compare it with the version the README and the site tell people to `cargo install`.
   - Run `grep -ohE '(curl|cargo install|irm|iwr)[^<]*' tools/*/docs/index.html tools/*/README.md docs/index.html` to list every documented install command.
   - Skills installed on the machine (`~/.config/opencode/skills/*`, `~/.claude/skills`) are also install targets: compare them with each tool's `docs/SKILL.md`.
3. **Worth building only if it stays cheap:** a script that takes each documented install command, runs it in a scratch `HOME` or a container, and runs `--version`. This is the "run every documented command" pattern. Do not build it until the manual check has found something twice.
4. **Judgment:** which machines and harnesses actually use these tools? Are there Windows machines (agent-tools#3)? Is anyone outside Max installing them? Those are questions for Max. Record the answers here so the next view does not have to ask again.

Not in scope: development builds inside a clone.

## Current view — 2026-09-28

This applies strongly, and the current state is weak. The repositories are well ahead of what actually runs.

- **Max's dev machine (observed):**
  - `~/.local/bin/trunc` is 0.2.0, installed 2026-02-08. The current release is 0.4.13.
  - `~/.local/bin/tb` is 0.1.12, installed 2026-05-16. The current release is 0.1.31.
  - `~/.cargo/bin/cargo-ratchet` is 1.0.3. The ledger pins 1.1.6. The umbrella `AGENTS.md` warns about exactly this hazard, but the warning did not remove it.
  - dotsync is 0.10.1, installed 2026-09-27. That is current, and newer than the umbrella's pin of 0.9.1.
  - The pattern suggests that tools get reinstalled only while they are being actively developed (inferred).
- **No update path exists (observed).** No maintained tool contains an update command or update code. The survey grepped `src` for `self_update`, `releases/latest` and `api.github.com`. `auto-update-integration` is `pending`, and it expects `libraries/agent-tools-updater`, which does not exist.
- **The primary tdd-ratchet install instruction installs a Feb 2026 version (observed).** Both `tools/tdd-ratchet/README.md` and the live site (tdd-ratchet.maxeonyx.com) lead with `cargo install tdd-ratchet`. crates.io's newest version is 0.1.0, from 2026-02-16, with 91 downloads. Anyone who follows the first instruction gets a ratchet that is a whole major version behind, and gets it silently.
- **The Pages curl installs are current, but unverified (observed):** `trunc.maxeonyx.com/releases/trunc-x86_64-linux` returns 200 and `version.json` says 0.4.13. There is no checksum and no signature. tb's README offers a second path, through GitHub `releases/latest`.
- **oc is archived, but its binary is still served** from oc.maxeonyx.com. Whether anyone still runs it is unknown.
- **Unknown:** other machines, Windows, and whether anyone besides Max installs these tools. Only Max can answer that.
- **Worth considering:**
  - Reinstall on this machine. That is a minute of work, but it is only local relief.
  - Remove `cargo install tdd-ratchet` from the README and the site, or publish current versions to crates.io. This is the highest-leverage, lowest-cost fix.
  - Treat auto-update as the structural answer, and at that point move the weight of this concern onto the integrity of the update channel.
- **Since last view:** first view.
- **Noticed along the way:** a local run of the umbrella's standards suite rewrites `devenv.lock` in the umbrella and in five submodules, through `devenv_check`. On this machine that happened even though its devenv matches the one CI pins (1.4.1). That is one more way the working tree drifts under agents.
