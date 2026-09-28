# Staying current

Installed copies of each tool get newer without anyone having to remember.

## User stories

- **Max, on any machine:** the tools he uses every day should not quietly be months old. On 2026-09-28 the trunc on his PATH was 0.2.0 while 0.4.13 was released.
- **An agent using a tool:** it should get the behaviour the docs describe, not an old binary's.

## What it looks like here

- CLI tools: a self-update (check on start, replace in place) or an install through a package manager that updates.
- Skills: the installed skill tracks the binary it describes.
- Libraries: consumers get an update prompt, for example a bot or a pinned-tag check.

## How to look

- In each tool's source, look for an update mechanism: `grep -rn -i 'update' src/ | head`, and read what you find.
- On this machine: `command -v <bin>` and `<bin> --version` against the latest release (`gh release view -R maxeonyx/<repo> --json tagName`).
- Judgment: for tools without a mechanism, what is the lightest way to add one, and could one shared implementation serve every tool?
