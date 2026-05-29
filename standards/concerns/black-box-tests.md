---
title: Black-box Test Architecture
type: auto
applies_to: all
checker: "tests/ directory exists AND tests use assert_cmd AND no mod imports from src/"
---

Tests spawn the binary and check behavior from outside. No internal imports.

Tests coupled to implementation break when you refactor, even if behavior is unchanged. Black-box tests verify the contract — input goes in, correct output comes out. They survive refactoring because they don't know or care about internal structure.

For CLI tools, the binary IS the interface. `Command::cargo_bin("trunc")`, pipe input, assert on stdout/stderr/exit code. That's it.

## Relationship to injectable-io

Both are important. They're complementary, not competing:
- **Black-box tests** prove the binary works end-to-end (slow, but definitive)
- **Injectable-io tests** cover edge cases and error paths fast (milliseconds, in-process)

A mature tool has both. Black-box tests are the acceptance tests. Injectable-io tests are where you do the detailed work. The black-box tests are slow checks; the injectable-io tests are fast checks.
