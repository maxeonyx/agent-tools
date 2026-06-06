//! Cross-cutting concerns for the agent-tools workspace.
//!
//! Each submodule defines one concern: what it is, why it matters,
//! review instructions (if agentic), and a mechanical test.

pub mod auto_update;
pub mod auto_update_integration;
pub mod black_box_tests;
pub mod code_review;
pub mod code_standards;
pub mod concern_module_coverage;
pub mod devenv_check;
pub mod error_messages;
pub mod fast_slow_checks;
pub mod help_text;
pub mod injectable_io;
pub mod landing_page;
pub mod latest_ci_green;
pub mod opencode_skill;
pub mod pinned_main_parity;
pub mod release_freshness;
pub mod release_pipeline;
pub mod standalone_publishability;
pub mod tdd_ratchet;
pub mod version_artifacts;
pub mod vision_and_process;
pub mod website_install_links;
pub mod workspace_routing;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConcernSpec {
    pub id: &'static str,
    pub definition_summary: &'static str,
    pub review_instructions: &'static str,
    pub review_file_name: Option<&'static str>,
    pub applies_to_workspace: bool,
    pub applicability_note: &'static str,
}

pub const ALL_CONCERN_SPECS: &[ConcernSpec] = &[
    workspace_routing::SPEC,
    tdd_ratchet::SPEC,
    black_box_tests::SPEC,
    code_standards::SPEC,
    devenv_check::SPEC,
    version_artifacts::SPEC,
    help_text::SPEC,
    error_messages::SPEC,
    landing_page::SPEC,
    release_pipeline::SPEC,
    release_freshness::SPEC,
    latest_ci_green::SPEC,
    pinned_main_parity::SPEC,
    standalone_publishability::SPEC,
    auto_update::SPEC,
    auto_update_integration::SPEC,
    website_install_links::SPEC,
    vision_and_process::SPEC,
    opencode_skill::SPEC,
    fast_slow_checks::SPEC,
    injectable_io::SPEC,
    code_review::SPEC,
    concern_module_coverage::SPEC,
];

/// All known concern IDs.
pub const ALL_CONCERNS: &[&str] = &[
    "workspace-routing",
    "tdd-ratchet",
    "black-box-tests",
    "code-standards",
    "devenv-check",
    "version-artifacts",
    "help-text",
    "error-messages",
    "landing-page",
    "release-pipeline",
    "release-freshness",
    "latest-ci-green",
    "pinned-main-parity",
    "standalone-publishability",
    "auto-update",
    "auto-update-integration",
    "website-install-links",
    "vision-and-process",
    "opencode-skill",
    "fast-slow-checks",
    "injectable-io",
    "code-review",
    "concern-module-coverage",
];

/// Concerns that require agentic review (have non-empty REVIEW_INSTRUCTIONS).
pub const AGENTIC_CONCERNS: &[&str] = &[
    "code-review",
    "error-messages",
    "injectable-io",
    "help-text",
];

pub fn concern_spec(id: &str) -> Option<&'static ConcernSpec> {
    ALL_CONCERN_SPECS.iter().find(|spec| spec.id == id)
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Eq, PartialEq)]
pub struct ReviewAttestation {
    pub reviewed_commit: String,
    pub concern: String,
    pub repo: String,
    pub attested_via: String,
}

#[cfg(test)]
pub(crate) fn review_attestation_failures(
    review_file_name: &str,
    not_applicable: &[&str],
) -> Vec<String> {
    let mut failures = Vec::new();

    for tool in crate::TOOLS
        .iter()
        .filter(|tool| !not_applicable.contains(tool))
    {
        let tool_dir = crate::tools_dir().join(tool);
        if let Some(failure) =
            review_attestation_failure_for_repo(tool, &tool_dir, review_file_name)
        {
            failures.push(failure);
        }
    }

    failures
}

#[cfg(test)]
pub(crate) fn review_attestation_failure_for_repo(
    repo_name: &str,
    repo_dir: &std::path::Path,
    review_file_name: &str,
) -> Option<String> {
    let review_file = repo_dir.join(review_file_name);

    if !review_file.exists() {
        return Some(format!("{repo_name}: {review_file_name} missing"));
    }

    let content = std::fs::read_to_string(&review_file).unwrap_or_default();
    let attestation = match parse_review_attestation(&content) {
        Ok(attestation) => attestation,
        Err(error) => {
            return Some(format!("{repo_name}: {review_file_name} invalid: {error}"));
        }
    };

    let expected_concern = ALL_CONCERN_SPECS
        .iter()
        .find(|spec| spec.review_file_name == Some(review_file_name))
        .map(|spec| spec.id)
        .unwrap_or("");

    if attestation.concern != expected_concern {
        return Some(format!(
            "{repo_name}: {review_file_name} recorded concern {}, expected {expected_concern}",
            attestation.concern
        ));
    }

    if attestation.repo != repo_name {
        return Some(format!(
            "{repo_name}: {review_file_name} recorded repo {}, expected {repo_name}",
            attestation.repo
        ));
    }

    if attestation.attested_via != "review-attest" {
        return Some(format!(
            "{repo_name}: {review_file_name} recorded attested_via {}, expected review-attest",
            attestation.attested_via
        ));
    }

    if attestation.reviewed_commit.trim().is_empty() {
        return Some(format!(
            "{repo_name}: {review_file_name} missing reviewed_commit"
        ));
    };

    let current_commit = latest_substantive_commit(repo_dir).unwrap_or_else(|error| {
        panic!(
            "failed to read current substantive commit for {}: {error}",
            repo_dir.display()
        )
    });

    if attestation.reviewed_commit != current_commit {
        return Some(format!(
            "{repo_name}: {review_file_name} reviewed {}, current substantive commit is {current_commit}",
            attestation.reviewed_commit
        ));
    }

    None
}

pub fn parse_review_attestation(content: &str) -> Result<ReviewAttestation, String> {
    serde_json::from_str(content).map_err(|error| error.to_string())
}

pub fn latest_substantive_commit(repo_dir: &std::path::Path) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args([
            "log",
            "-1",
            "--format=%H",
            "--",
            ".",
            ":(exclude)docs/reviews",
        ])
        .current_dir(repo_dir)
        .output()
        .map_err(|error| error.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.trim().to_string());
    }

    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if commit.is_empty() {
        return Err("git log returned no substantive commit".to_string());
    }

    Ok(commit)
}

#[cfg(test)]
mod tests {
    use super::{
        concern_spec, latest_substantive_commit, parse_review_attestation,
        review_attestation_failure_for_repo, AGENTIC_CONCERNS, ALL_CONCERNS, ALL_CONCERN_SPECS,
    };
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    fn declared_concerns() -> Vec<(&'static str, &'static str)> {
        ALL_CONCERN_SPECS
            .iter()
            .map(|spec| (spec.id, spec.review_instructions))
            .collect()
    }

    #[test]
    fn all_concerns_list_matches_declared_modules() {
        let actual: Vec<_> = declared_concerns().into_iter().map(|(id, _)| id).collect();
        assert_eq!(actual, ALL_CONCERNS, "all concern IDs must stay in sync");
    }

    #[test]
    fn agentic_concerns_are_exactly_modules_with_review_instructions() {
        let actual: BTreeSet<_> = declared_concerns()
            .into_iter()
            .filter_map(|(id, review_instructions)| {
                (!review_instructions.trim().is_empty()).then_some(id)
            })
            .collect();
        let expected: BTreeSet<_> = AGENTIC_CONCERNS.iter().copied().collect();

        assert_eq!(
            actual, expected,
            "agentic concerns must stay in sync with review instructions"
        );
    }

    #[test]
    fn agentic_concerns_have_review_files_and_non_agentic_concerns_do_not() {
        for spec in ALL_CONCERN_SPECS {
            if AGENTIC_CONCERNS.contains(&spec.id) {
                assert!(
                    spec.review_file_name.is_some(),
                    "{} must declare review_file_name",
                    spec.id
                );
            } else {
                assert!(
                    spec.review_file_name.is_none(),
                    "{} should not declare review_file_name",
                    spec.id
                );
            }
        }
    }

    #[test]
    fn concern_spec_lookup_finds_known_concern() {
        let spec = concern_spec("code-review").expect("code-review spec should exist");
        assert_eq!(
            spec.review_file_name,
            Some("docs/reviews/code-quality.json")
        );
    }

    #[test]
    fn parse_review_attestation_extracts_fields() {
        let attestation = parse_review_attestation(
            r#"{"reviewed_commit":"abc123","concern":"code-review","repo":"workspace","attested_via":"review-attest"}"#,
        )
        .unwrap();
        assert_eq!(attestation.reviewed_commit, "abc123");
        assert_eq!(attestation.concern, "code-review");
    }

    #[test]
    fn latest_substantive_commit_ignores_review_only_followup_commits() {
        let repo = test_repo_dir("latest_substantive_commit");
        init_git_repo(&repo);

        fs::write(repo.join("src.txt"), "v1\n").unwrap();
        git(&repo, &["add", "src.txt"]);
        git(&repo, &["commit", "-m", "substantive"]);
        let substantive = git_output(&repo, &["rev-parse", "HEAD"]);

        fs::create_dir_all(repo.join("docs/reviews")).unwrap();
        fs::write(
            repo.join("docs/reviews/code-quality.json"),
            format!("{{\"reviewed_commit\":\"{}\"}}\n", substantive),
        )
        .unwrap();
        git(&repo, &["add", "docs/reviews/code-quality.json"]);
        git(&repo, &["commit", "-m", "review only"]);

        let latest = latest_substantive_commit(&repo).unwrap();
        assert_eq!(latest, substantive);
    }

    #[test]
    fn review_attestation_failure_rejects_legacy_schema() {
        let repo = test_repo_dir("legacy_attestation_schema");
        init_git_repo(&repo);

        fs::write(repo.join("src.txt"), "v1\n").unwrap();
        git(&repo, &["add", "src.txt"]);
        git(&repo, &["commit", "-m", "substantive"]);

        fs::create_dir_all(repo.join("docs/reviews")).unwrap();
        fs::write(
            repo.join("docs/reviews/code-quality.json"),
            "{\"reviewed_commit\":\"abc123\"}\n",
        )
        .unwrap();

        let failure = review_attestation_failure_for_repo(
            "workspace",
            &repo,
            "docs/reviews/code-quality.json",
        )
        .unwrap();
        assert!(failure.contains("invalid"));
    }

    fn test_repo_dir(name: &str) -> std::path::PathBuf {
        let path = crate::workspace_root()
            .join("target")
            .join("standards-fixtures")
            .join(name);
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn init_git_repo(path: &Path) {
        git(path, &["init", "-b", "main"]);
        git(path, &["config", "user.name", "Fixture User"]);
        git(path, &["config", "user.email", "fixture@example.com"]);
    }

    fn git(path: &Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(path)
            .status()
            .unwrap();
        assert!(
            status.success(),
            "git {:?} failed in {}",
            args,
            path.display()
        );
    }

    fn git_output(path: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed in {}",
            args,
            path.display()
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}
