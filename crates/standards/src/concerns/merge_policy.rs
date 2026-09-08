//! # Merge Policy
//!
//! Active ecosystem repositories preserve branch history by accepting merge
//! commits and rejecting GitHub's squash and rebase merge modes.

pub const NOT_APPLICABLE: &[&str] = &["oc", "wmux"];

pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the live GitHub merge settings for the target repository. This is a
manual attestation because GitHub does not expose all required settings to the
least-privilege Actions token used for untrusted pull-request tests.

Required review method:
1. Resolve the repository from the target's `origin` remote. For the `workspace`
   target, review both `maxeonyx/agent-tools` and
   `maxeonyx/agent-tools-workspace`.
2. For each repository, run `gh api repos/OWNER/REPO` with reviewer credentials
   that can observe its merge settings.
3. Verify `archived` is false, `allow_merge_commit` is true,
   `allow_squash_merge` is false, and `allow_rebase_merge` is false.
4. Treat missing or inaccessible fields as a finding; do not infer compliance.
5. Report the repositories and fields inspected. Record the attestation only
   when every repository in the target is clean.
"#;

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
    use super::{NOT_APPLICABLE, REVIEW_INSTRUCTIONS};
    use crate::{checked_libraries, concerns, libraries_dir, workspace_root};
    use serde_json::Value;

    #[test]
    fn merge_policy_attestations() {
        assert!(
            !REVIEW_INSTRUCTIONS.trim().is_empty(),
            "merge-policy must remain an explicit manual concern"
        );

        let mut failures = concerns::review_attestation_failures("merge-policy", NOT_APPLICABLE);

        for library in checked_libraries() {
            if let Some(failure) = concerns::review_attestation_failure_for_repo(
                library,
                &libraries_dir().join(library),
                "merge-policy",
            ) {
                failures.push(failure);
            }
        }

        if let Some(failure) = concerns::review_attestation_failure_for_repo(
            "workspace",
            &workspace_root(),
            "merge-policy",
        ) {
            failures.push(failure);
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
