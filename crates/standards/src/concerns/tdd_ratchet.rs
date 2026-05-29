//! # TDD Ratchet Enforcement
//!
//! New tests must fail before they pass. Once a test passes, it must keep
//! passing.
//!
//! Tests that never failed might test the wrong thing. Tests written after the
//! code don't prove the code is what made them pass — they could pass for
//! accidental reasons. The ratchet catches tests that don't test anything real,
//! regressions, and silent test removal.
//!
//! To comply, a tool should commit `.test-status.json`, use `cargo ratchet` as
//! the test command, run it in CI, and keep a gatekeeper test so `cargo test`
//! alone is not the development path.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{TOOLS, tools_dir};

    #[test]
    fn tdd_ratchet() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let path = tools_dir().join(tool).join(".test-status.json");
            if !path.exists() {
                failures.push(format!("{tool}: .test-status.json missing"));
            }
        }

        if !failures.is_empty() {
            panic!("tdd-ratchet non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
