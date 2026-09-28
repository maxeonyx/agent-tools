# What these concerns look at

The umbrella and its submodules are most of it, but not all of it. Several concerns depend on things that are not in any repository: the live sites, the release binaries, crates.io, the GitHub account's settings, and the binaries actually installed on the machines where the tools get used. A refresh that cannot reach one of these should say so in its view. That is a fact to report, not an error.

## Repositories

All of these are public repositories under the `maxeonyx` GitHub account. Each has its own CI, releases and Pages site. In a full umbrella checkout they are pinned as submodules.

- **agent-tools** (this umbrella), at the checkout root. It is the coordination point: submodule pins, `docs/version.json`, the site at tools.maxeonyx.com (`docs/`), and these concerns. It has no code of its own beyond two site scripts. Only Max installs and uses these tools today (Max, 2026-09-28).
- **trunc** (`tools/trunc`, maxeonyx/trunc): head/tail truncation for pipe output. Stateless.
- **tmux-bridge** (`tools/tb`, maxeonyx/tmux-bridge, binary `tb`): lets agents drive the user's live tmux server. Its tests talk to a real tmux server.
- **dotsync** (`tools/dotsync`, maxeonyx/dotsync): an agent-first dotfile manager built on jj-lib and gix. It writes into `$HOME` and keeps a hidden repo in `~/.local/share/dotsync/repo`. It is the only tool that holds durable user state, and it is by far the most active repository.
- **tdd-ratchet** (`tools/tdd-ratchet`, maxeonyx/tdd-ratchet-rs, binary `cargo-ratchet`): the trust root for every sibling's CI gate and ledger. It is also published on crates.io as `tdd-ratchet`, but only as 0.1.0.
- **agent-harness** (`tools/agent-harness`, maxeonyx/agent-harness): an experimental workbench, mostly design and process documents. It releases binaries to Pages.
- **oc** (`tools/oc`, maxeonyx/oc): archived on GitHub. Its v0.3.20 binary is still served from oc.maxeonyx.com. It does not build from source as it stands, because its path dependencies were removed from the umbrella.
- **help-test** (`libraries/help-test`, maxeonyx/help-test): a shared dev-dependency that runs `--help` examples. It is consumed by git tag.
- **crosscut** (`tools/crosscut`): CrossCut itself. It is out of scope for these concerns and handled separately.

## Not repositories

- **Sites**: `<tool>.maxeonyx.com` and tools.maxeonyx.com, served by GitHub Pages. Each serves `/version.json`, and every tool site except oc's serves `/releases/<binary>-x86_64-linux`.
- **crates.io**: the `tdd-ratchet` crate, owned by maxeonyx.
- **Machines that run the tools**: Max's Linux dev machine (installs in `~/.local/bin`, `~/.cargo/bin`, and skills in `~/.config/opencode/skills`). There may be others, including Windows, since issue agent-tools#3 wants wmux development there. That is unknown.
- **The GitHub account**: one login owns every repository, the Pages domains, the releases, the Actions tokens and the crates.io crate.
