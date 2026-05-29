//! # Code Quality Review
//!
//! Each tool must have a recorded thermonuclear review — structural critique,
//! not just "does it work." Uses the `thermonuclear-review` skill.
//!
//! This is a one-time gate after initial adoption into the workspace,
//! re-triggered when the tool changes significantly.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Perform a thermonuclear code review of the tool.
Check for:
1. Code judo: are there simpler shapes that eliminate incidental complexity?
2. Size and spaghetti: are modules and flows too tangled or too large?
3. Boring over clever: does the implementation choose straightforward patterns?
4. Type cleanliness and ownership clarity
5. Concurrency model correctness and unnecessary coordination
6. Functions over 50 lines that should be split
7. Boolean parameters and stringly-typed interfaces
8. Dead code, stale abstractions, and unnecessary wrappers

Reference the `thermonuclear-review` skill for the full standard.
"#;

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn code_review() {
        let failures = concerns::review_attestation_failures(
            "docs/reviews/code-quality.json",
            NOT_APPLICABLE,
        );

        if !failures.is_empty() {
            panic!("code-review non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
