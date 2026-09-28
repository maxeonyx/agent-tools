# Narrow release authority

You know what code and which credentials can publish a release that users' machines will run, and it is as little as possible.

## User stories

- **Anyone who installs a tool:** they trust whatever the release job ran.
- **Max:** one account owns everything. The less code with publishing rights, the smaller that single point of failure.

## What it looks like here

- Tool CI that clones tdd-ratchet `main` at run time, inside the job that merges and releases.
- Third-party actions pinned by tag or by SHA.
- Binaries downloaded with `curl` and no checksum.

## How to look

- `grep -n 'git clone\|uses:' tools/*/.github/workflows/*.yml`, and whether each `uses:` is pinned to a SHA.
- Repository settings: `gh api repos/maxeonyx/<repo> --jq '{allow_merge_commit, allow_squash_merge, delete_branch_on_merge}'` and branch protection.
- Judgment: for each repository, what could change the bytes of the next release without a reviewed commit here?
