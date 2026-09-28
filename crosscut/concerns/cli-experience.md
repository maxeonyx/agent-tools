# Can someone with only a tool's binary, `--help`, skill and error messages get real work done?

## Why this matters here

These tools are built for agents first. An agent does not read the source; it reads `--help`, the skill, and whatever the tool prints when something goes wrong. If those are unclear, stale or inconsistent between sibling tools, every session pays for it in retries and guesses. The retired standards suite asked this through five separate review attestations (help text, error messages, interactive use, the skill, output contracts), and none of them was ever current.

This applies strongly to trunc, tb, dotsync and tdd-ratchet, weakly to agent-harness while it is experimental, and not to oc (archived). Better means a fresh agent given a realistic task and only the installed tool succeeds without reading the source, and the tools feel like one family.

## How to look

- **Deterministic:**
  - Each tool's own tests already run its `--help` examples through `help-test`. Confirm this with `grep -l help-test tools/*/Cargo.toml` rather than re-running them.
  - Compare shapes across tools: `<bin> --help`, `<bin> --version --json`, and the exit code of a deliberate misuse such as an unknown flag.
- **Judgment, the main mechanism:** for one or two tools per refresh, rotating through them, give yourself a realistic task, for example "truncate this 10,000-line build log so the error is visible". Use only `--help`, `docs/SKILL.md` and the tool's output. Then provoke two or three plausible mistakes and read the errors.
  - Did you succeed, and where did you hesitate?
  - Did an error say what happened, why, and what to do next?
  - Would a terminal user and a headless agent both be well served?
- Record which tools this view covers, so the next refresh can take the others.

## Current view — 2026-09-28

**What an agent on Max's machine actually gets is mostly old binaries, so most of the CLI improvements in the repos never reach it. Only dotsync is current. The installed trunc is two minor versions behind the skill that describes it.**

This view covers the cross-tool shape checks for trunc, tb, dotsync and tdd-ratchet, and a hands-on task with trunc only. The next refresh should take tb or dotsync hands-on.

- **Installed and pinned versions differ a lot** (observed, high confidence). The installed binaries and the umbrella's pins:

  | Binary | Installed | Pinned |
  |---|---|---|
  | `~/.local/bin/trunc` (Feb 2026) | 0.2.0 | 0.4.13 |
  | `~/.local/bin/tb` (May 2026) | 0.1.12 | 0.1.32 |
  | `~/.cargo/bin/cargo-ratchet` | 1.0.3 | 1.1.7 |
  | `~/.local/bin/dotsync` (today) | 0.11.0 | 0.9.1 |

  dotsync's live site also says 0.11.0, so for dotsync it is the umbrella pin that is behind. tools.maxeonyx.com/version.json doesn't match `docs/version.json` either: it says tb 0.1.31, tdd-ratchet 1.1.6, dotsync 0.11.0.
- **Skills installed** (observed): `~/.config/opencode/skills` has `tmux-bridge` and `tdd-ratchet`. Both differ from the repos' `docs/SKILL.md`. The installed tdd-ratchet skill is an older, much longer hand-written version. There are no `trunc` or `dotsync` skills installed, so an agent there finds those tools only by chance.
- **help-test:** every tool's Cargo.toml references it, including oc and agent-harness. I confirmed this with grep and did not re-run the tests.
- **Shapes across the tools, as installed:**
  - `--version --json` gives JSON from dotsync and cargo-ratchet, but they order the keys differently. The installed trunc and tb print plain text. Current trunc and tb source does support it, so this is staleness, not a design gap.
  - Exit code for an unknown flag: trunc 2, tb 2, dotsync 1. The installed cargo-ratchet ignores the flag and fails on "not a git repository" (exit 1). Current source rejects unrecognized options. I did not run it inside a repo, because that would run the tests and write `.test-status.json`.
- **trunc, hands-on:** the task was "surface the error in a 10,000-line build log", using the installed 0.2.0 and the repo skill. `trunc error` succeeded at once. Where I hesitated:
  - The skill says the default is "first 30 + last 30". The installed `--help` says 10 each. The skill is right for current source and wrong for what is installed.
  - The installed markers (`[... matches follow ...]`) don't say how many lines were cut. Current source says `[... N lines truncated ...]`, which is better for an agent that is judging completeness.
  - `trunc "error["` fails with a clear regex parse error but gives no next step. There is no hint to escape it and no fixed-string mode.
  - `-n 5` (a habit from `head`) gets clap's generic rejection. `-f abc` gives a good error.

  Both a terminal user and a headless agent are served reasonably well by the tool itself. The weak link is getting it installed and keeping it current.
- **Unknowns:**
  - Whether other machines exist and what they have installed (no access).
  - Whether the pinned trunc actually behaves better (I didn't build or download it, to avoid changing state).
  - The tb, dotsync and agent-harness task quality (not covered this time).
- **Worth considering:**
  - One way to update every installed tool and skill from the sites, such as a documented loop over the `/releases/` URLs, would fix most of this at once. That could make this concern much smaller.
  - Agree on one `--version --json` key order and one misuse exit code across the family.
  - A hint on regex errors in trunc.
- **Since last view:** first view.
- **Noticed along the way:**
  - There's no concern for installed drift: whether the binaries and skills on the machines match the releases. It may deserve its own concern, since it decides whether any of this matters.
  - The umbrella pin, the umbrella site and the tool sites disagree on versions. That points to a separate question about whether pins and published versions agree.
