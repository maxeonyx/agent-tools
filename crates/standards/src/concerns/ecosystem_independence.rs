//! # Ecosystem Independence
//!
//! The umbrella coordinates repositories; it must not be a hidden build or
//! development dependency of them. Every maintained child must work from an
//! ordinary standalone clone. Shared code belongs in its own leaf repository,
//! while review attestation state belongs only to the umbrella control plane.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "ecosystem-independence",
    definition_summary: "Maintained child repositories must build, test, and document development without depending on the umbrella checkout.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Children must be standalone; the workspace is checked because it exclusively owns centralized review state.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{checked_tools, tools_dir, workspace_root};
    use std::path::Path;

    #[test]
    fn maintained_children_are_standalone() {
        let mut failures = Vec::new();

        let state_path = workspace_root().join("state.json");
        if !state_path.is_file() {
            failures.push("workspace: centralized state.json is missing".to_string());
        }

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(check_repo(tool, &tools_dir().join(tool)));
        }

        if !failures.is_empty() {
            panic!(
                "ecosystem-independence non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn check_repo(repo: &str, repo_dir: &Path) -> Vec<String> {
        let mut failures = Vec::new();
        let manifest = read(repo, &repo_dir.join("Cargo.toml"), &mut failures);
        let agents = read(repo, &repo_dir.join("AGENTS.md"), &mut failures);
        let workflow = read(
            repo,
            &repo_dir.join(".github/workflows/ci.yml"),
            &mut failures,
        );

        if manifest
            .lines()
            .any(|line| line.contains("path = \"../") || line.contains("path=\"../"))
        {
            failures.push(format!(
                "{repo}: Cargo.toml has a dependency outside its checkout"
            ));
        }

        for forbidden in [
            "github.com/maxeonyx/agent-tools ",
            "github.com/maxeonyx/agent-tools.git",
            "agent-tools-deps",
            "../../crates",
        ] {
            if workflow.contains(forbidden) {
                failures.push(format!(
                    "{repo}: ci.yml reconstructs umbrella context via {forbidden}"
                ));
            }
        }

        if !agents.to_ascii_lowercase().contains("standalone clone") {
            failures.push(format!(
                "{repo}: AGENTS.md must say development works from a standalone clone"
            ));
        }
        for forbidden in [
            "not from this repo",
            "not from this repository",
            "../../AGENTS.md",
            "/home/maxeonyx/agent-tools",
        ] {
            if agents.contains(forbidden) {
                failures.push(format!(
                    "{repo}: AGENTS.md requires parent context via {forbidden}"
                ));
            }
        }

        if repo_dir.join("state.json").exists() {
            failures.push(format!(
                "{repo}: review state belongs in umbrella state.json, not the child repo"
            ));
        }

        failures
    }

    fn read(repo: &str, path: &Path, failures: &mut Vec<String>) -> String {
        match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) => {
                failures.push(format!("{repo}: cannot read {}: {error}", path.display()));
                String::new()
            }
        }
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        workspace_root()
            .join("crates/standards/src/concerns/ecosystem_independence/fixtures")
            .join(name)
    }

    #[test]
    fn standalone_fixture_passes() {
        assert!(check_repo("fixture", &fixture("pass")).is_empty());
    }

    #[test]
    fn parent_coupled_fixture_fails() {
        let failures = check_repo("fixture", &fixture("fail-parent-coupled"));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("outside its checkout")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("umbrella context")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("parent context")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("review state")));
    }
}
