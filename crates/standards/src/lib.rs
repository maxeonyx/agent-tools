//! Workspace compliance enforcement.
//!
//! This crate has no runtime purpose. Its tests verify that all tools
//! in the workspace meet cross-cutting standards. Run with `cargo test -p standards`.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn tools_dir() -> PathBuf {
    workspace_root().join("tools")
}

pub const TOOLS: &[&str] = &["trunc", "tb", "dotsync", "tdd-ratchet", "oc"];

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ComplianceEntry {
    Simple(String),
    WithCommit { status: String, checked: String },
}

impl ComplianceEntry {
    pub fn status(&self) -> &str {
        match self {
            Self::Simple(s) => s,
            Self::WithCommit { status, .. } => status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ConcernFrontmatter {
    pub title: String,
    #[serde(rename = "type")]
    pub concern_type: String,
    pub applies_to: String,
    pub checker: String,
}

pub fn load_concerns() -> HashMap<String, ConcernFrontmatter> {
    let concerns_dir = workspace_root().join("standards/concerns");
    let mut concerns = HashMap::new();

    for entry in std::fs::read_dir(&concerns_dir)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", concerns_dir.display()))
    {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "md") {
            let content = std::fs::read_to_string(&path).unwrap();
            if let Some(frontmatter) = extract_frontmatter(&content) {
                let fm: ConcernFrontmatter = serde_yaml::from_str(frontmatter)
                    .unwrap_or_else(|e| panic!("Bad frontmatter in {}: {e}", path.display()));
                let id = path.file_stem().unwrap().to_string_lossy().to_string();
                concerns.insert(id, fm);
            }
        }
    }

    concerns
}

fn extract_frontmatter(content: &str) -> Option<&str> {
    let content = content.strip_prefix("---\n")?;
    let end = content.find("\n---")?;
    Some(&content[..end])
}

pub fn load_compliance() -> HashMap<String, HashMap<String, ComplianceEntry>> {
    let path = workspace_root().join("standards/compliance.toml");
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
    toml::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tool_has_status_for_every_concern() {
        let concerns = load_concerns();
        let compliance = load_compliance();

        let mut missing = Vec::new();
        for tool in TOOLS {
            let tool_entry = compliance.get(*tool);
            if tool_entry.is_none() {
                missing.push(format!("Tool '{tool}' entirely missing from compliance.toml"));
                continue;
            }
            let tool_entry = tool_entry.unwrap();
            for concern_id in concerns.keys() {
                if !tool_entry.contains_key(concern_id) {
                    missing.push(format!("Tool '{tool}' missing status for '{concern_id}'"));
                }
            }
        }

        assert!(missing.is_empty(), "Compliance gaps:\n  {}", missing.join("\n  "));
    }

    #[test]
    fn all_statuses_are_valid() {
        let valid = ["compliant", "waived", "non-compliant", "not-applicable"];
        let compliance = load_compliance();

        let mut invalid = Vec::new();
        for tool in TOOLS {
            if let Some(tool_entry) = compliance.get(*tool) {
                for (concern, entry) in tool_entry {
                    let status = entry.status();
                    if !valid.contains(&status) {
                        invalid.push(format!("'{tool}'.'{concern}' = '{status}'"));
                    }
                }
            }
        }

        assert!(invalid.is_empty(), "Invalid statuses:\n  {}", invalid.join("\n  "));
    }

    #[test]
    fn workspace_routing_claims_match_reality() {
        let compliance = load_compliance();

        for tool in TOOLS {
            let tool_path = tools_dir().join(tool);
            let agents_file = tool_path.join("AGENTS.md");

            let status = compliance
                .get(*tool)
                .and_then(|t| t.get("workspace-routing"))
                .map(|e| e.status())
                .unwrap_or("???");

            if status == "compliant" {
                let content = std::fs::read_to_string(&agents_file).unwrap_or_default();
                let has_routing = content.contains("agent-tools")
                    && (content.contains("workspace") || content.contains("Workspace"));
                assert!(
                    has_routing,
                    "Tool '{tool}' claims workspace-routing compliant but AGENTS.md doesn't direct to workspace"
                );
            }
        }
    }

    #[test]
    fn tdd_ratchet_claims_match_reality() {
        let compliance = load_compliance();

        for tool in TOOLS {
            let status = compliance
                .get(*tool)
                .and_then(|t| t.get("tdd-ratchet"))
                .map(|e| e.status())
                .unwrap_or("???");

            if status == "not-applicable" {
                continue;
            }

            let tool_path = tools_dir().join(tool);

            if status == "compliant" {
                assert!(
                    tool_path.join(".test-status.json").exists(),
                    "Tool '{tool}' claims tdd-ratchet compliant but .test-status.json missing"
                );
            }
        }
    }

    #[test]
    fn black_box_test_claims_match_reality() {
        let compliance = load_compliance();

        for tool in TOOLS {
            let status = compliance
                .get(*tool)
                .and_then(|t| t.get("black-box-tests"))
                .map(|e| e.status())
                .unwrap_or("???");

            if status == "compliant" {
                let tool_path = tools_dir().join(tool);
                assert!(
                    tool_path.join("tests").exists(),
                    "Tool '{tool}' claims black-box-tests compliant but tests/ directory missing"
                );
                let cargo_content =
                    std::fs::read_to_string(tool_path.join("Cargo.toml")).unwrap_or_default();
                assert!(
                    cargo_content.contains("assert_cmd"),
                    "Tool '{tool}' claims black-box-tests compliant but no assert_cmd in Cargo.toml"
                );
            }
        }
    }

    #[test]
    fn auto_update_claims_match_reality() {
        let compliance = load_compliance();

        for tool in TOOLS {
            let status = compliance
                .get(*tool)
                .and_then(|t| t.get("auto-update"))
                .map(|e| e.status())
                .unwrap_or("???");

            if status == "compliant" {
                let tool_path = tools_dir().join(tool);
                let cargo_content =
                    std::fs::read_to_string(tool_path.join("Cargo.toml")).unwrap_or_default();
                assert!(
                    cargo_content.contains("agent-tools-updater"),
                    "Tool '{tool}' claims auto-update compliant but doesn't depend on agent-tools-updater"
                );
            }
        }
    }

    #[test]
    fn release_pipeline_claims_match_reality() {
        let compliance = load_compliance();

        for tool in TOOLS {
            let status = compliance
                .get(*tool)
                .and_then(|t| t.get("release-pipeline"))
                .map(|e| e.status())
                .unwrap_or("???");

            if status == "compliant" {
                let tool_path = tools_dir().join(tool);
                let ci_path = tool_path.join(".github/workflows/ci.yml");
                assert!(ci_path.exists(), "Tool '{tool}' claims release-pipeline compliant but no ci.yml");

                let ci_content = std::fs::read_to_string(&ci_path).unwrap_or_default();
                assert!(
                    ci_content.contains("cancel-in-progress: true"),
                    "Tool '{tool}' CI missing cancel-in-progress"
                );
                assert!(
                    ci_content.contains("gh release"),
                    "Tool '{tool}' CI missing release step"
                );
                assert!(
                    ci_content.contains("deploy-pages"),
                    "Tool '{tool}' CI missing pages deploy"
                );
            }
        }
    }
}
