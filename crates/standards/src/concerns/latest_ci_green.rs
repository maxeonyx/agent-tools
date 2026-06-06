//! # Latest CI Green
//!
//! The workspace should only point at tool commits that have a successful
//! `main` CI run.
//!
//! Release and website checks validate public artifacts. They do not prove the
//! current recorded tool head passed its own repository CI. Compliance here is
//! remote: for each tool repo, there must be a completed successful `ci.yml`
//! run on `main` for the exact commit recorded in this workspace.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use serde_json::Value;

    #[test]
    fn latest_ci_green() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            let repo_url = package_field(&cargo_toml, "repository")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing repository URL"));
            let repo = repo_url
                .strip_prefix("https://github.com/")
                .unwrap_or_else(|| panic!("{tool}: repository is not a GitHub URL"));
            let head = command_stdout(
                "git",
                &["-C", tool_dir.to_str().unwrap(), "rev-parse", "HEAD"],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read HEAD: {error}"));

            let runs = command_stdout(
                "gh",
                &[
                    "run",
                    "list",
                    "--repo",
                    repo,
                    "--workflow",
                    "ci.yml",
                    "--branch",
                    "main",
                    "--commit",
                    &head,
                    "--limit",
                    "1",
                    "--json",
                    "headSha,status,conclusion,url,displayTitle",
                ],
            )
            .unwrap_or_else(|error| panic!("{tool}: failed to read CI runs: {error}"));

            let parsed: Value = serde_json::from_str(&runs)
                .unwrap_or_else(|error| panic!("{tool}: invalid gh run list JSON: {error}"));
            let Some(run) = parsed.as_array().and_then(|runs| runs.first()) else {
                failures.push(format!(
                    "{tool}: no successful ci.yml run found for pinned commit {head} on main"
                ));
                continue;
            };

            let run_sha = run
                .get("headSha")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let status = run
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let conclusion = run
                .get("conclusion")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let url = run.get("url").and_then(Value::as_str).unwrap_or_default();

            if run_sha != head {
                failures.push(format!(
                    "{tool}: ci.yml run lookup returned {run_sha}, expected pinned commit {head} ({url})"
                ));
                continue;
            }

            if status != "completed" || conclusion != "success" {
                failures.push(format!(
                    "{tool}: latest main ci.yml run for {head} is status={status} conclusion={conclusion} ({url})"
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
