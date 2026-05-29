# Fast and Slow Check Separation

Fast checks give immediate feedback. Slow checks run separately.

## Why

Tight feedback loops matter. If every check takes 30 seconds because slow tests run alongside fast ones, developers (and agents) stop running checks between changes. Separating fast from slow means you can run fast checks constantly — after every edit — and save slow checks for before-push verification.

**Fast** (milliseconds, no real IO): lint, format, in-memory integration tests, pure logic tests.
**Slow** (seconds+): tests that create directory structures, launch subprocesses, spawn binaries, or need external services.

Build time is excluded from this calculation — it's a prerequisite for both.

## What compliance looks like

- CI has distinct fast and slow stages
- Fast checks can be run independently without triggering slow checks
- Fast checks include: fmt, clippy, in-process tests (the injectable-io tests)
- Slow checks include: black-box tests (subprocess-spawning), any tests needing real filesystem/tmux/network
- The tool's test suite is organized so you can run fast tests alone

## How to bring a tool into compliance

1. Identify which tests are fast (pure logic, in-memory) vs slow (spawn process, touch filesystem)
2. Organize them so they can be run separately (separate test files, feature flags, or test name conventions)
3. Update CI to run fast checks first, slow checks after
4. Document how to run each tier locally

## How to maintain

When adding tests, put them in the right tier. If a test spawns a process or touches real IO, it's slow. If not, it's fast. The injectable-io concern helps move tests from slow to fast by eliminating real IO.
