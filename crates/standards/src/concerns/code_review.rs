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

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "code-review",
    definition_summary: "Substantive code must have a current recorded code-quality review attestation.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: Some("docs/reviews/code-quality.json"),
    applies_to_workspace: true,
    applicability_note: "Applies to tool repos and to substantive workspace-owned code such as standards and shared crates.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{concerns, workspace_root};

    #[test]
    fn code_review() {
        let mut failures =
            concerns::review_attestation_failures("docs/reviews/code-quality.json", NOT_APPLICABLE);

        if let Some(failure) = concerns::review_attestation_failure_for_repo(
            "workspace",
            &workspace_root(),
            "docs/reviews/code-quality.json",
        ) {
            failures.push(failure);
        }

        if !failures.is_empty() {
            panic!("code-review non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
