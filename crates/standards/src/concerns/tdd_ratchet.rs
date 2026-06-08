//! # TDD Ratchet Enforcement
//!
//! New tests must fail before they pass. Once a test passes, it must keep
//! passing.
//!
//! Tests that never failed might test the wrong thing. Tests written after the
//! code don't prove the code is what made them pass — they could pass for
//! accidental reasons. The ratchet catches tests that don't test anything real,
//! regressions, and silent test removal.
//!
//! To comply, `cargo ratchet` must pass in the tool repo, and plain
//! `cargo test` must fail in the same repo. The first proves the ratchet is the
//! working test entrypoint; the second proves the bypass-prevention gate is
//! active.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "tdd-ratchet",
    definition_summary:
        "Each tool repo must pass cargo ratchet and reject plain cargo test bypasses.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to tool repos with their own ratchet gate, not to the workspace root.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use std::path::Path;
    use std::process::{Command, Output};

    #[test]
    fn tdd_ratchet() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(tdd_ratchet_failures_for_tool(tool, &tools_dir().join(tool)));
        }

        if !failures.is_empty() {
            panic!("tdd-ratchet non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn tdd_ratchet_failures_for_tool(tool: &str, tool_dir: &Path) -> Vec<String> {
        let mut failures = Vec::new();

        match run_cargo(tool_dir, &["ratchet"]) {
            Ok(output) if output.status.success() => {}
            Ok(output) => failures.push(format!(
                "{tool}: cargo ratchet failed{}",
                output_detail(&output)
            )),
            Err(error) => failures.push(format!("{tool}: failed to run cargo ratchet: {error}")),
        }

        match run_cargo(tool_dir, &["test"]) {
            Ok(output) if output.status.success() => failures.push(format!(
                "{tool}: plain cargo test passed; expected bypass-prevention failure"
            )),
            Ok(_) => {}
            Err(error) => failures.push(format!("{tool}: failed to run plain cargo test: {error}")),
        }

        failures
    }

    fn run_cargo(tool_dir: &Path, args: &[&str]) -> std::io::Result<Output> {
        Command::new("cargo")
            .args(args)
            .current_dir(tool_dir)
            .output()
    }

    fn output_detail(output: &Output) -> String {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        output_detail_from_text(&stderr, &stdout)
    }

    fn output_detail_from_text(stderr: &str, stdout: &str) -> String {
        let lines: Vec<_> = stderr
            .lines()
            .chain(stdout.lines())
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            String::new()
        } else {
            let start = lines.len().saturating_sub(3);
            format!(":\n    {}", lines[start..].join("\n    "))
        }
    }

    #[test]
    fn output_detail_uses_tail_non_empty_lines() {
        let detail = output_detail_from_text(
            "Compiling crate\n\nerror: real failure\nnext action\n",
            "ignored older stdout\n",
        );

        assert_eq!(
            detail,
            ":\n    error: real failure\n    next action\n    ignored older stdout"
        );
    }
}
