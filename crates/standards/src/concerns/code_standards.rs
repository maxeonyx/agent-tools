//! # Code Standards
//!
//! Formatting and lint rules should stay shared across the suite.
//!
//! Inconsistent tool-local config creates unnecessary diffs and divergent idioms.
//! Shared config means one set of rules and one muscle memory when moving across
//! tools.
//!
//! Compliance means each tool passes `cargo fmt --check` when run from that
//! tool's repository, so rustfmt uses the effective configuration for the
//! checkout under test. Tool-local rustfmt configuration is reported as a drift
//! diagnostic, but formatting output is the policy.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "code-standards",
    definition_summary: "Tools must pass cargo fmt with their effective rustfmt configuration.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to Rust tool repos where cargo fmt is the shared formatter.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use std::process::Command;

    #[test]
    fn code_standards() {
        let mut failures = Vec::new();
        let mut diagnostics = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);

            let output = Command::new("cargo")
                .args(["fmt", "--check"])
                .current_dir(&tool_dir)
                .output()
                .unwrap_or_else(|error| {
                    panic!("failed to run cargo fmt --check for {tool}: {error}")
                });

            if !output.status.success() {
                failures.push(format!(
                    "{tool}: cargo fmt --check failed\n{}",
                    command_output(&output)
                ));
            }

            for local_config in ["rustfmt.toml", ".rustfmt.toml"] {
                if tool_dir.join(local_config).exists() {
                    diagnostics.push(format!(
                        "{tool}: diagnostic: local {local_config} present; confirm it intentionally matches suite formatting policy"
                    ));
                }
            }
        }

        if !diagnostics.is_empty() {
            eprintln!(
                "code-standards diagnostics:\n  {}",
                diagnostics.join("\n  ")
            );
        }

        if !failures.is_empty() {
            panic!("code-standards non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn command_output(output: &std::process::Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut parts = Vec::new();
        if !stdout.trim().is_empty() {
            parts.push(format!("stdout:\n{}", stdout.trim()));
        }
        if !stderr.trim().is_empty() {
            parts.push(format!("stderr:\n{}", stderr.trim()));
        }
        if parts.is_empty() {
            format!("exit status: {}", output.status)
        } else {
            parts.join("\n")
        }
    }
}
