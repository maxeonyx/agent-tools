---
title: Injectable IO for In-Process Integration Tests
type: auto
applies_to: all
checker: "core logic parameterized over IO providers AND in-process integration tests exist using in-memory implementations"
---

The biggest architectural investment in this list. IO boundaries (filesystem, network, subprocess) go through traits. Tests provide in-memory implementations. Full application logic runs in-process at unit-test speed.

This matters more than any other testing concern because it determines whether your test suite gives millisecond feedback or second feedback. Millisecond tests get run after every edit. Second tests get run before push. The difference in development speed is enormous.

## The pattern

Instead of:
```rust
fn sync_files(src: &Path, dest: &Path) -> Result<()> {
    for entry in std::fs::read_dir(src)? { ... }
}
```

You have:
```rust
fn sync_files(fs: &dyn Filesystem, src: &Path, dest: &Path) -> Result<()> {
    for entry in fs.read_dir(src)? { ... }
}
```

Tests use `InMemoryFs`. Production uses `RealFs`. The logic is identical. The IO is swapped.

## What this enables

- Edge cases tested without creating real temp directories
- Error paths tested without provoking real filesystem errors
- Hundreds of integration scenarios running in milliseconds
- Black-box tests demoted to smoke tests (do they still work end-to-end?)
- Refactoring with confidence (fast tests mean you run them constantly)

## Current state

None of these tools do this yet. Most are small enough that black-box tests are tolerable. But as tools grow (oc, dotsync especially), the slow-test bottleneck will become painful. This is the architectural remedy.

## Bringing a tool in

This is a refactoring effort, not a feature addition. For each tool:
1. Identify IO boundaries (where does it touch the real world?)
2. Extract traits for those boundaries
3. Implement in-memory versions
4. Move existing test logic to use in-memory providers
5. Keep a few black-box tests as smoke tests
