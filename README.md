# agent-tools

Shared development workspace for [maxeonyx agent-tools](https://tools.maxeonyx.com).

Each tool is a standalone repository with its own development, CI and releases, pinned here as a submodule. This umbrella is where work that cuts across them happens.

Cross-cutting concerns live in [`crosscut/concerns/`](crosscut/concerns) as [CrossCut](https://github.com/maxeonyx/crosscut) concern files: questions worth asking again about the whole suite, each with a dated view of where things sit. They are visibility, not gates.

## Maintained tools

| Tool | Binary | Repo | Site |
| --- | --- | --- | --- |
| trunc | `trunc` | [maxeonyx/trunc](https://github.com/maxeonyx/trunc) | [trunc.maxeonyx.com](https://trunc.maxeonyx.com) |
| tmux-bridge | `tb` | [maxeonyx/tmux-bridge](https://github.com/maxeonyx/tmux-bridge) | [tmux-bridge.maxeonyx.com](https://tmux-bridge.maxeonyx.com) |
| dotsync | `dotsync` | [maxeonyx/dotsync](https://github.com/maxeonyx/dotsync) | [dotsync.maxeonyx.com](https://dotsync.maxeonyx.com) |
| tdd-ratchet | `cargo-ratchet` | [maxeonyx/tdd-ratchet-rs](https://github.com/maxeonyx/tdd-ratchet-rs) | [tdd-ratchet.maxeonyx.com](https://tdd-ratchet.maxeonyx.com) |
| agent-harness | (experimental) | [maxeonyx/agent-harness](https://github.com/maxeonyx/agent-harness) | [agent-harness.maxeonyx.com](https://agent-harness.maxeonyx.com) |
| CrossCut | `crosscut` | [maxeonyx/crosscut](https://github.com/maxeonyx/crosscut) | [maxeonyx.github.io/crosscut](https://maxeonyx.github.io/crosscut/) |

## Old tools

| Tool | Last binary | Repo | Historical site |
| --- | --- | --- | --- |
| oc (archived) | `oc` | [maxeonyx/oc](https://github.com/maxeonyx/oc) | [oc.maxeonyx.com](https://oc.maxeonyx.com) |

## Quick start

```bash
git clone git@github.com:maxeonyx/agent-tools-workspace.git
cd agent-tools-workspace
git clone git@github.com:maxeonyx/agent-tools.git at-my-feature
cd at-my-feature
git switch -c my-feature
git submodule update --init tools/trunc  # initialize only what the task needs
(cd tools/trunc && devenv test)          # a tool's own checks
crosscut list                            # the suite's concerns and how fresh each view is
```
