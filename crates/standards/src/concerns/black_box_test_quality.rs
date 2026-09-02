//! # Black-box Test Quality
//!
//! Tests should verify behavior from the outside, through shipped binaries,
//! public commands, built artifacts, repository workflows, and observable
//! filesystem or network effects. File presence is enforced by `tests-present`;
//! this concern requires a current agentic review that the tests prove the
//! right contract.
//!
//! High-quality black-box tests cover successful workflows, failure paths, and
//! user-visible output without importing internal modules for behavior
//! assertions.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the repository's tests as public behavior tests, separate from the
mechanical question of whether integration tests exist. For the umbrella,
review the concern checkers and their evidence boundaries as the public
standards-enforcement surface.

Required review method:
1. Run the repository's black-box or integration test command. For the
   umbrella, run `cargo ratchet` or the equivalent offline `devenv test` entrypoint.
2. Read the tests that spawn the binary or exercise the documented public
   command, artifact, workflow, filesystem, or rendered-output surface.
3. Map the covered scenarios to the repository's primary user or enforcement
   workflows and failure modes.
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
7. Concern checkers use executable or rendered evidence when practical;
   source-marker inspection is only a structural backstop, not a substitute
   for an observable behavior test.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "black-box-test-quality",
    definition_summary: "Each applicable repository must have a current review attestation for black-box test quality.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to shipped CLI tools and to the umbrella standards-enforcement surface, whose concerns execute commands, inspect built artifacts, and validate external behavior.",
};

#[cfg(test)]
mod tests {
    use super::{NOT_APPLICABLE, SPEC};
    use crate::{concerns, workspace_root};

    #[test]
    fn workspace_is_in_black_box_quality_scope() {
        assert!(
            SPEC.applies_to_workspace,
            "umbrella concern checkers need black-box quality review"
        );
    }

    #[test]
    fn black_box_test_quality() {
        let mut failures =
            concerns::review_attestation_failures("black-box-test-quality", NOT_APPLICABLE);

        if let Some(failure) = concerns::review_attestation_failure_for_repo(
            "workspace",
            &workspace_root(),
            "black-box-test-quality",
        ) {
            failures.push(failure);
        }

        if !failures.is_empty() {
            panic!(
                "black-box-test-quality non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
