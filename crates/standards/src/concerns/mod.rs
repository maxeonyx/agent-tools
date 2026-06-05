//! Cross-cutting concerns for the agent-tools workspace.
//!
//! Each submodule defines one concern: what it is, why it matters,
//! review instructions (if agentic), and a mechanical test.

pub mod auto_update;
pub mod auto_update_integration;
pub mod black_box_tests;
pub mod code_review;
pub mod code_standards;
pub mod devenv_check;
pub mod error_messages;
pub mod fast_slow_checks;
pub mod help_text;
pub mod injectable_io;
pub mod landing_page;
pub mod opencode_skill;
pub mod release_freshness;
pub mod release_pipeline;
pub mod standalone_publishability;
pub mod tdd_ratchet;
pub mod vision_and_process;
pub mod website_install_links;
pub mod workspace_routing;

/// All known concern IDs.
pub const ALL_CONCERNS: &[&str] = &[
    "workspace-routing",
    "tdd-ratchet",
    "black-box-tests",
    "code-standards",
    "devenv-check",
    "help-text",
    "error-messages",
    "landing-page",
    "release-pipeline",
    "release-freshness",
    "standalone-publishability",
    "auto-update",
    "auto-update-integration",
    "website-install-links",
    "vision-and-process",
    "opencode-skill",
    "fast-slow-checks",
    "injectable-io",
    "code-review",
];

/// Concerns that require agentic review (have non-empty REVIEW_INSTRUCTIONS).
pub const AGENTIC_CONCERNS: &[&str] = &[
    "code-review",
    "error-messages",
    "injectable-io",
    "help-text",
];

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
        let review_file = tool_dir.join(review_file_name);

        if !review_file.exists() {
            failures.push(format!("{tool}: {review_file_name} missing"));
            continue;
        }

        let content = std::fs::read_to_string(&review_file).unwrap_or_default();
        let Some(reviewed_commit) = reviewed_commit(&content) else {
            failures.push(format!(
                "{tool}: {review_file_name} missing reviewed_commit"
            ));
            continue;
        };

        let output = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&tool_dir)
            .output()
            .unwrap_or_else(|error| {
                panic!(
                    "failed to read current commit for {}: {error}",
                    tool_dir.display()
                )
            });

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            failures.push(format!(
                "{tool}: failed to read current commit for attestation check: {}",
                stderr.trim()
            ));
            continue;
        }

        let current_commit = String::from_utf8_lossy(&output.stdout);
        let current_commit = current_commit.trim();
        if reviewed_commit != current_commit {
            failures.push(format!(
                "{tool}: {review_file_name} reviewed {reviewed_commit}, current commit is {current_commit}"
            ));
        }
    }

    failures
}

#[cfg(test)]
fn reviewed_commit(content: &str) -> Option<&str> {
    let key = "\"reviewed_commit\"";
    let after_key = content.split_once(key)?.1;
    let after_colon = after_key.split_once(':')?.1.trim_start();
    let after_quote = after_colon.strip_prefix('"')?;
    let (commit, _) = after_quote.split_once('"')?;
    Some(commit)
}

#[cfg(test)]
mod tests {
    use super::{
        auto_update, auto_update_integration, black_box_tests, code_review, code_standards,
        devenv_check, error_messages, fast_slow_checks, help_text, injectable_io, landing_page,
        opencode_skill, release_freshness, release_pipeline, standalone_publishability,
        tdd_ratchet, vision_and_process, website_install_links, workspace_routing,
        AGENTIC_CONCERNS, ALL_CONCERNS,
    };
    use std::collections::BTreeSet;

    fn declared_concerns() -> [(&'static str, &'static str); 19] {
        [
            ("workspace-routing", workspace_routing::REVIEW_INSTRUCTIONS),
            ("tdd-ratchet", tdd_ratchet::REVIEW_INSTRUCTIONS),
            ("black-box-tests", black_box_tests::REVIEW_INSTRUCTIONS),
            ("code-standards", code_standards::REVIEW_INSTRUCTIONS),
            ("devenv-check", devenv_check::REVIEW_INSTRUCTIONS),
            ("help-text", help_text::REVIEW_INSTRUCTIONS),
            ("error-messages", error_messages::REVIEW_INSTRUCTIONS),
            ("landing-page", landing_page::REVIEW_INSTRUCTIONS),
            ("release-pipeline", release_pipeline::REVIEW_INSTRUCTIONS),
            ("release-freshness", release_freshness::REVIEW_INSTRUCTIONS),
            (
                "standalone-publishability",
                standalone_publishability::REVIEW_INSTRUCTIONS,
            ),
            ("auto-update", auto_update::REVIEW_INSTRUCTIONS),
            (
                "auto-update-integration",
                auto_update_integration::REVIEW_INSTRUCTIONS,
            ),
            (
                "website-install-links",
                website_install_links::REVIEW_INSTRUCTIONS,
            ),
            (
                "vision-and-process",
                vision_and_process::REVIEW_INSTRUCTIONS,
            ),
            ("opencode-skill", opencode_skill::REVIEW_INSTRUCTIONS),
            ("fast-slow-checks", fast_slow_checks::REVIEW_INSTRUCTIONS),
            ("injectable-io", injectable_io::REVIEW_INSTRUCTIONS),
            ("code-review", code_review::REVIEW_INSTRUCTIONS),
        ]
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
}
