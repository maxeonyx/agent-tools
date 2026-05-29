---
title: TDD Ratchet Enforcement
type: auto
applies_to: all
checker: ".test-status.json exists AND cargo test fails (gatekeeper) AND cargo ratchet succeeds"
---

New tests must fail before they pass. Once a test passes, it must keep passing.

Tests that never failed might test the wrong thing. Tests written after the code don't prove the code is what made them pass — they could pass for accidental reasons. The ratchet catches all three failure modes: tests that don't test anything real, regressions, and silent test removal.

Without this, agents write tests that look correct but verify nothing.

## Compliance

- `.test-status.json` committed
- `cargo ratchet` is the test command
- CI runs `cargo ratchet` and checks `.test-status.json` unchanged
- Gatekeeper test panics if `cargo test` is run directly

## Bringing a tool in

1. `cargo install --git https://github.com/maxeonyx/tdd-ratchet-rs`
2. `cargo ratchet --init`
3. Add gatekeeper test
4. Update CI: `cargo ratchet` instead of `cargo test`
5. Commit `.test-status.json`

From then on: write test (fails) → commit → implement (passes) → commit. The ratchet verifies this sequence via git history.
