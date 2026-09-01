# Agent guide

## TDD ratchet — read before testing

Run `cargo ratchet`, not plain `cargo test`. A new test must be red when first introduced and recorded as `pending`; that expected red test keeps CI green. A new test must not pass when first introduced—doing so makes the ratchet and CI red. Commit and push the red test, wait for the trusted ledger workflow's bot commit, then implement, rerun the ratchet, and push the green implementation so the bot can record the promotion to `passing`.
