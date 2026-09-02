# Fixture

Run cargo ratchet, not plain cargo test. A new test must be red when first introduced and recorded as pending. Push the red test and wait for the trusted ledger workflow's bot commit before implementing it. Push the green implementation and wait for the bot to record passing.
