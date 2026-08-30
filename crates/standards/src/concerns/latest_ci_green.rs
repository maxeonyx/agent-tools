//! # Latest CI Green
//!
//! The workspace should only point at tool commits that have a successful
//! `main` CI run.
//!
//! Release and website checks validate public artifacts. They do not prove the
//! current recorded tool head passed its own repository integration. Compliance
//! here is remote: the exact merge commit must carry the successful
//! `integrated-ci` commit status written after the one-run pipeline publishes.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "latest-ci-green",
    definition_summary:
        "Pinned tool commits must have a successful integrated-ci status on the exact recorded merge commit.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to subrepo commits pinned by the workspace, not to the workspace repo itself.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
    use crate::{checked_tools, tools_dir, workspace_root};
    use serde_json::Value;

    #[test]
    fn latest_ci_green() {
        let mut failures = Vec::new();

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            let repo_url = evidence::package_field(&cargo_toml, "repository")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing repository URL"));
            let repo = repo_url
                .strip_prefix("https://github.com/")
                .unwrap_or_else(|| panic!("{tool}: repository is not a GitHub URL"));
            let head = evidence::tool_commit(&tool_dir)
                .unwrap_or_else(|error| panic!("{tool}: failed to read HEAD: {error}"));

            let endpoint = format!("repos/{repo}/commits/{head}/status");
            let statuses = command_stdout(
                EvidenceKey::new("github-integrated-ci-status", format!("{repo}:{head}"))
                    .repo(repo)
                    .tool(tool)
                    .commit(&head),
                "gh",
                &["api", &endpoint],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read commit statuses: {error}"));

            let parsed: Value = serde_json::from_str(&statuses)
                .unwrap_or_else(|error| panic!("{tool}: invalid commit status JSON: {error}"));
            let status = parsed
                .get("statuses")
                .and_then(Value::as_array)
                .and_then(|statuses| {
                    statuses.iter().find(|status| {
                        status.get("context").and_then(Value::as_str) == Some("integrated-ci")
                    })
                });
            let Some(status) = status else {
                failures.push(format!(
                    "{tool}: no integrated-ci status found for pinned merge commit {head}"
                ));
                continue;
            };

            let state = status
                .get("state")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let url = status
                .get("target_url")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if state != "success" {
                failures.push(format!(
                    "{tool}: integrated-ci status for {head} is state={state} ({url})"
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "latest-ci-green non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn command_stdout(key: EvidenceKey, command: &str, args: &[&str]) -> Result<String, String> {
        let output = evidence::context().command(key, command, args, &workspace_root());

        if !output.status_success {
            return Err(output.stderr.trim().to_string());
        }

        Ok(output.stdout.trim().to_string())
    }
}
