# Lifecycle is clear

For each project it is obvious whether it is maintained, experimental, frozen or archived, and what that implies.

## User stories

- **A newcomer:** they know whether to depend on it.
- **An agent:** it knows whether to invest in changes, or leave a frozen project alone.

## What it looks like here

- The umbrella README's "Maintained tools" and "Old tools" tables.
- The GitHub archive flag, and the wording on each front door.
- Experimental projects (agent-harness) saying so up front.

## How to look

- `gh repo view maxeonyx/<repo> --json isArchived,description`.
- Compare the README tables, the umbrella site, and each tool's own front door.
- Judgment: does every surface agree about each project's status?
