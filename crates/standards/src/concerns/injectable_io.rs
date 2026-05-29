//! # Injectable IO for In-Process Integration Tests
//!
//! Nontrivial tools with multiple IO boundaries should push side effects to the
//! edges and support fast in-process tests by injecting boundary
//! implementations at construction time.
//!
//! IO boundaries (filesystem, network, subprocess) go through traits. Tests
//! provide in-memory implementations. Full application logic runs in-process at
//! unit-test speed.
//!
//! References: `programming` skill (side effects at the edges), `tests` skill
//! (Tier 2: fast in-process integration tests).

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["trunc", "tb", "tdd-ratchet"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the tool's architecture for injectable IO:
1. Does the tool push IO to the edges rather than mixing it into core logic?
2. Are there trait-based or equivalent boundaries around filesystem, network, or subprocess IO?
3. Do tests use in-memory or fake implementations of those boundaries?
4. Is the pure core testable independently of real IO?

Reference the `programming` skill and `tests` skill Tier 2 guidance.
"#;

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn injectable_io() {
        let failures = concerns::review_attestation_failures(
            "docs/reviews/injectable-io.json",
            NOT_APPLICABLE,
        );

        if !failures.is_empty() {
            panic!("injectable-io non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
