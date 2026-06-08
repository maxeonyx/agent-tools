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
Review the tool's help text as a user trying to complete real work without
opening the source.

Required review method:
1. Run the top-level `--help`, every documented subcommand `--help`, and the
   examples shown in help where they are safe to run.
2. Compare option names, examples, and tone with sibling tools in this suite.
3. Produce findings with the exact command and help text location. If there are
   no findings, say which help surfaces and examples you exercised.

Check for:
1. The help explains what the tool is for, when to use it, and the expected
   workflow without becoming a manual.
2. Examples are copy-pasteable, realistic, and use long flags unless a short
   flag is explicitly allowed by this concern.
3. Subcommand help stands on its own and does not require reading top-level help
   first.
4. Terminology, flag names, exit behavior, and output descriptions are
   consistent with sibling tools.
5. The first-time path is obvious and dangerous or destructive operations are
   clearly signposted.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "help-text",
    definition_summary:
        "Each tool needs tested help examples and a current help-text quality review.",
    review_instructions: REVIEW_INSTRUCTIONS,
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
            "help-text",
            NOT_APPLICABLE,
        ));

        if !failures.is_empty() {
            panic!("help-text non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
