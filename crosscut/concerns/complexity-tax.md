# The complexity tax

Every part still present has a discoverable reason to be there, and anything unneeded lives only in git history.

## User stories

- **Max:** the current state should be minimal. Cleanup should be repeatable because reasons can be found, not rediscovered.
- **A fresh agent:** it should not have to maintain machinery whose reason has gone. The umbrella's retired concern ratchet is the example.

## What it looks like here

- The reasons live in VISION, design notes, AGENTS.md and commit messages. The system's goal sits outside the software, so these records go stale without anything in the code changing.
- The candidates are workarounds, compatibility layers, copied templates, dead modules, and process text that polices rather than explains, including this `crosscut/` directory itself.

## How to look

- This concern is a different kind from the others. Its map rows say whether each project's reasons are discoverable at all: is there a VISION or design record, and does it match what exists?
- The substance is a short list of specific things that no longer seem to earn their place. For each, give where it is, the reason it seemed to exist, and why that reason looks gone. Look at one or two projects per refresh, and say which.
- Treat each item as a candidate for deletion, not as a defect.
