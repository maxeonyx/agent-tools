//! # Black-box Test Quality
//!
//! Tests should verify the tool from the outside, through the shipped binary and
//! public behavior. File presence is enforced by `tests-present`; this concern
//! requires a current agentic review that the tests prove the right contract.
//!
//! High-quality black-box tests cover successful workflows, failure paths, and
//! user-visible output without importing internal modules for behavior
//! assertions.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the tool's tests as public behavior tests, separate from the mechanical
question of whether integration tests exist.

Required review method:
1. Run the tool's black-box or integration test command.
2. Read the tests that spawn the binary or exercise the documented public
   command surface.
3. Map the covered scenarios to the tool's primary user workflows and failure
   modes.
4. Produce findings with test file/line references. If there are no findings,
   list the successful workflows, failure paths, and output assertions covered.

Check for:
1. Tests execute the built CLI binary or documented public command surface.
2. Successful workflows users rely on are covered end to end.
3. Failure paths cover bad input, missing resources, invalid state, and boundary
   failures where relevant.
4. Assertions cover user-visible stdout, stderr, exit status, and filesystem
   effects where relevant.
5. Behavior assertions avoid importing internal modules or reaching into private
   implementation details.
6. The tests would still be meaningful after a correct internal refactor.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "black-box-test-quality",
    definition_summary:
        "Each tool must have a current review attestation for black-box test quality.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to shipped CLI tools; the workspace root is not itself a shipped binary.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn black_box_test_quality() {
        let failures =
            concerns::review_attestation_failures("black-box-test-quality", NOT_APPLICABLE);

        if !failures.is_empty() {
            panic!(
                "black-box-test-quality non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
