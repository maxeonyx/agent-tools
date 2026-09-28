# One obvious install

A newcomer follows the first install instruction they find and ends up with the current release.

## User stories

- **Someone arriving from a search result:** they copy the first command on the README or site, and it should just work and be current.
- **An agent told to install a tool:** it will use the first instruction it sees.

## What it looks like here

- Each tool's README, site and `docs/SKILL.md`, plus the umbrella site's tool cards, all state install instructions.
- Release binaries at `<site>/releases/<bin>-x86_64-linux`, `cargo install --git`, and crates.io, where only `tdd-ratchet` exists and it is an old 0.1.0.

## How to look

- Collect the first install instruction from each surface: `grep -n -A3 -i 'install' README.md docs/index.html docs/SKILL.md` in each tool, and in the umbrella's `docs/index.html` and `docs/SKILL.md`.
- For each one, what version would it install today? Check release URLs with `curl -sI`.
- Judgment: is there one obvious path per tool, and do all the surfaces agree on it?
