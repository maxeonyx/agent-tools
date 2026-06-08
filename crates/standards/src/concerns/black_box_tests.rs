//! # Test Presence
//!
//! Every shipped CLI tool must have an integration test surface.
//!
//! This is the mechanical companion to `black-box-test-quality`: it proves the
//! repo has a `tests/` directory and the standard subprocess test dependency.
//! The qualitative review decides whether those tests actually cover public
//! binary behavior well.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "tests-present",
    definition_summary: "Each tool must have a tests directory and subprocess test dependency.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to tool binaries; the workspace itself is not a shipped CLI product.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use std::path::Path;

    #[test]
    fn tests_present() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(test_presence_failures_for_tool(
                tool,
                &tools_dir().join(tool),
            ));
        }

        if !failures.is_empty() {
            panic!("tests-present non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn test_presence_failures_for_tool(tool: &str, tool_path: &Path) -> Vec<String> {
        let mut failures = Vec::new();

        if !tool_path.join("tests").is_dir() {
            failures.push(format!("{tool}: tests/ directory missing"));
        }

        let cargo_toml = tool_path.join("Cargo.toml");
        let cargo_toml_contents = std::fs::read_to_string(&cargo_toml)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", cargo_toml.display()));
        if !cargo_toml_contents.contains("assert_cmd") {
            failures.push(format!("{tool}: Cargo.toml missing assert_cmd"));
        }

        failures
    }
}
