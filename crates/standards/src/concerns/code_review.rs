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
Perform a fresh code-quality review of the repo as if you were deciding whether
to maintain it next month.

Required review method:
1. Read the entry points, core modules, tests, and error paths before judging.
2. Run the normal fast test command and at least one representative tool command.
3. Trace one successful workflow and one failure workflow from CLI boundary to
   core logic and back.
4. Produce findings by concrete file/line references. If there are no findings,
   say so and name the highest-risk paths you inspected.

Check for:
1. Simpler shapes that would remove incidental complexity.
2. Modules or flows that are tangled, oversized, or hard to change safely.
3. Cleverness where boring, explicit Rust would be clearer.
4. Type boundaries, ownership, and error types that either clarify or obscure
   the domain.
5. Unnecessary coordination, global state, hidden IO, or concurrency risks.
6. Functions over 50 lines, boolean parameters, stringly-typed interfaces, dead
   code, stale abstractions, and unnecessary wrappers.
7. Tests that assert behavior at the right boundary and would catch realistic
   regressions.

Reference the `thermonuclear-review` skill for the full standard.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "code-review",
    definition_summary: "Substantive code must have a current recorded code-quality review attestation.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to tool repos and to substantive workspace-owned code such as standards and shared crates.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{concerns, workspace_root};

    #[test]
    fn code_review() {
        let mut failures = concerns::review_attestation_failures("code-review", NOT_APPLICABLE);

        if let Some(failure) = concerns::review_attestation_failure_for_repo(
            "workspace",
            &workspace_root(),
            "code-review",
        ) {
            failures.push(failure);
        }

        if !failures.is_empty() {
            panic!("code-review non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
