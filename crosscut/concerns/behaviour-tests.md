# Tests that catch real breakage

Each tool's tests exercise its behaviour from the outside, and turn red when that behaviour breaks.

## User stories

- **An agent changing code cold:** the tests are what tells it that it broke something else.
- **Max:** green should mean the tool works, not that the code compiles.

## What it looks like here

- CLI tools: black-box tests that spawn the binary (`assert_cmd`) and check its output, exit codes and files.
- Libraries: tests against the public API.

## How to look

- `ls tools/*/tests libraries/*/tests`, and skim each tool's test names.
- For one tool per refresh: in a scratch copy (`cp -r tools/<t> $TMP/`, never the checkout), break one user-visible behaviour, and see whether `cargo test` catches it. Say which tool you tried.
