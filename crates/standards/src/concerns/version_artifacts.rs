//! # Version Artifacts
//!
//! Every published tool should expose the same version information through its
//! CLI and its website package.
//!
//! Compliance means:
//! - the tool website package has `docs/version.json` which will publish as
//!   site-root `/version.json`
//! - `tool --version` prints `<binary> <version>`
//! - `tool --version --json` prints machine-readable JSON with `package`,
//!   `binary`, and `version`
//! - the umbrella site package has `docs/version.json` listing the current
//!   tool versions recorded in this workspace

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::path::Path;

    #[test]
    fn version_artifacts() {
        let mut failures = Vec::new();
        let mut expected_versions = BTreeMap::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            let package = package_field(&cargo_toml, "name")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package name"));
            let version = package_field(&cargo_toml, "version")
                .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package version"));
            let binary = binary_name(&cargo_toml).unwrap_or(package);
            expected_versions.insert(tool.to_string(), version.to_string());

            check_tool_website_json(
                tool,
                &tool_dir.join("docs/version.json"),
                package,
                binary,
                version,
                &mut failures,
            );
            check_cli_version_output(package, binary, version, &mut failures);
        }

        check_workspace_website_json(&expected_versions, &mut failures);

        if !failures.is_empty() {
            panic!(
                "version-artifacts non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn check_workspace_website_json(
        expected_versions: &BTreeMap<String, String>,
        failures: &mut Vec<String>,
    ) {
        let path = workspace_root().join("docs/version.json");
        let value = match read_json(&path) {
            Ok(value) => value,
            Err(error) => {
                failures.push(format!("workspace: docs/version.json {error}"));
                return;
            }
        };

        if value.get("site").and_then(Value::as_str) != Some("agent-tools") {
            failures.push("workspace: docs/version.json missing site=agent-tools".to_string());
        }

        let Some(tools) = value.get("tools").and_then(Value::as_object) else {
            failures.push("workspace: docs/version.json missing tools object".to_string());
            return;
        };

        for (tool, version) in expected_versions {
            if tools.get(tool).and_then(Value::as_str) != Some(version.as_str()) {
                failures.push(format!(
                    "workspace: docs/version.json tool {tool} expected version {version}"
                ));
            }
        }
    }

    fn check_tool_website_json(
        tool: &str,
        path: &Path,
        package: &str,
        binary: &str,
        version: &str,
        failures: &mut Vec<String>,
    ) {
        let value = match read_json(path) {
            Ok(value) => value,
            Err(error) => {
                failures.push(format!("{tool}: docs/version.json {error}"));
                return;
            }
        };

        if value.get("package").and_then(Value::as_str) != Some(package) {
            failures.push(format!(
                "{tool}: docs/version.json package expected {package}"
            ));
        }
        if value.get("binary").and_then(Value::as_str) != Some(binary) {
            failures.push(format!(
                "{tool}: docs/version.json binary expected {binary}"
            ));
        }
        if value.get("version").and_then(Value::as_str) != Some(version) {
            failures.push(format!(
                "{tool}: docs/version.json version expected {version}"
            ));
        }
    }

    fn check_cli_version_output(
        package: &str,
        binary: &str,
        version: &str,
        failures: &mut Vec<String>,
    ) {
        build_binary(package, binary, failures);

        match binary_stdout(binary, &["--version"]) {
            Ok(stdout) => {
                let expected = format!("{binary} {version}");
                if stdout.trim_end() != expected {
                    failures.push(format!(
                        "{package}: --version expected `{expected}`, got `{}`",
                        stdout.trim_end()
                    ));
                }
            }
            Err(error) => failures.push(format!("{package}: --version {error}")),
        }

        match binary_stdout(binary, &["--version", "--json"]) {
            Ok(stdout) => match serde_json::from_str::<Value>(&stdout) {
                Ok(value) => {
                    if value.get("package").and_then(Value::as_str) != Some(package) {
                        failures.push(format!(
                            "{package}: --version --json package expected {package}"
                        ));
                    }
                    if value.get("binary").and_then(Value::as_str) != Some(binary) {
                        failures.push(format!(
                            "{package}: --version --json binary expected {binary}"
                        ));
                    }
                    if value.get("version").and_then(Value::as_str) != Some(version) {
                        failures.push(format!(
                            "{package}: --version --json version expected {version}"
                        ));
                    }
                }
                Err(error) => failures.push(format!(
                    "{package}: --version --json emitted invalid JSON: {error}"
                )),
            },
            Err(error) => failures.push(format!("{package}: --version --json {error}")),
        }
    }

    fn build_binary(package: &str, binary: &str, failures: &mut Vec<String>) {
        let output = std::process::Command::new("cargo")
            .args(["build", "--quiet", "-p", package, "--bin", binary])
            .current_dir(workspace_root())
            .output()
            .unwrap_or_else(|error| panic!("failed to start cargo build for {package}: {error}"));

        if !output.status.success() {
            failures.push(format!(
                "{package}: cargo build failed with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
    }

    fn binary_stdout(binary: &str, args: &[&str]) -> Result<String, String> {
        let output = std::process::Command::new(workspace_root().join("target/debug").join(binary))
            .args(args)
            .current_dir(workspace_root())
            .output()
            .map_err(|error| format!("failed to run built binary: {error}"))?;

        if !output.status.success() {
            return Err(format!(
                "failed with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn read_json(path: &Path) -> Result<Value, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|error| format!("missing or unreadable: {error}"))?;
        serde_json::from_str(&content).map_err(|error| format!("invalid JSON: {error}"))
    }

    fn package_field<'a>(cargo_toml: &'a str, field: &str) -> Option<&'a str> {
        let prefix = format!("{field} = \"");
        cargo_toml.lines().find_map(|line| {
            line.strip_prefix(&prefix)?
                .split_once('"')
                .map(|(value, _)| value)
        })
    }

    fn binary_name<'a>(cargo_toml: &'a str) -> Option<&'a str> {
        let mut in_bin = false;
        for line in cargo_toml.lines() {
            let trimmed = line.trim();
            if trimmed == "[[bin]]" {
                in_bin = true;
                continue;
            }
            if in_bin && trimmed.starts_with('[') {
                in_bin = false;
            }
            if in_bin {
                if let Some(value) = trimmed.strip_prefix("name = \"") {
                    return value.split_once('"').map(|(name, _)| name);
                }
            }
        }
        None
    }
}
