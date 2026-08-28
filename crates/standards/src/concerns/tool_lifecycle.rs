//! # Tool Lifecycle
//!
//! Archived tools are historical products, not deleted or silently abandoned
//! active products. Their source, final release, and explanatory site remain
//! public, while maintained-tool checks stop requiring new compliance work.

pub const NOT_APPLICABLE: &[&str] = &[];

pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "tool-lifecycle",
    definition_summary:
        "Archived tools must remain publicly identified as archived with preserved source and final-release links, and the umbrella site must list them separately from maintained tools.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note:
        "Applies to workspace lifecycle inventory and every tool listed in ARCHIVED_TOOLS.",
};

#[cfg(test)]
mod tests {
    use crate::{workspace_root, ARCHIVED_TOOLS};
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn tool_lifecycle() {
        let umbrella = read(workspace_root().join("docs/index.html"));
        let mut failures = Vec::new();

        for tool in ARCHIVED_TOOLS {
            let repo = repository(tool);
            let site_url = site_url(tool);
            let site = fetch(&site_url).unwrap_or_else(|error| {
                failures.push(format!("{tool}: historical site unavailable: {error}"));
                String::new()
            });
            let archived = repository_is_archived(&repo).unwrap_or_else(|error| {
                failures.push(format!("{tool}: cannot read repository lifecycle: {error}"));
                false
            });

            failures.extend(presentation_failures(
                tool, &repo, &site_url, &umbrella, &site, archived,
            ));
        }

        if !failures.is_empty() {
            panic!("tool-lifecycle non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn presentation_failures(
        tool: &str,
        repo: &str,
        site_url: &str,
        umbrella: &str,
        site: &str,
        archived: bool,
    ) -> Vec<String> {
        let mut failures = Vec::new();
        let umbrella_lower = umbrella.to_lowercase();
        let site_lower = site.to_lowercase();

        if !umbrella.contains("id=\"old-tools\"") {
            failures.push("workspace: umbrella site missing old-tools section".to_string());
        }
        if !umbrella.contains(repo) || !umbrella.contains(site_url) {
            failures.push(format!(
                "{tool}: umbrella old-tools entry must preserve source and site links"
            ));
        }
        if !umbrella_lower.contains("archived") {
            failures.push("workspace: old-tools presentation must explain archival".to_string());
        }
        if !site_lower.contains("archived") {
            failures.push(format!(
                "{tool}: historical site does not say it is archived"
            ));
        }
        if !site.contains(repo) || !site.contains("/releases/tag/") {
            failures.push(format!(
                "{tool}: historical site must preserve source and final-release links"
            ));
        }
        if site.contains("<h2>Install</h2>") {
            failures.push(format!(
                "{tool}: historical site still presents an active installation section"
            ));
        }
        if !archived {
            failures.push(format!("{tool}: GitHub repository is not archived"));
        }

        failures
    }

    fn repository(tool: &str) -> String {
        format!("https://github.com/maxeonyx/{tool}")
    }

    fn site_url(tool: &str) -> String {
        format!("https://{tool}.maxeonyx.com")
    }

    fn repository_is_archived(repo: &str) -> Result<bool, String> {
        let name = repo
            .strip_prefix("https://github.com/")
            .ok_or_else(|| format!("invalid repository URL {repo}"))?;
        let output = Command::new("gh")
            .args([
                "repo",
                "view",
                name,
                "--json",
                "isArchived",
                "--jq",
                ".isArchived",
            ])
            .output()
            .map_err(|error| format!("failed to run gh: {error}"))?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim() == "true")
    }

    fn fetch(url: &str) -> Result<String, String> {
        let output = Command::new("curl")
            .args(["-fsSL", url])
            .output()
            .map_err(|error| format!("failed to run curl: {error}"))?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn read(path: PathBuf) -> String {
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
    }

    fn fixture(name: &str, file: &str) -> String {
        read(
            workspace_root()
                .join("crates/standards/src/concerns/tool_lifecycle/fixtures")
                .join(name)
                .join(file),
        )
    }

    #[test]
    fn compliant_archive_fixture_passes() {
        let failures = presentation_failures(
            "oc",
            "https://github.com/maxeonyx/oc",
            "https://oc.maxeonyx.com",
            &fixture("pass", "umbrella.html"),
            &fixture("pass", "site.html"),
            fixture("pass", "repo.txt").trim() == "archived",
        );

        assert!(failures.is_empty(), "{failures:?}");
    }

    #[test]
    fn active_archive_fixture_is_rejected() {
        let failures = presentation_failures(
            "oc",
            "https://github.com/maxeonyx/oc",
            "https://oc.maxeonyx.com",
            &fixture("fail-active", "umbrella.html"),
            &fixture("fail-active", "site.html"),
            false,
        );

        assert!(failures
            .iter()
            .any(|failure| failure.contains("not archived")));
        assert!(failures.iter().any(|failure| failure.contains("old-tools")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("installation")));
    }
}
