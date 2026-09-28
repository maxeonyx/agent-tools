# Fast feedback

The common edit to each project gets a trustworthy signal in seconds.

## User stories

- **An agent iterating on a fix:** a two-minute test loop means fewer, larger, riskier steps.
- **Max:** agents that wait on slow checks run up cost and lose context.

## What it looks like here

- Rust tools: `cargo test` on a warm build, or the tool's documented fast check.
- Sites: a local preview.
- Test suites that need external services, such as a live tmux server, are slower and flakier by nature.

## How to look

- In each tool: `cargo build -q && time cargo test -q` twice, recording the warm run. First note `git status`, and restore anything the run changes. tb's suite uses the live tmux server, so check `tmux ls` before and after.
- Read the AGENTS.md: is the fast check named, and is it what an agent would run?
