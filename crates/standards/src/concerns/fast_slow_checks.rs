//! # Fast and Slow Check Separation
//!
//! Fast checks (fmt, clippy, in-memory or unit tests) must complete quickly for
//! each tool on a warm build. Slow checks (black-box tests that spawn
//! subprocesses) run separately.
//!
//! This ensures tight feedback loops during development. If fast checks take too
//! long, agents and developers stop running them between edits.
//!
//! The mechanical enforcement: run each tool's fast command twice and enforce
//! the second, warm run is under 5 seconds.
//!
//! Fast command pattern:
//! - library tools: `cargo test -p <tool> --lib -- --quiet`
//! - binary-only tools: `cargo test -p <tool> --bins -- --quiet`
//!
//! Slow entrypoint pattern:
//! - workspace: `cargo test -p <tool> --test '*'`
//! - standalone tool repo: `cargo test --test '*'`

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "fast-slow-checks",
    definition_summary:
        "Each tool should keep its fast verification loop under the wall-clock budget.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to per-tool check loops rather than the workspace control plane.",
};

#[cfg(test)]
const MAX_WARM_FAST_CHECK_DURATION: std::time::Duration = std::time::Duration::from_secs(5);

#[cfg(test)]
mod tests {
    use super::{MAX_WARM_FAST_CHECK_DURATION, NOT_APPLICABLE};
    use crate::{tools_dir, workspace_root, TOOLS};
    use std::time::Instant;

    #[test]
    fn fast_slow_checks() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let args = fast_command_args(tool);

            let warmup = run_fast_command(&args);
            if !warmup.status.success() {
                failures.push(format!(
                    "{tool}: warm-up fast command failed: cargo {}\n{}",
                    args.join(" "),
                    command_output(&warmup)
                ));
                continue;
            }

            let start = Instant::now();
            let warm = run_fast_command(&args);
            let elapsed = start.elapsed();

            if !warm.status.success() {
                failures.push(format!(
                    "{tool}: warm fast command failed: cargo {}\n{}",
                    args.join(" "),
                    command_output(&warm)
                ));
                continue;
            }

            if elapsed > MAX_WARM_FAST_CHECK_DURATION {
                failures.push(format!(
                    "{tool}: warm fast command took {:.1}s (limit: {}s): cargo {}",
                    elapsed.as_secs_f64(),
                    MAX_WARM_FAST_CHECK_DURATION.as_secs(),
                    args.join(" ")
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "fast-slow-checks non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn fast_command_args(tool: &str) -> Vec<&str> {
        if tools_dir().join(tool).join("src/lib.rs").exists() {
            vec!["test", "-p", tool, "--lib", "--", "--quiet"]
        } else {
            vec!["test", "-p", tool, "--bins", "--", "--quiet"]
        }
    }

    fn run_fast_command(args: &[&str]) -> std::process::Output {
        std::process::Command::new("cargo")
            .args(args)
            .current_dir(workspace_root())
            .output()
            .unwrap_or_else(|error| panic!("failed to run cargo {}: {error}", args.join(" ")))
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
