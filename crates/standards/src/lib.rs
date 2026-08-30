//! Workspace standards enforcement.
//!
//! This crate has no runtime purpose. Its concern modules define cross-cutting
//! standards and their mechanical tests fail until every applicable tool
//! complies. Run with `cargo test -p standards`.

use std::path::{Path, PathBuf};

pub mod concerns;
pub mod evidence;
pub mod review_attest;

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

pub fn libraries_dir() -> PathBuf {
    workspace_root().join("libraries")
}

pub const LIBRARIES: &[&str] = &["help-test"];

pub const TOOLS: &[&str] = &[
    "trunc",
    "tb",
    "dotsync",
    "tdd-ratchet",
    "oc",
    "agent-harness",
];

pub const MAINTAINED_TOOLS: &[&str] = &["trunc", "tb", "dotsync", "tdd-ratchet", "agent-harness"];

pub const ARCHIVED_TOOLS: &[&str] = &["oc"];

/// Tool repositories that should participate in this standards run.
///
/// Local clones may initialize only the submodules relevant to their task. CI
/// is the backstop: when `CI` is set, every configured tool must be initialized
/// and every concern sees the complete inventory.
pub fn checked_tools() -> std::vec::IntoIter<&'static str> {
    let ci = std::env::var_os("CI").is_some();
    select_tools(ci, |tool| tools_dir().join(tool).join(".git").exists())
        .unwrap_or_else(|error| panic!("tool inventory invalid: {error}"))
        .into_iter()
}

pub fn checked_libraries() -> std::vec::IntoIter<&'static str> {
    let ci = std::env::var_os("CI").is_some();
    select_libraries(ci, |library| {
        libraries_dir().join(library).join(".git").exists()
    })
    .unwrap_or_else(|error| panic!("library inventory invalid: {error}"))
    .into_iter()
}

fn select_libraries(
    ci: bool,
    initialized: impl Fn(&str) -> bool,
) -> Result<Vec<&'static str>, String> {
    if ci {
        let missing: Vec<_> = LIBRARIES
            .iter()
            .copied()
            .filter(|library| !initialized(library))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "CI must initialize every library submodule; missing: {}",
                missing.join(", ")
            ));
        }
        return Ok(LIBRARIES.to_vec());
    }

    Ok(LIBRARIES
        .iter()
        .copied()
        .filter(|library| initialized(library))
        .collect())
}

fn select_tools(ci: bool, initialized: impl Fn(&str) -> bool) -> Result<Vec<&'static str>, String> {
    if ci {
        let missing: Vec<_> = MAINTAINED_TOOLS
            .iter()
            .copied()
            .filter(|tool| !initialized(tool))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "CI must initialize every tool submodule; missing: {}",
                missing.join(", ")
            ));
        }
        return Ok(MAINTAINED_TOOLS.to_vec());
    }

    Ok(MAINTAINED_TOOLS
        .iter()
        .copied()
        .filter(|tool| initialized(tool))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{
        select_libraries, select_tools, ARCHIVED_TOOLS, LIBRARIES, MAINTAINED_TOOLS, TOOLS,
    };

    #[test]
    fn local_selection_contains_only_initialized_tools() {
        let selected = select_tools(false, |tool| matches!(tool, "trunc" | "tb")).unwrap();

        assert_eq!(selected, vec!["trunc", "tb"]);
    }

    #[test]
    fn ci_selection_contains_the_complete_inventory() {
        let selected = select_tools(true, |_| true).unwrap();

        assert_eq!(selected, MAINTAINED_TOOLS);
    }

    #[test]
    fn ci_selection_rejects_missing_submodules() {
        let error = select_tools(true, |tool| tool != "tb").unwrap_err();

        assert!(error.contains("CI must initialize every tool submodule"));
        assert!(error.contains("tb"));
    }

    #[test]
    fn ci_library_selection_requires_the_complete_inventory() {
        assert_eq!(select_libraries(true, |_| true).unwrap(), LIBRARIES);
        assert!(select_libraries(true, |_| false)
            .unwrap_err()
            .contains("CI must initialize every library submodule"));
    }

    #[test]
    fn maintained_and_archived_tools_partition_the_inventory() {
        let mut lifecycle_tools = MAINTAINED_TOOLS
            .iter()
            .chain(ARCHIVED_TOOLS)
            .copied()
            .collect::<Vec<_>>();
        lifecycle_tools.sort_unstable();

        let mut all_tools = TOOLS.to_vec();
        all_tools.sort_unstable();

        assert_eq!(lifecycle_tools, all_tools);
    }

    #[test]
    fn root_cargo_workspace_does_not_require_tool_submodules() {
        let manifest = std::fs::read_to_string(super::workspace_root().join("Cargo.toml"))
            .expect("read workspace Cargo.toml");

        for tool in TOOLS {
            assert!(
                manifest.contains(&format!("\"tools/{tool}\"")),
                "root Cargo workspace must explicitly exclude tool {tool}"
            );
        }
    }
}
