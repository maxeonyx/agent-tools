//! # Merge Policy
//!
//! Active ecosystem repositories preserve branch history by accepting merge
//! commits and rejecting GitHub's squash and rebase merge modes.

pub const NOT_APPLICABLE: &[&str] = &["oc", "wmux"];

pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "merge-policy",
    definition_summary:
        "Active ecosystem repositories must allow merge commits and disable squash and rebase merges.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note:
        "Applies to the active umbrella, workspace, library, and maintained tool repositories; archived or not-yet-integrated repositories are excluded.",
};

#[cfg(test)]
mod tests {
    use crate::evidence::{self, EvidenceKey};
    use crate::workspace_root;
    use serde_json::Value;

    const ACTIVE_REPOSITORIES: &[&str] = &[
        "agent-tools",
        "agent-tools-workspace",
        "help-test",
        "trunc",
        "tmux-bridge",
        "dotsync",
        "tdd-ratchet-rs",
        "agent-harness",
    ];

    #[test]
    fn merge_policy() {
        let mut failures = Vec::new();

        for repo in ACTIVE_REPOSITORIES {
            let endpoint = format!("repos/maxeonyx/{repo}");
            let output = evidence::context().command(
                EvidenceKey::new("github-repository-settings", &endpoint).repo(*repo),
                "gh",
                &["api", &endpoint],
                &workspace_root(),
            );
            if !output.status_success {
                failures.push(format!(
                    "{repo}: cannot read GitHub repository settings: {}",
                    output.stderr.trim()
                ));
                continue;
            }
            failures.extend(settings_failures(repo, &output.stdout));
        }

        if !failures.is_empty() {
            panic!("merge-policy non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn settings_failures(repo: &str, content: &str) -> Vec<String> {
        let value: Value = match serde_json::from_str(content) {
            Ok(value) => value,
            Err(error) => {
                return vec![format!("{repo}: invalid repository settings JSON: {error}")]
            }
        };
        let mut failures = Vec::new();

        if value.get("archived").and_then(Value::as_bool) != Some(false) {
            failures.push(format!("{repo}: repository is not active"));
        }
        if value.get("allow_merge_commit").and_then(Value::as_bool) != Some(true) {
            failures.push(format!("{repo}: merge commits are disabled"));
        }
        if value.get("allow_squash_merge").and_then(Value::as_bool) != Some(false) {
            failures.push(format!("{repo}: squash merges are enabled"));
        }
        if value.get("allow_rebase_merge").and_then(Value::as_bool) != Some(false) {
            failures.push(format!("{repo}: rebase merges are enabled"));
        }

        failures
    }

    fn fixture(name: &str) -> String {
        let path = workspace_root()
            .join("crates/standards/src/concerns/merge_policy/fixtures")
            .join(name)
            .join("settings.json");
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
    }

    #[test]
    fn merge_only_fixture_passes() {
        assert!(settings_failures("fixture", &fixture("pass")).is_empty());
    }

    #[test]
    fn history_rewriting_fixture_fails() {
        let failures = settings_failures("fixture", &fixture("fail-history-rewriting"));

        assert!(failures.iter().any(|failure| failure.contains("squash")));
        assert!(failures.iter().any(|failure| failure.contains("rebase")));
    }
}
