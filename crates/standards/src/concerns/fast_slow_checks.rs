//! # Fast and Slow Check Separation
//!
//! Fast checks (fmt, clippy, in-memory or unit tests) across all repos must
//! complete in under 10 seconds on a warm build. Slow checks (black-box tests
//! that spawn subprocesses) run separately.
//!
//! This ensures tight feedback loops during development. If fast checks take too
//! long, agents and developers stop running them between edits.
//!
//! The mechanical enforcement: run fast checks across all tools and verify total
//! wall-clock time is under 10 seconds.
//!
//! Fast = `cargo test --lib -p <tool>`
//! Slow = `cargo test --test '*' -p <tool>`

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
const MAX_FAST_CHECK_DURATION: std::time::Duration = std::time::Duration::from_secs(10);

#[cfg(test)]
mod tests {
    use super::{MAX_FAST_CHECK_DURATION, NOT_APPLICABLE};
    use crate::{workspace_root, TOOLS};
    use std::time::Instant;

    #[test]
    fn fast_slow_checks() {
        let start = Instant::now();
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let output = std::process::Command::new("cargo")
                .args(["test", "--lib", "-p", tool, "--", "--quiet"])
                .current_dir(workspace_root())
                .output()
                .unwrap_or_else(|error| {
                    panic!("failed to run cargo test --lib -p {tool}: {error}")
                });

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("no library targets") {
                    failures.push(format!("{tool}: fast tests failed"));
                }
            }
        }

        let elapsed = start.elapsed();

        if !failures.is_empty() {
            panic!(
                "fast-slow-checks non-compliant (test failures):\n  {}",
                failures.join("\n  ")
            );
        }

        if elapsed > MAX_FAST_CHECK_DURATION {
            panic!(
                "fast-slow-checks non-compliant: fast checks took {:.1}s (limit: {}s)",
                elapsed.as_secs_f64(),
                MAX_FAST_CHECK_DURATION.as_secs()
            );
        }
    }
}
