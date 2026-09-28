# Explains itself

In use, each tool tells you how to use it: help with runnable examples, and errors that say what happened, why, and what to do next.

## User stories

- **An agent with only the binary:** it reads `--help` and the errors, never the source. Every unclear message costs a retry.
- **Max in a terminal:** the first-time path is obvious, and destructive operations are signposted.

## What it looks like here

- CLI tools: `--help` with an Examples section, examples tested by help-test, and error messages with context and a next step.
- Libraries: rustdoc and the error types they return.

## How to look

- `grep -l help-test tools/*/Cargo.toml` shows which tools test their help examples.
- For one or two tools per refresh, rotating through them: run `--help`, one subcommand's `--help`, and two deliberate mistakes such as an unknown flag or a missing file. Read the output as an agent would.
- Say which tools this map covers, so the next refresh can take others.
