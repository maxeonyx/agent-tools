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
//!
//! The local invariant above is checked against the ambient `cargo-ratchet`
//! binary. That is necessary but not sufficient: a tool's CI can still drift
//! out of the canonical pattern (e.g. a gatekeeper test gets added but CI still
//! runs plain `cargo test`, or a tool keeps an older bespoke ratchet script).
//! When that happens the local check is green while CI is structurally
//! guaranteed red, and nothing couples the two. So this concern ALSO checks
//! each tool's `.github/workflows/ci.yml` uses the canonical ratchet pattern:
//! install `cargo-ratchet` from source (latest) + `cargo-nextest`, run
//! `cargo ratchet`, and never run a bare `cargo test` / bespoke ratchet script
//! as the test step. The exemplar is dotsync's ci.yml.

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

    /// Each tool's CI must invoke the canonical ratchet pattern, not a bypass.
    ///
    /// Mirrors the runtime invariant from the CI side: catches a tool whose
    /// working tree passes `cargo ratchet` locally but whose `ci.yml` still
    /// runs plain `cargo test` (which would trip the gatekeeper) or a bespoke
    /// ratchet script. dotsync's ci.yml is the exemplar.
    #[test]
    fn ci_uses_canonical_ratchet_pattern() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let ci_path = tools_dir().join(tool).join(".github/workflows/ci.yml");
            failures.extend(ci_pattern_failures(tool, &ci_path));
        }

        if !failures.is_empty() {
            panic!(
                "tdd-ratchet CI pattern non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn ci_pattern_failures(tool: &str, ci_path: &Path) -> Vec<String> {
        let ci = match std::fs::read_to_string(ci_path) {
            Ok(text) => text,
            Err(error) => return vec![format!("{tool}: cannot read ci.yml: {error}")],
        };

        ci_pattern_failures_from_text(tool, &ci)
    }

    fn ci_pattern_failures_from_text(tool: &str, ci: &str) -> Vec<String> {
        let mut failures = Vec::new();

        if !ci.contains("cargo install --path") || !ci.contains("tdd-ratchet") {
            failures.push(format!(
                "{tool}: ci.yml must install cargo-ratchet from source (cargo install --path ...tdd-ratchet)"
            ));
        }
        if !ci.contains("cargo install cargo-nextest") {
            failures.push(format!(
                "{tool}: ci.yml must install cargo-nextest (the ratchet shells out to nextest)"
            ));
        }
        if !run_steps(ci).any(|step| step.contains("cargo ratchet")) {
            failures.push(format!(
                "{tool}: ci.yml must run `cargo ratchet` as the test step"
            ));
        }

        // Bypass paths: a `run:` step body that invokes the test runner
        // directly instead of going through the ratchet.
        for step in run_steps(ci) {
            if step.contains("cargo ratchet") {
                continue;
            }
            if runs_test_directly(step) {
                failures.push(format!(
                    "{tool}: ci.yml runs the test suite directly (bypassing the ratchet): `{}`",
                    step.trim()
                ));
            }
            if step.contains("scripts/ratchet.py") {
                failures.push(format!(
                    "{tool}: ci.yml runs a bespoke ratchet script (scripts/ratchet.py); use `cargo ratchet`"
                ));
            }
        }

        failures
    }

    /// Yield the body line(s) of each `run:` invocation in a workflow file.
    ///
    /// Cheap line-based scan rather than a full YAML parse — substring checks
    /// over `run:` bodies are enough to catch the structural drift we care
    /// about, and the procedural "CI observed green" gate backstops the rest.
    fn run_steps(ci: &str) -> impl Iterator<Item = &str> {
        ci.lines().filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .strip_prefix("- run:")
                .or_else(|| trimmed.strip_prefix("run:"))
                .map(str::trim)
        })
    }

    fn runs_test_directly(step: &str) -> bool {
        step.contains("cargo test")
            || step.contains("cargo nextest")
            || step.contains("cargo +nightly test")
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

    const CANONICAL_CI: &str = r#"
      - name: Prepare workspace path dependencies
        run: |
          git clone --depth=1 https://github.com/maxeonyx/tdd-ratchet-rs ../../ratchet-install/tools/tdd-ratchet
      - name: Install cargo-ratchet
        run: |
          cargo install --path ../../ratchet-install/tools/tdd-ratchet --locked
          cargo install cargo-nextest --locked
      - name: Run tests (ratchet)
        run: cargo ratchet
"#;

    #[test]
    fn ci_pattern_accepts_canonical_workflow() {
        let failures = ci_pattern_failures_from_text("exemplar", CANONICAL_CI);
        assert!(
            failures.is_empty(),
            "expected no failures, got: {failures:?}"
        );
    }

    #[test]
    fn ci_pattern_rejects_plain_cargo_test_step() {
        let ci = r#"
      - name: Install cargo-ratchet
        run: |
          cargo install --path ../../ratchet-install/tools/tdd-ratchet --locked
          cargo install cargo-nextest --locked
      - name: Run tests
        run: cargo test
"#;
        let failures = ci_pattern_failures_from_text("drifted", ci);
        assert!(
            failures.iter().any(|f| f.contains("bypassing the ratchet")),
            "expected a bypass failure, got: {failures:?}"
        );
        assert!(
            failures
                .iter()
                .any(|f| f.contains("must run `cargo ratchet`")),
            "expected a missing-ratchet-step failure, got: {failures:?}"
        );
    }

    #[test]
    fn ci_pattern_rejects_bespoke_python_ratchet() {
        let ci = r#"
      - name: Run test ratchet
        run: python3 scripts/ratchet.py
"#;
        let failures = ci_pattern_failures_from_text("python", ci);
        assert!(
            failures.iter().any(|f| f.contains("scripts/ratchet.py")),
            "expected a bespoke-script failure, got: {failures:?}"
        );
        assert!(
            failures.iter().any(|f| f.contains("install cargo-ratchet")),
            "expected a missing-install failure, got: {failures:?}"
        );
    }

    #[test]
    fn ci_pattern_requires_nextest_install() {
        let ci = r#"
      - name: Install cargo-ratchet
        run: cargo install --path ../../ratchet-install/tools/tdd-ratchet --locked
      - name: Run tests (ratchet)
        run: cargo ratchet
"#;
        let failures = ci_pattern_failures_from_text("no-nextest", ci);
        assert!(
            failures.iter().any(|f| f.contains("cargo-nextest")),
            "expected a missing-nextest failure, got: {failures:?}"
        );
    }
}
