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

The tdd-ratchet crates.io trap is fixed in its own README and site, but the umbrella site at tools.maxeonyx.com still leads with `cargo install tdd-ratchet`. On Max's machine, trunc, tb and cargo-ratchet are still well behind their releases.

- **Max's dev machine (observed with `command -v`, `--version` and `ls`):**
  - `~/.local/bin/trunc` is 0.2.0, installed 2026-02-08. The site says 0.4.13. No change since the last view.
  - `~/.local/bin/tb` is 0.1.12, installed 2026-05-16. The site says 0.1.32, one release further ahead than at the last view.
  - `~/.cargo/bin/cargo-ratchet` is 1.0.3, with a file date of 2026-08-30. The site and the ledger pin now say 1.1.7. This is still the hazard the umbrella `AGENTS.md` warns about.
  - `~/.local/bin/dotsync` is 0.11.0, installed today. It matches the site and is two minor versions newer than the umbrella's pin and `docs/version.json` (0.9.1).
  - agent-harness and oc are not installed.
  - This supports the earlier inference: a tool gets reinstalled only while it is being worked on.
- **The crates.io path is half fixed (observed):**
  - Today's tdd-ratchet commit `c5f8c68` makes its README and live site lead with the Pages binary, then `cargo install --git … --locked`, and says outright that crates.io's 0.1.0 is unmaintained.
  - The umbrella's `docs/index.html:200` (served live at tools.maxeonyx.com) still shows `cargo install tdd-ratchet` first. crates.io is unchanged: max_version 0.1.0, 91 downloads.
  - Someone arriving from the umbrella site still gets, silently, a ratchet a whole major version behind.
- **The Pages curl installs are current (observed):** every documented `/releases/<bin>-x86_64-linux` URL returns 200, and every site's `version.json` matches the umbrella pins, except dotsync's.
  - There are still no checksums; `cargo-ratchet-x86_64-linux.sha256` returns 404.
  - `cargo install --git` with no tag (tb, tdd-ratchet) builds whatever is on the main branch, not a release (inferred from the command).
- **Skills are an install target that drifts too (observed):**
  - The installed opencode skills for tmux-bridge and tdd-ratchet differ from each tool's `docs/SKILL.md`. tb's differs only in its description line. tdd-ratchet's differs by about 160 diff lines, and the repo copy was last changed 2026-08-31.
  - No trunc, dotsync or agent-harness skill is installed, even though each ships a `docs/SKILL.md`.
  - An `oc` skill is still installed, dated 2026-04-06.
- **No update path exists (observed in the last view, not re-surveyed today).** `auto-update-integration` and agent-tools#31 are still the plan.
- **oc:** it is archived, but oc.maxeonyx.com now serves both `version.json` (0.3.23) and `/releases/oc-x86_64-linux` (200). That contradicts `projects.md`, which gives v0.3.20 and says oc has no release path. Whether anyone runs it is unknown.
- **Unknown:** other machines, Windows (agent-tools#3), and whether anyone besides Max installs these tools. Only Max can answer these, and this headless refresh could not ask.
- **Worth considering:**
  - Change the umbrella site's tdd-ratchet card to match the tdd-ratchet README. That is a one-line edit and closes the last public crates.io trap. Yanking or republishing on crates.io is the structural version of the same fix.
  - Reinstall trunc, tb and cargo-ratchet on this machine. It takes a minute and is local relief only.
  - Fold skill installation into whatever becomes the update mechanism, since skills go stale the same way binaries do.
- **Since last view:** the tdd-ratchet README and site were fixed. dotsync went to 0.11.0 and was installed. The tb and tdd-ratchet releases moved on (0.1.32 and 1.1.7) while the installed copies did not. Skill drift and the oc binary were checked for the first time.
- **Noticed along the way:**
  - The umbrella's own `docs/version.json` lags dotsync by two releases, so the umbrella site under-reports what it offers. That may belong to `release-freshness`.
  - `jq`, which "How to look" uses, is not installed on this machine. I used `python3` instead.
  - `projects.md`'s description of oc looks out of date.
