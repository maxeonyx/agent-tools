# Consistent conventions

Sibling tools behave alike where it helps, so knowing one tool means knowing the others.

## User stories

- **An agent using several tools in one session:** the same flag spellings, `--version --json` shape, exit codes and error style everywhere.
- **Max:** a pattern improved in one tool should be recognisable in the rest.

## What it looks like here

- The CLI surface: `--help` layout, `--version --json`, exit codes on misuse, and long flags in examples.
- Repository shape: devenv, CI templates, and the site style.

## How to look

- For each tool: `<bin> --version --json`, and the exit code of `<bin> --no-such-flag`. Compare them side by side.
- `diff` the `ci.yml` of each pair of tools, to see where the copies have drifted.
- Judgment: which differences are deliberate, and which are drift that could be shared structurally, for example as one reusable workflow?
