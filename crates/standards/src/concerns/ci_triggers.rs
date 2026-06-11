//! # Well-Tuned CI Triggers
//!
//! Tool repositories should not run source checks, release builds, or binary
//! publishing for every repository change. Source CI is expensive and
//! release-producing, so it should trigger only for source/release inputs.
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
        "Tool CI must use path filters so source/release CI, Pages CI, and internal docs changes trigger only the appropriate workflow.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to standalone tool repos because they own release-producing CI and Pages workflows.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use std::path::Path;

    #[test]
    fn ci_triggers_are_well_tuned() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            check_tool(tool, &tool_dir, &mut failures);
        }

        if !failures.is_empty() {
            panic!("ci-triggers non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn check_tool(tool: &str, tool_dir: &Path, failures: &mut Vec<String>) {
        let workflow_dir = tool_dir.join(".github/workflows");
        let ci_path = workflow_dir.join("ci.yml");
        let pages_path = workflow_dir.join("pages.yml");

        let ci = read_workflow(tool, &ci_path, failures);
        let pages = read_workflow(tool, &pages_path, failures);
        let (Some(ci), Some(pages)) = (ci, pages) else {
            return;
        };

        for required in [
            "paths:",
            ".github/workflows/ci.yml",
            "Cargo.toml",
            "Cargo.lock",
            "src/**",
            "tests/**",
            "docs/version.json",
        ] {
            if !ci.contains(required) {
                failures.push(format!(
                    "{tool}: ci.yml missing source path filter {required}"
                ));
            }
        }

        if ci.contains("docs/**")
            || ci.contains("docs/process/**")
            || ci.contains("docs/source-notes/**")
        {
            failures.push(format!(
                "{tool}: ci.yml must not broadly trigger source/release CI for internal docs"
            ));
        }

        if !ci.contains("gh release") {
            failures.push(format!(
                "{tool}: ci.yml no longer appears release-producing"
            ));
        }

        for required in [
            "paths:",
            ".github/workflows/pages.yml",
            "docs/index.html",
            "docs/CNAME",
        ] {
            if !pages.contains(required) {
                failures.push(format!(
                    "{tool}: pages.yml missing website path filter {required}"
                ));
            }
        }

        if pages.contains("docs/**")
            || pages.contains("docs/process/**")
            || pages.contains("docs/source-notes/**")
        {
            failures.push(format!(
                "{tool}: pages.yml must not broadly trigger for internal docs"
            ));
        }

        if !pages.contains("deploy-pages") {
            failures.push(format!("{tool}: pages.yml missing Pages deployment"));
        }

        if !pages.contains("gh release download") {
            failures.push(format!(
                "{tool}: pages.yml should preserve release downloads when deploying website-only changes"
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
