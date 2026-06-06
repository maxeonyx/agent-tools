//! # Pinned Main Parity
//!
//! The workspace should not pin submodule commits that are behind a tool
//! repository's remote `main`.
//!
//! Compliance means the commit recorded in this workspace is either exactly the
//! current remote `origin/main` commit or a descendant of it. That allows
//! in-flight local commits while rejecting stale pins.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn pinned_main_parity() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let head = command_stdout(
                "git",
                &["-C", tool_dir.to_str().unwrap(), "rev-parse", "HEAD"],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read HEAD: {error}"));

            let remote_main = command_stdout(
                "git",
                &[
                    "-C",
                    tool_dir.to_str().unwrap(),
                    "ls-remote",
                    "--heads",
                    "origin",
                    "main",
                ],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read origin/main: {error}"));

            let remote_main = remote_main
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_string();
            if remote_main.is_empty() {
                failures.push(format!("{tool}: origin/main not found"));
                continue;
            }

            if remote_main == head {
                continue;
            }

            let fetch_status = std::process::Command::new("git")
                .args([
                    "-C",
                    tool_dir.to_str().unwrap(),
                    "fetch",
                    "--quiet",
                    "origin",
                    "main",
                ])
                .status()
                .unwrap_or_else(|error| panic!("{tool}: failed to fetch origin/main: {error}"));
            if !fetch_status.success() {
                failures.push(format!("{tool}: failed to fetch origin/main"));
                continue;
            }

            let status = std::process::Command::new("git")
                .args([
                    "-C",
                    tool_dir.to_str().unwrap(),
                    "merge-base",
                    "--is-ancestor",
                    &remote_main,
                    &head,
                ])
                .status()
                .unwrap_or_else(|error| panic!("{tool}: failed to compare commits: {error}"));

            if !status.success() {
                failures.push(format!(
                    "{tool}: pinned commit {head} is behind or diverged from origin/main {remote_main}"
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "pinned-main-parity non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
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
