# Supported environments

Each tool says where it runs, and it really does run there.

## User stories

- **Max on Windows:** the suite ships Windows builds, so they should work.
- **A newcomer:** they can tell before installing whether their platform is supported.

## What it looks like here

- The standard release targets are `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc` (umbrella AGENTS.md). oc is Linux-only.
- Shell and path assumptions, such as `sh -c` and `/` separators, break Windows quietly.

## How to look

- Release assets per target: `gh release view -R maxeonyx/<repo> --json assets --jq '[.assets[].name]'`.
- Does CI build or test on Windows? Read the workflows.
- Judgment: does each front door say what is supported, and does it match the assets?
