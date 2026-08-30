//! # Integration Policy
//!
//! One-run integration relies on repository settings as well as workflow
//! source: auto-merge must be available and `main` must require a current
//! successful Ready check, including for administrators.

pub const NOT_APPLICABLE: &[&str] = &[];

pub const REVIEW_INSTRUCTIONS: &str = "";

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
    use crate::evidence::{self, EvidenceKey};
    use crate::workspace_root;
    use serde_json::Value;

    const REPOSITORIES: &[&str] = &[
        "help-test",
        "trunc",
        "tmux-bridge",
        "dotsync",
        "tdd-ratchet-rs",
        "agent-harness",
    ];

    #[test]
    fn integration_policy() {
        let mut failures = Vec::new();

        for repo in REPOSITORIES {
            let settings_endpoint = format!("repos/maxeonyx/{repo}");
            let settings = github_json(repo, "repository-settings", &settings_endpoint);
            match settings {
                Ok(content) => failures.extend(settings_failures(repo, &content)),
                Err(error) => failures.push(error),
            }

            let protection_endpoint = format!("repos/maxeonyx/{repo}/branches/main/protection");
            let protection = github_json(repo, "main-protection", &protection_endpoint);
            match protection {
                Ok(content) => failures.extend(protection_failures(repo, &content)),
                Err(error) => failures.push(error),
            }
        }

        if !failures.is_empty() {
            panic!(
                "integration-policy non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn github_json(repo: &str, kind: &str, endpoint: &str) -> Result<String, String> {
        let output = evidence::context().command(
            EvidenceKey::new(kind, endpoint).repo(repo),
            "gh",
            &["api", endpoint],
            &workspace_root(),
        );
        if output.status_success {
            Ok(output.stdout)
        } else {
            Err(format!(
                "{repo}: cannot read {kind}: {}",
                output.stderr.trim()
            ))
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
}
