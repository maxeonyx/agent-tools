# Help Text Standards

Consistent `--help` output across all tools.

## Why

These tools form a suite. Users (and agents) encounter multiple tools and expect consistency. Inconsistent flag naming, formatting, or description quality makes the suite feel accidental rather than designed. Good help text is often the only documentation a user reads.

## What compliance looks like

- `--help` output is clear and complete
- Flag conventions match across tools (e.g., `-v`/`--verbose` if applicable, not `-V` in one and `--verbose` in another)
- Descriptions are specific and accurate (not placeholder text)
- Examples section exists for non-trivial usage
- Output follows clap derive conventions consistently

## How to bring a tool into compliance

1. Run `<tool> --help` and read it as a new user would
2. Compare with sibling tools' help text for consistency
3. Fix flag names, descriptions, and formatting
4. Add examples for common usage patterns
5. Have an agent review: "is this help text useful to someone who has never used this tool?"

## How to maintain

When adding new flags or subcommands, review the help text in context of the full `--help` output. Don't just add a flag — make sure it fits the existing structure and naming conventions.
