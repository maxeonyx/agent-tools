# Checks before landing

Proposed changes are checked before they land, the triggers are right, and the checks are worth their cost.

## User stories

- **Max:** a PR that breaks a tool should say so before it merges.
- **An agent:** it gets feedback on its PR without having to ask.

## What it looks like here

- Tools: a PR-triggered ledger run, and an integration run that must be dispatched.
- CrossCut: `ci.yml` runs on push and on PRs.
- The umbrella: Pages only, with no checks on PRs.

## How to look

- `grep -n -A6 '^on:' tools/*/.github/workflows/*.yml .github/workflows/*.yml`.
- Recent run times: `gh run list -R maxeonyx/<repo> -L 20 --json workflowName,conclusion,createdAt,updatedAt`.
- Judgment: does each repository's check run when changes are proposed, and is its latency proportionate?
