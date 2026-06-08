//! # Version Artifacts
//!
//! Every published tool should expose the same version information through its
//! CLI and its website package.
//!
//! Compliance means:
//! - the published tool website serves `/version.json`
//! - `tool --version` prints `<binary> <version>`
//! - `tool --version --json` prints machine-readable JSON with `package`,
//!   `binary`, and `version`
//! - the umbrella site package has `docs/version.json` listing the current
//!   tool versions recorded in this workspace, plus site identity and build
//!   metadata when available.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "version-artifacts",
    definition_summary: "Website packages and built binaries must expose machine-readable version artifacts.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to tool websites and binaries, and to the umbrella site package at the workspace root.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
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

            check_tool_website_json(tool, package, binary, version, &mut failures);
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
        if !value.get("git_commit").is_some_and(Value::is_string)
            && !value.get("commit").is_some_and(Value::is_string)
        {
            failures.push(
                "workspace: docs/version.json missing git_commit/commit metadata".to_string(),
            );
        }
        if !value.get("published_at").is_some_and(Value::is_string)
            && !value.get("built_at").is_some_and(Value::is_string)
        {
            failures.push(
                "workspace: docs/version.json missing published_at/built_at metadata".to_string(),
            );
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
        package: &str,
        binary: &str,
        version: &str,
        failures: &mut Vec<String>,
    ) {
        let Some(site_url) = tool_site_url(tool) else {
            failures.push(format!("{tool}: no public site URL found"));
            return;
        };
        let url = format!("{}/version.json", site_url.trim_end_matches('/'));
        let key = EvidenceKey::new("live-version-json", &url).tool(tool);
        let output = evidence::context().command(
            key,
            "curl",
            &["-fsSL", "--max-time", "10", &url],
            &workspace_root(),
        );
        if !output.status_success {
            failures.push(format!(
                "{tool}: live /version.json failed: {}",
                output.stderr.trim()
            ));
            return;
        }

        let value = match serde_json::from_str::<Value>(&output.stdout) {
            Ok(value) => value,
            Err(error) => {
                failures.push(format!("{tool}: live /version.json invalid JSON: {error}"));
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
        build_binary(package, binary, version, failures);

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

    fn build_binary(package: &str, binary: &str, version: &str, failures: &mut Vec<String>) {
        let key = EvidenceKey::new("build-binary", binary)
            .tool(package)
            .version(version);
        let output = evidence::context().command(
            key,
            "cargo",
            &["build", "--quiet", "-p", package, "--bin", binary],
            &workspace_root(),
        );

        if !output.status_success {
            failures.push(format!(
                "{package}: cargo build failed with status {}: {}",
                output.status,
                output.stderr.trim()
            ));
        }
    }

    fn binary_stdout(binary: &str, args: &[&str]) -> Result<String, String> {
        let mut key =
            EvidenceKey::new("built-binary-version", format!("{binary} {args:?}")).tool(binary);
        if let Ok(commit) = crate::evidence::tool_commit(&tools_dir().join(binary)) {
            key = key.commit(commit);
        }
        let binary_path = evidence::target_debug_binary(binary);
        let output = evidence::context().command(
            key,
            binary_path.to_str().unwrap_or(binary),
            args,
            &workspace_root(),
        );

        if !output.status_success {
            return Err(format!(
                "failed with status {}: {}",
                output.status,
                output.stderr.trim()
            ));
        }

        Ok(output.stdout)
    }

    fn read_json(path: &Path) -> Result<Value, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|error| format!("missing or unreadable: {error}"))?;
        serde_json::from_str(&content).map_err(|error| format!("invalid JSON: {error}"))
    }

    fn package_field<'a>(cargo_toml: &'a str, field: &str) -> Option<&'a str> {
        evidence::package_field(cargo_toml, field)
    }

    fn binary_name(cargo_toml: &str) -> Option<&str> {
        evidence::binary_name(cargo_toml)
    }

    fn tool_site_url(tool: &str) -> Option<String> {
        for file in [
            tools_dir().join(tool).join("docs/index.html"),
            tools_dir().join(tool).join("README.md"),
        ] {
            let Ok(content) = std::fs::read_to_string(file) else {
                continue;
            };
            if let Some(url) = release_site_url(&content) {
                return Some(url);
            }
            for token in content.split(|character: char| {
                character.is_whitespace() || matches!(character, '<' | '"' | '\'' | '`' | ')' | '(')
            }) {
                let url = token.trim_end_matches(|character: char| {
                    matches!(character, ',' | ';' | '.' | '\\')
                });
                if url.starts_with("https://")
                    && url.ends_with(".maxeonyx.com")
                    && !url.contains("tools.maxeonyx.com")
                {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    fn release_site_url(content: &str) -> Option<String> {
        for token in content.split(|character: char| {
            character.is_whitespace() || matches!(character, '<' | '"' | '\'' | '`' | ')' | '(')
        }) {
            let url = token
                .trim_end_matches(|character: char| matches!(character, ',' | ';' | '.' | '\\'));
            if url.starts_with("https://")
                && url.contains(".maxeonyx.com/releases/")
                && !url.contains("${")
            {
                let (site, _) = url.split_once("/releases/")?;
                return Some(site.to_string());
            }
        }
        None
    }
}
