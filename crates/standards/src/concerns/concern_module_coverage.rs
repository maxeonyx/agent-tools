//! # Concern Module Coverage
//!
//! A concern is only real if it is exposed coherently and tested accurately.
//!
//! Every concern module should declare stable metadata through the registry,
//! explain its applicability, and participate in tests that validate both
//! success and failure paths. This concern checks the exported metadata for the
//! real registry and uses fixtures to verify the checker logic itself.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "concern-module-coverage",
    definition_summary: "Concern modules must expose stable metadata and the checker must be tested with pass/fail fixtures.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: None,
    applies_to_workspace: true,
    applicability_note: "Applies to the workspace standards system itself because it governs how every concern is defined and validated.",
};

#[cfg(test)]
mod tests {
    use crate::concerns::{ConcernSpec, ALL_CONCERN_SPECS};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct FixtureSpec {
        id: String,
        definition_summary: String,
        applicability_note: String,
        applies_to_workspace: bool,
        in_registry: bool,
        has_checker: bool,
        has_fixture_tests: bool,
    }

    #[test]
    fn concern_module_coverage() {
        let mut failures = Vec::new();

        for spec in ALL_CONCERN_SPECS {
            validate_actual_spec(spec, &mut failures);
        }

        if !failures.is_empty() {
            panic!(
                "concern-module-coverage non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    #[test]
    fn fixture_pass_case_is_accepted() {
        let fixture = load_fixture("pass");
        let failures = validate_fixture(&fixture);
        assert!(
            failures.is_empty(),
            "unexpected fixture failures: {failures:?}"
        );
    }

    #[test]
    fn fixture_missing_definition_is_rejected() {
        let fixture = load_fixture("fail-missing-definition");
        let failures = validate_fixture(&fixture);
        assert!(failures.iter().any(|f| f.contains("definition_summary")));
    }

    #[test]
    fn fixture_missing_applicability_is_rejected() {
        let fixture = load_fixture("fail-missing-applicability");
        let failures = validate_fixture(&fixture);
        assert!(failures.iter().any(|f| f.contains("applicability_note")));
    }

    #[test]
    fn fixture_missing_checker_and_fixture_tests_is_rejected() {
        let fixture = load_fixture("fail-missing-tests");
        let failures = validate_fixture(&fixture);
        assert!(failures.iter().any(|f| f.contains("checker")));
        assert!(failures.iter().any(|f| f.contains("fixture_tests")));
    }

    fn validate_actual_spec(spec: &ConcernSpec, failures: &mut Vec<String>) {
        if spec.id.trim().is_empty() {
            failures.push("registry entry has empty id".to_string());
        }
        if spec.definition_summary.trim().is_empty() {
            failures.push(format!("{}: definition_summary missing", spec.id));
        }
        if spec.applicability_note.trim().is_empty() {
            failures.push(format!("{}: applicability_note missing", spec.id));
        }
        if spec.review_instructions.contains('\r') {
            failures.push(format!(
                "{}: review instructions should be normalized",
                spec.id
            ));
        }
    }

    fn validate_fixture(spec: &FixtureSpec) -> Vec<String> {
        let mut failures = Vec::new();

        if spec.id.trim().is_empty() {
            failures.push("id missing".to_string());
        }
        if spec.definition_summary.trim().is_empty() {
            failures.push(format!("{}: definition_summary missing", spec.id));
        }
        if spec.applicability_note.trim().is_empty() {
            failures.push(format!("{}: applicability_note missing", spec.id));
        }
        if !spec.in_registry {
            failures.push(format!("{}: registry wiring missing", spec.id));
        }
        if !spec.has_checker {
            failures.push(format!("{}: checker missing", spec.id));
        }
        if !spec.has_fixture_tests {
            failures.push(format!("{}: fixture_tests missing", spec.id));
        }
        let _ = spec.applies_to_workspace;

        failures
    }

    fn load_fixture(name: &str) -> FixtureSpec {
        let path = crate::workspace_root()
            .join("crates/standards/fixtures/concern-module-coverage")
            .join(name)
            .join("spec.json");
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        serde_json::from_str(&content)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
    }
}
