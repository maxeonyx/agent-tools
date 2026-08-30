//! # Well-Tuned CI Triggers
//!
//! Source CI is expensive and release-producing. It runs once, by explicit
//! dispatch against a pull request that has already passed the repository's
//! offline checks, and serializes integration across the repository.
//!
//! Website updates are separate: if a repo uses `docs/` as its Pages site, a
//! docs website change should run a Pages deployment, not the source release
//! pipeline. Internal process/design docs under `docs/process/` and
//! `docs/source-notes/` are not website content and should not trigger either
//! workflow by default.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "ci-triggers",
    definition_summary:
        "Release CI must be explicitly dispatched, serialized through one Ready-to-merge run, and reproducible through repository-local offline checks.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to standalone maintained tools and libraries because they own release-producing CI; tool website-only deployments remain separate.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{checked_libraries, checked_tools, libraries_dir, tools_dir};
    use std::path::Path;

    #[test]
    fn ci_triggers_are_well_tuned() {
        let mut failures = Vec::new();

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            check_repository(tool, &tool_dir, true, &mut failures);
        }
        for library in checked_libraries() {
            let library_dir = libraries_dir().join(library);
            check_repository(library, &library_dir, false, &mut failures);
        }

        if !failures.is_empty() {
            panic!("ci-triggers non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn check_repository(repo: &str, repo_dir: &Path, has_pages: bool, failures: &mut Vec<String>) {
        let workflow_dir = repo_dir.join(".github/workflows");
        let ci_path = workflow_dir.join("ci.yml");
        let pages_path = workflow_dir.join("pages.yml");

        let Some(ci) = read_workflow(repo, &ci_path, failures) else {
            return;
        };

        for required in [
            "workflow_dispatch:",
            "pr_number:",
            "group: integration",
            "cancel-in-progress: false",
            "name: Ready",
            "actionlint",
            "gh pr merge",
            "--merge",
            "--auto",
            "integrated-ci",
            "cargo build --release",
            "gh release",
        ] {
            if !ci.contains(required) {
                failures.push(format!(
                    "{repo}: ci.yml missing integration evidence {required}"
                ));
            }
        }

        if ci.contains("push:") || ci.contains("pull_request:") || ci.contains("merge_group:") {
            failures.push(format!(
                "{repo}: ci.yml must run only by explicit workflow dispatch"
            ));
        }

        let devenv = std::fs::read_to_string(repo_dir.join("devenv.nix")).unwrap_or_default();
        if !devenv.contains("pkgs.actionlint") || !devenv.contains("actionlint") {
            failures.push(format!(
                "{repo}: devenv test must install and run actionlint offline"
            ));
        }

        if !has_pages {
            return;
        }
        let Some(pages) = read_workflow(repo, &pages_path, failures) else {
            return;
        };
        for required in [
            "paths:",
            ".github/workflows/pages.yml",
            "docs/index.html",
            "docs/CNAME",
        ] {
            if !pages.contains(required) {
                failures.push(format!(
                    "{repo}: pages.yml missing website path filter {required}"
                ));
            }
        }

        if pages.contains("docs/**")
            || pages.contains("docs/process/**")
            || pages.contains("docs/source-notes/**")
        {
            failures.push(format!(
                "{repo}: pages.yml must not broadly trigger for internal docs"
            ));
        }

        if !pages.contains("deploy-pages") {
            failures.push(format!("{repo}: pages.yml missing Pages deployment"));
        }

        if !pages.contains("gh release download") {
            failures.push(format!(
                "{repo}: pages.yml should preserve release downloads when deploying website-only changes"
            ));
        }
    }

    fn read_workflow(tool: &str, path: &Path, failures: &mut Vec<String>) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(error) => {
                failures.push(format!("{tool}: missing {}: {error}", path.display()));
                None
            }
        }
    }
}
