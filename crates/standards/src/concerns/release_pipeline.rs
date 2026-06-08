//! # Release Pipeline Standards
//!
//! Each tool should have consistent CI: check → build → release → pages.
//!
//! Consistent pipelines mean consistent behavior: debounced runs, predictable
//! release mechanics, and automatic Pages deployment. When you fix a CI issue in
//! one tool, you should be able to apply the same fix across all tools without
//! re-learning each pipeline.
//!
//! Compliance is primarily release evidence: the current GitHub Release must
//! expose the expected Linux binary asset for the tool. Workflow YAML markers
//! remain secondary diagnostics because they explain why artifact evidence may
//! be missing, but they are not the proof by themselves.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "release-pipeline",
    definition_summary:
        "Each tool repo must include the expected release and Pages workflow structure.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to standalone tool repos where release CI runs.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
    use crate::{tools_dir, workspace_root, TOOLS};
    use serde_json::Value;

    #[test]
    fn release_pipeline() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            check_release_asset(tool, &tool_dir, &mut failures);

            let ci_path = tool_dir.join(".github/workflows/ci.yml");
            if !ci_path.exists() {
                failures.push(format!("{tool}: .github/workflows/ci.yml missing"));
                continue;
            }

            let ci_contents = std::fs::read_to_string(&ci_path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", ci_path.display()));

            if !ci_contents.contains("cancel-in-progress: true") {
                failures.push(format!("{tool}: ci.yml missing cancel-in-progress: true"));
            }
            if !ci_contents.contains("gh release") {
                failures.push(format!("{tool}: ci.yml missing gh release"));
            }
            if !ci_contents.contains("deploy-pages") {
                failures.push(format!("{tool}: ci.yml missing deploy-pages"));
            }
        }

        if !failures.is_empty() {
            panic!(
                "release-pipeline non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn check_release_asset(tool: &str, tool_dir: &std::path::Path, failures: &mut Vec<String>) {
        let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
            .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
        let package = evidence::package_field(&cargo_toml, "name")
            .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package name"));
        let version = evidence::package_field(&cargo_toml, "version")
            .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package version"));
        let binary = evidence::binary_name(&cargo_toml).unwrap_or(package);
        let repository = evidence::package_field(&cargo_toml, "repository")
            .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing repository URL"));
        let repo = repository
            .strip_prefix("https://github.com/")
            .unwrap_or_else(|| panic!("{tool}: repository is not a GitHub URL"));
        let tag = format!("v{version}");
        let expected_asset = format!("{binary}-x86_64-linux");
        let commit = evidence::tool_commit(tool_dir).unwrap_or_default();

        let output = evidence::context().command(
            EvidenceKey::new("github-release-assets", format!("{repo}:{tag}"))
                .repo(repo)
                .tool(tool)
                .version(version)
                .commit(commit),
            "gh",
            &["release", "view", &tag, "--repo", repo, "--json", "assets"],
            &workspace_root(),
        );

        if !output.status_success {
            failures.push(format!(
                "{tool}: failed to read GitHub Release {tag} assets: {}",
                output.stderr.trim()
            ));
            return;
        }

        let parsed: Value = match serde_json::from_str(&output.stdout) {
            Ok(parsed) => parsed,
            Err(error) => {
                failures.push(format!("{tool}: gh release asset JSON invalid: {error}"));
                return;
            }
        };
        let assets = parsed
            .get("assets")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let has_expected_asset = assets.iter().any(|asset| {
            asset
                .get("name")
                .and_then(Value::as_str)
                .is_some_and(|name| name == expected_asset)
        });
        if !has_expected_asset {
            failures.push(format!(
                "{tool}: GitHub Release {tag} missing asset {expected_asset}"
            ));
        }
    }
}
