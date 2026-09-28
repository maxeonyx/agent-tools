# Recoverable state

Whatever would hurt to lose can be got back, and someone has actually done it.

## User stories

- **Max, after a disk dies:** his dotfiles and their history, managed by dotsync, come back.
- **Any tool user:** a crash mid-operation does not leave state that cannot be recovered.

## What it looks like here

- dotsync holds durable user state: the hidden repo in `~/.local/share/dotsync/repo` and its remote.
- Most tools are stateless (trunc, tb, crosscut) or hold only repository state. Those are n/a, and each row should say why.

## How to look

- Read dotsync's DESIGN and AGENTS for its recovery story: remote, abort and convergence.
- Is there a documented restore onto a fresh machine, and has one been done? That is a question for Max if the docs don't say.
- For every other project, confirm it holds no state beyond git, and mark it `n/a` with the reason.
