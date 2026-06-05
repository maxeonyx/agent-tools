//! # Auto-update Integration
//!
//! Auto-update is not real until there is an implementation, every applicable
//! tool calls it at startup, and tests cover the behavior.
//!
//! The dependency-only check is too weak: an unused dependency can make the
//! standard pass while users still run stale binaries. Compliance here checks
//! for a non-empty shared updater implementation, a tool source call site, and
//! updater test coverage.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["tdd-ratchet"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};

    #[test]
    fn auto_update_integration() {
        let mut failures = Vec::new();

        let updater = workspace_root().join("crates/agent-tools-updater/src/lib.rs");
        let updater_content = std::fs::read_to_string(&updater)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", updater.display()));
        if !updater_content.contains("pub fn") && !updater_content.contains("pub async fn") {
            failures.push("agent-tools-updater: no public updater API implemented".to_string());
        }
        if !updater_content.contains("#[test]") && !updater_content.contains("#[tokio::test]") {
            failures.push("agent-tools-updater: no updater tests found".to_string());
        }

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            if !cargo_toml.contains("agent-tools-updater") {
                failures.push(format!("{tool}: Cargo.toml missing agent-tools-updater"));
            }

            let source = source_tree_contents(tool);
            if !source.contains("agent_tools_updater::") {
                failures.push(format!(
                    "{tool}: no source call site for agent_tools_updater"
                ));
            }

            let tests = test_tree_contents(tool);
            if !tests.contains("agent_tools_updater") && !tests.contains("auto_update") {
                failures.push(format!("{tool}: no auto-update test coverage found"));
            }
        }

        if !failures.is_empty() {
            panic!(
                "auto-update-integration non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn source_tree_contents(tool: &str) -> String {
        tree_contents(tools_dir().join(tool).join("src"))
    }

    fn test_tree_contents(tool: &str) -> String {
        tree_contents(tools_dir().join(tool).join("tests"))
    }

    fn tree_contents(root: std::path::PathBuf) -> String {
        let mut content = String::new();
        collect_rs_contents(&root, &mut content);
        content
    }

    fn collect_rs_contents(path: &std::path::Path, content: &mut String) {
        let Ok(metadata) = std::fs::metadata(path) else {
            return;
        };

        if metadata.is_file() {
            if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
                content.push_str(&std::fs::read_to_string(path).unwrap_or_default());
            }
            return;
        }

        let entries = std::fs::read_dir(path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for entry in entries.filter_map(Result::ok) {
            collect_rs_contents(&entry.path(), content);
        }
    }
}
