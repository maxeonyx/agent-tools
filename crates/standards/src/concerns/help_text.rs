//! # Help Text Standards
//!
//! Every tool must have help text tests that verify `--help` examples are
//! runnable, and every tool must have a recorded quality review of that help
//! text.
//!
//! Requirements:
//! - `--help` at top level exits 0 and contains an Examples section
//! - Subcommands (where they exist) each have their own `--help`
//! - Examples are in `$ command args` format and are parsed and executed by the
//!   tool's own test suite via the shared `help-test` crate
//! - Examples must use long flags (self-documenting), not short flags
//! - Help text is succinct, explains why and when (not just what)
//!
//! ## Short-flag allowlist (per tool)
//!
//! Some short flags are self-explanatory and may appear in examples:
//! - dotsync: `-m` (message, like git commit -m)

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the tool's help text as a new user would:
1. Read the top-level help and any important subcommand help
2. Is it succinct?
3. Does it explain why and when to use the tool, not just what flags exist?
4. Are flag names consistent with sibling tools in the suite?
5. Would a new user understand how to get started from the help alone?
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "help-text",
    definition_summary:
        "Each tool needs tested help examples and a current help-text quality review.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: Some("docs/reviews/help-text.json"),
    applies_to_workspace: false,
    applicability_note: "Applies to user-facing tool CLIs rather than the workspace control plane.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{concerns, tools_dir, TOOLS};
    use std::fs;

    #[test]
    fn help_text() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_path = tools_dir().join(tool);
            let cargo_toml = tool_path.join("Cargo.toml");

            let cargo_toml_content = fs::read_to_string(&cargo_toml)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", cargo_toml.display()));

            if !cargo_toml_content.contains("help-test") {
                failures.push(format!(
                    "{tool}: Cargo.toml missing help-test dev-dependency"
                ));
            }

            let tests_dir = tool_path.join("tests");
            if !tests_dir.exists() {
                failures.push(format!("{tool}: no tests/ directory"));
            } else {
                let has_help_tests = fs::read_dir(&tests_dir)
                    .unwrap_or_else(|error| {
                        panic!("failed to read {}: {error}", tests_dir.display())
                    })
                    .filter_map(|entry| entry.ok())
                    .any(|entry| {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if !name.ends_with(".rs") {
                            return false;
                        }
                        let content = fs::read_to_string(entry.path()).unwrap_or_default();
                        content.contains("help_test::")
                    });

                if !has_help_tests {
                    failures.push(format!(
                        "{tool}: no help text tests found (need a test file in tests/ using help_test::)"
                    ));
                }
            }
        }

        failures.extend(concerns::review_attestation_failures(
            "docs/reviews/help-text.json",
            NOT_APPLICABLE,
        ));

        if !failures.is_empty() {
            panic!("help-text non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
