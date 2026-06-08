//! # Concern Module Coverage
//!
//! A concern is only real if it is exposed coherently and tested accurately.
//!
//! Every concern module should declare stable metadata through the registry,
//! explain its applicability, have a checker, and validate checker behavior
//! with adjacent pass/fail fixtures unless fixtures are explicitly impractical.
//! This concern checks the exported metadata for the real registry and uses
//! adjacent fixtures to verify the checker logic itself.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "concern-module-coverage",
    definition_summary: "Concern modules must expose stable metadata and the checker must be tested with pass/fail fixtures.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to the workspace standards system itself because it governs how every concern is defined and validated.",
};

#[cfg(test)]
mod tests {
    use crate::concerns::{ConcernSpec, ALL_CONCERN_SPECS};
    use serde::Deserialize;
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    const FIXTURES_IMPRACTICAL: &[(&str, &str)] = &[
        (
            "auto-update",
            "The current checker inspects live tool source trees for updater wiring; useful pass/fail fixtures need a synthetic tool-repo harness.",
        ),
        (
            "auto-update-integration",
            "The current checker inspects live updater integration tests across tool repos; fixtures would need a synthetic tool-repo harness.",
        ),
        (
            "tests-present",
            "The current checker inspects live tool Cargo manifests and tests directories; fixtures need a synthetic tool-repo harness.",
        ),
        (
            "black-box-test-quality",
            "The checker validates centralized attestation state and live git commits; fixture coverage belongs in review-attestation unit tests.",
        ),
        (
            "code-review",
            "The checker validates centralized attestation state and live git commits; fixture coverage belongs in review-attestation unit tests.",
        ),
        (
            "code-standards",
            "The current checker shells out to formatting policy over live tool repos; isolated fixtures need command-runner evidence injection first.",
        ),
        (
            "devenv-check",
            "The current checker inspects live repo environment files and build commands; fixtures need a synthetic repo harness.",
        ),
        (
            "error-messages",
            "The checker validates centralized attestation state and live git commits; fixture coverage belongs in review-attestation unit tests.",
        ),
        (
            "fast-slow-checks",
            "The current checker inspects live tool scripts and command configuration; fixtures need a synthetic tool-repo harness.",
        ),
        (
            "help-text",
            "The checker combines live tool manifest/test inspection with attestation state; fixtures need a synthetic tool-repo harness.",
        ),
        (
            "injectable-io",
            "The checker validates centralized attestation state and live git commits; fixture coverage belongs in review-attestation unit tests.",
        ),
        (
            "landing-page",
            "The current checker inspects live website files and will move to live-site evidence; fixtures should wait for the shared evidence harness.",
        ),
        (
            "latest-ci-green",
            "The current checker depends on live GitHub CI state; fixtures should be added after CI evidence is injectable.",
        ),
        (
            "opencode-skill",
            "The current checker inspects live tool skill files; fixtures need a synthetic tool-repo harness.",
        ),
        (
            "pinned-main-parity",
            "The current checker compares live git branches and commits; fixtures need git-repo test harness support.",
        ),
        (
            "release-freshness",
            "The current checker depends on live GitHub release metadata; fixtures should be added after release evidence is injectable.",
        ),
        (
            "release-pipeline",
            "The current checker inspects live workflow files and release metadata; fixtures should wait for shared release evidence.",
        ),
        (
            "standalone-publishability",
            "The current checker inspects live tool path dependencies and build shape; fixtures need a synthetic repo harness.",
        ),
        (
            "tdd-ratchet",
            "The current checker runs live cargo ratchet commands per tool; fixtures need command-runner evidence injection first.",
        ),
        (
            "version-artifacts",
            "The current checker inspects live version files and command output; fixtures should wait for shared version evidence.",
        ),
        (
            "vision-and-process",
            "The current checker inspects live process documentation and repo files; fixtures need a synthetic workspace harness.",
        ),
        (
            "website-install-links",
            "The current checker inspects live website/install metadata and will share release/version evidence; fixtures should wait for that harness.",
        ),
        (
            "workspace-routing",
            "The current checker inspects live workspace routing and submodule layout; fixtures need a synthetic workspace harness.",
        ),
    ];

    #[derive(Deserialize)]
    struct FixtureSpec {
        id: String,
        definition_summary: String,
        applicability_note: String,
        applies_to_workspace: bool,
        in_registry: bool,
        has_checker: bool,
        has_adjacent_pass_fixture: bool,
        has_adjacent_fail_fixture: bool,
        fixtures_impractical_reason: String,
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
        assert!(failures.iter().any(|f| f.contains("fixtures")));
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

        let module_path = module_source_path(spec.id);
        let module_content = std::fs::read_to_string(&module_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", module_path.display()));
        if !has_checker(&module_content) {
            failures.push(format!("{}: checker missing", spec.id));
        }

        let fixture_dir = adjacent_fixture_dir(spec.id);
        let has_pass_fixture = fixture_dir.join("pass").is_dir();
        let has_fail_fixture = has_fail_fixture(&fixture_dir);
        let fixtures_impractical = fixtures_impractical_reason(spec.id).is_some();
        if !(has_pass_fixture && has_fail_fixture) && !fixtures_impractical {
            failures.push(format!(
                "{}: adjacent pass/fail fixtures missing and no explicit fixtures-impractical reason recorded",
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
        let has_adjacent_fixtures =
            spec.has_adjacent_pass_fixture && spec.has_adjacent_fail_fixture;
        if !has_adjacent_fixtures && spec.fixtures_impractical_reason.trim().is_empty() {
            failures.push(format!(
                "{}: adjacent pass/fail fixtures missing and no explicit fixtures-impractical reason recorded",
                spec.id
            ));
        }
        let _ = spec.applies_to_workspace;

        failures
    }

    fn load_fixture(name: &str) -> FixtureSpec {
        let path = crate::workspace_root()
            .join("crates/standards/src/concerns/concern_module_coverage/fixtures")
            .join(name)
            .join("spec.json");
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        serde_json::from_str(&content)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
    }

    fn module_source_path(concern_id: &str) -> PathBuf {
        crate::workspace_root()
            .join("crates/standards/src/concerns")
            .join(format!("{}.rs", concern_module_name(concern_id)))
    }

    fn concern_module_name(concern_id: &str) -> String {
        match concern_id {
            "tests-present" => "black_box_tests".to_string(),
            other => other.replace('-', "_"),
        }
    }

    fn adjacent_fixture_dir(concern_id: &str) -> PathBuf {
        crate::workspace_root()
            .join("crates/standards/src/concerns")
            .join(concern_id.replace('-', "_"))
            .join("fixtures")
    }

    fn has_checker(module_content: &str) -> bool {
        module_content.contains("#[cfg(test)]") && module_content.contains("#[test]")
    }

    fn has_fail_fixture(fixture_dir: &Path) -> bool {
        if !fixture_dir.is_dir() {
            return false;
        }

        std::fs::read_dir(fixture_dir)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture_dir.display()))
            .filter_map(|entry| entry.ok())
            .any(|entry| {
                entry.file_type().is_ok_and(|file_type| file_type.is_dir())
                    && entry.file_name().to_string_lossy().starts_with("fail")
            })
    }

    fn fixtures_impractical_reason(concern_id: &str) -> Option<&'static str> {
        FIXTURES_IMPRACTICAL
            .iter()
            .find_map(|(id, reason)| (*id == concern_id).then_some(*reason))
    }

    #[test]
    fn fixture_impractical_reasons_are_current_and_specific() {
        let known_ids: BTreeSet<_> = ALL_CONCERN_SPECS.iter().map(|spec| spec.id).collect();

        for (id, reason) in FIXTURES_IMPRACTICAL {
            assert!(known_ids.contains(id), "{id}: unknown concern id");
            assert!(!reason.trim().is_empty(), "{id}: reason must not be empty");
            assert!(
                reason.len() >= 40,
                "{id}: reason should explain why fixtures are impractical"
            );

            let fixture_dir = adjacent_fixture_dir(id);
            assert!(
                !(fixture_dir.join("pass").is_dir() && has_fail_fixture(&fixture_dir)),
                "{id}: remove fixtures-impractical reason now that adjacent pass/fail fixtures exist"
            );
        }
    }
}
