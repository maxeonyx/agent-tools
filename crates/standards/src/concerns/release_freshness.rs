//! # Release Freshness
//!
//! Every published tool repo should have a GitHub Release for the current
//! `Cargo.toml` version, and the corresponding remote tag should point at the
//! tool commit recorded in this workspace.
//!
//! If `main` moves without a fresh release, install instructions and
//! auto-update checks serve stale binaries. Compliance here is intentionally
//! remote: local tags are not enough, because users install from GitHub
//! Releases and Pages.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "release-freshness",
    definition_summary: "Each tool's latest GitHub release must match the current workspace version and pinned commit.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to tool release state on GitHub, not to the workspace repo itself.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
    use crate::{tools_dir, workspace_root, TOOLS};

    #[test]
    fn release_freshness() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            let version = evidence::package_field(&cargo_toml, "version")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package version"));
            let repository = evidence::package_field(&cargo_toml, "repository")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing repository URL"));
            let repo = repository
                .strip_prefix("https://github.com/")
                .unwrap_or_else(|| panic!("{tool}: repository is not a GitHub URL"));
            let tag = format!("v{version}");
            let head = evidence::tool_commit(&tool_dir)
                .unwrap_or_else(|error| panic!("{tool}: failed to read HEAD: {error}"));

            let release = command_stdout(
                EvidenceKey::new("github-release-tag", format!("{repo}:{tag}"))
                    .repo(repo)
                    .tool(tool)
                    .version(version)
                    .commit(&head),
                "gh",
                &[
                    "release", "view", &tag, "--repo", repo, "--json", "tagName", "-q", ".tagName",
                ],
            );
            if release.is_err() {
                failures.push(format!("{tool}: GitHub Release {tag} missing in {repo}"));
                continue;
            }

            let remote_tag = command_stdout(
                EvidenceKey::new("remote-release-tag", format!("{repo}:{tag}"))
                    .repo(repo)
                    .tool(tool)
                    .version(version)
                    .commit(&head),
                "git",
                &[
                    "ls-remote",
                    "--tags",
                    &format!("https://github.com/{repo}.git"),
                    &format!("refs/tags/{tag}"),
                ],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read remote tag {tag}: {error}"));

            let remote_commit = remote_tag.split_whitespace().next().unwrap_or_default();
            if remote_commit != head {
                failures.push(format!(
                    "{tool}: release tag {tag} points at {remote_commit}, workspace records {head}"
                ));
            }

            let latest = command_stdout(
                EvidenceKey::new("github-latest-release", repo)
                    .repo(repo)
                    .tool(tool)
                    .commit(&head),
                "gh",
                &[
                    "release", "view", "--repo", repo, "--json", "tagName", "-q", ".tagName",
                ],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read latest release: {error}"));
            if latest != tag {
                failures.push(format!(
                    "{tool}: latest GitHub Release is {latest}, current version is {tag}"
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "release-freshness non-compliant:\n  {}",
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
