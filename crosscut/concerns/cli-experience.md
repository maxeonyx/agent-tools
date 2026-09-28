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
