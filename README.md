# agent-tools

Shared development workspace for [maxeonyx agent-tools](https://tools.maxeonyx.com).

All tools are developed from this workspace. Individual repos exist for CI, releases, and Pages.

## Tools

| Tool | Binary | Repo | Site |
|------|--------|------|------|
| trunc | `trunc` | [maxeonyx/trunc](https://github.com/maxeonyx/trunc) | [trunc.maxeonyx.com](https://trunc.maxeonyx.com) |
| tmux-bridge | `tb` | [maxeonyx/tmux-bridge](https://github.com/maxeonyx/tmux-bridge) | [tmux-bridge.maxeonyx.com](https://tmux-bridge.maxeonyx.com) |
| dotsync | `dotsync` | [maxeonyx/dotsync](https://github.com/maxeonyx/dotsync) | [dotsync.maxeonyx.com](https://dotsync.maxeonyx.com) |
| tdd-ratchet | `cargo-ratchet` | [maxeonyx/tdd-ratchet-rs](https://github.com/maxeonyx/tdd-ratchet-rs) | [tdd-ratchet.maxeonyx.com](https://tdd-ratchet.maxeonyx.com) |
| oc | `oc` | [maxeonyx/oc](https://github.com/maxeonyx/oc) | [oc.maxeonyx.com](https://oc.maxeonyx.com) |

## Quick start

```bash
git clone --recurse-submodules git@github.com:maxeonyx/agent-tools.git
cd agent-tools
cargo check --all
```
