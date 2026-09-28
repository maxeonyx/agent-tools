# One-step release

From 'this is ready' to published, versioned artifacts takes one step, with no hand work to remember.

## User stories

- **Max:** releasing a fix should not need a checklist in his head.
- **An agent finishing a change:** it can ship it by following one documented step.

## What it looks like here

- Tools: a dispatched integration workflow that merges, tags, builds and publishes release assets and the site.
- Libraries: a tag.
- The umbrella: a pointer bump plus `generate-version-json.py` plus a Pages deploy, which is three steps.

## How to look

- Read each tool's `ci.yml` and `pages.yml`, and its AGENTS.md release section.
- `gh release view -R maxeonyx/<repo> --json tagName,assets --jq '.tagName, [.assets[].name]'`.
- Judgment: count the hand steps from a ready branch to published, including version bumps.
