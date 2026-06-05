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

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn release_freshness() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            let version = package_field(&cargo_toml, "version")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package version"));
            let repository = package_field(&cargo_toml, "repository")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing repository URL"));
            let repo = repository
                .strip_prefix("https://github.com/")
                .unwrap_or_else(|| panic!("{tool}: repository is not a GitHub URL"));
            let tag = format!("v{version}");
            let head = command_stdout(
                "git",
                &["-C", tool_dir.to_str().unwrap(), "rev-parse", "HEAD"],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read HEAD: {error}"));

            let release = command_stdout(
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

    fn package_field<'a>(cargo_toml: &'a str, field: &str) -> Option<&'a str> {
        let prefix = format!("{field} = \"");
        cargo_toml.lines().find_map(|line| {
            line.strip_prefix(&prefix)?
                .split_once('"')
                .map(|(value, _)| value)
        })
    }

    fn command_stdout(command: &str, args: &[&str]) -> Result<String, String> {
        let output = std::process::Command::new(command)
            .args(args)
            .output()
            .map_err(|error| format!("{command} failed to start: {error}"))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
