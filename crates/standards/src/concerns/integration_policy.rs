//! # Integration Policy
//!
//! One-run integration relies on repository settings as well as workflow
//! source: auto-merge must be available and `main` must require a current
//! successful Ready check, including for administrators.

pub const NOT_APPLICABLE: &[&str] = &[];

pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the live GitHub integration settings for the target repository. This is
a manual attestation because branch protection and environment policy are not
visible to the least-privilege Actions token used for untrusted pull-request
tests.

Required review method:
1. Resolve the GitHub repository from the target's `origin` remote.
2. Run `gh api repos/OWNER/REPO` and verify `allow_auto_merge` is true.
3. Run `gh api repos/OWNER/REPO/branches/main/protection` and verify required
   status checks are strict, include `Ready`, and `enforce_admins.enabled` is
   true.
4. For maintained tools (all targets except `help-test`), run
   `gh api repos/OWNER/REPO/environments/github-pages/deployment-branch-policies`
   and verify a branch policy named `*` permits validated integration branches.
5. Treat missing or inaccessible fields as a finding; do not infer compliance.
6. Report the endpoints and fields inspected. Record the attestation only when
   every required setting is clean.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "integration-policy",
    definition_summary:
        "Integrated repositories must enable auto-merge and protect main with a strict required Ready check enforced for administrators.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to help-test and the five maintained tools that use the serialized one-run integration workflow.",
};

#[cfg(test)]
mod tests {
    use super::REVIEW_INSTRUCTIONS;
    use crate::{checked_libraries, concerns, libraries_dir, workspace_root};
    use serde_json::Value;

    #[test]
    fn integration_policy_attestations() {
        assert!(
            !REVIEW_INSTRUCTIONS.trim().is_empty(),
            "integration-policy must remain an explicit manual concern"
        );

        let mut failures = concerns::review_attestation_failures("integration-policy", &[]);

        for library in checked_libraries() {
            if let Some(failure) = concerns::review_attestation_failure_for_repo(
                library,
                &libraries_dir().join(library),
                "integration-policy",
            ) {
                failures.push(failure);
            }
        }

        if !failures.is_empty() {
            panic!(
                "integration-policy non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn settings_failures(repo: &str, content: &str) -> Vec<String> {
        let value: Value = match serde_json::from_str(content) {
            Ok(value) => value,
            Err(error) => return vec![format!("{repo}: invalid settings JSON: {error}")],
        };
        if value.get("allow_auto_merge").and_then(Value::as_bool) == Some(true) {
            Vec::new()
        } else {
            vec![format!("{repo}: auto-merge is disabled")]
        }
    }

    fn protection_failures(repo: &str, content: &str) -> Vec<String> {
        let value: Value = match serde_json::from_str(content) {
            Ok(value) => value,
            Err(error) => return vec![format!("{repo}: invalid protection JSON: {error}")],
        };
        let mut failures = Vec::new();
        let checks = value.get("required_status_checks");
        if checks
            .and_then(|checks| checks.get("strict"))
            .and_then(Value::as_bool)
            != Some(true)
        {
            failures.push(format!(
                "{repo}: main does not require an up-to-date branch"
            ));
        }
        let has_ready = checks
            .and_then(|checks| checks.get("contexts"))
            .and_then(Value::as_array)
            .is_some_and(|contexts| {
                contexts
                    .iter()
                    .any(|context| context.as_str() == Some("Ready"))
            });
        if !has_ready {
            failures.push(format!("{repo}: main does not require the Ready check"));
        }
        if value
            .get("enforce_admins")
            .and_then(|admins| admins.get("enabled"))
            .and_then(Value::as_bool)
            != Some(true)
        {
            failures.push(format!(
                "{repo}: main protection is not enforced for admins"
            ));
        }
        failures
    }

    fn pages_policy_failures(repo: &str, content: &str) -> Vec<String> {
        let value: Value = match serde_json::from_str(content) {
            Ok(value) => value,
            Err(error) => return vec![format!("{repo}: invalid Pages policy JSON: {error}")],
        };
        let accepts_integration_branches = value
            .get("branch_policies")
            .and_then(Value::as_array)
            .is_some_and(|policies| {
                policies.iter().any(|policy| {
                    policy.get("type").and_then(Value::as_str) == Some("branch")
                        && policy.get("name").and_then(Value::as_str) == Some("*")
                })
            });
        if accepts_integration_branches {
            Vec::new()
        } else {
            vec![format!(
                "{repo}: Pages environment does not accept validated integration branches"
            )]
        }
    }

    fn fixture(name: &str, file: &str) -> String {
        let path = workspace_root()
            .join("crates/standards/src/concerns/integration_policy/fixtures")
            .join(name)
            .join(file);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
    }

    #[test]
    fn protected_integration_fixture_passes() {
        assert!(settings_failures("fixture", &fixture("pass", "settings.json")).is_empty());
        assert!(protection_failures("fixture", &fixture("pass", "protection.json")).is_empty());
        assert!(
            pages_policy_failures("fixture", &fixture("pass", "pages-policies.json")).is_empty()
        );
    }

    #[test]
    fn unprotected_integration_fixture_fails() {
        assert!(
            !settings_failures("fixture", &fixture("fail-unprotected", "settings.json")).is_empty()
        );
        let failures =
            protection_failures("fixture", &fixture("fail-unprotected", "protection.json"));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("up-to-date")));
        assert!(failures.iter().any(|failure| failure.contains("Ready")));
        assert!(failures.iter().any(|failure| failure.contains("admins")));
    }

    #[test]
    fn main_only_pages_policy_is_rejected() {
        let failures = pages_policy_failures(
            "fixture",
            &fixture("fail-unprotected", "pages-policies.json"),
        );

        assert!(failures
            .iter()
            .any(|failure| failure.contains("integration branches")));
    }
}
