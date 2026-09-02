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
//! To comply, `cargo ratchet` must be the test entrypoint in the umbrella and
//! every maintained tool, and plain `cargo test` must be rejected by a
//! gatekeeper. The umbrella cannot recursively invoke its own complete ratchet
//! while this concern is running, so its check proves the committed ledger,
//! pinned source wrapper, offline entrypoint, and gatekeeper structurally and
//! executes the gatekeeper in isolation. The outer ratchet run is the outcome
//! evidence for the complete umbrella suite.
//!
//! The local invariant above is checked against the workspace-pinned
//! `tools/tdd-ratchet` source, never an ambient or stale installation. That is
//! necessary but not sufficient: a tool's CI can still drift
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
        "The umbrella and each tool repo must use cargo ratchet and reject plain cargo test bypasses.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to the workspace standards suite and every maintained tool because all test and concern changes require failing-first history.",
};

#[cfg(test)]
mod tests {
    use super::{NOT_APPLICABLE, SPEC};
    use crate::{checked_tools, tools_dir, workspace_root};
    use std::path::Path;
    use std::process::{Command, Output};

    #[test]
    fn workspace_is_in_tdd_ratchet_scope() {
        assert!(
            SPEC.applies_to_workspace,
            "the umbrella standards suite must dogfood tdd-ratchet"
        );
    }

    #[test]
    fn tdd_ratchet() {
        let mut failures = tdd_ratchet_failures_for_workspace(&workspace_root());

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(tdd_ratchet_failures_for_tool(tool, &tools_dir().join(tool)));
        }

        if !failures.is_empty() {
            panic!("tdd-ratchet non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn tdd_ratchet_failures_for_workspace(workspace: &Path) -> Vec<String> {
        let mut failures = Vec::new();

        let ledger = read(workspace.join(".test-status.json"));
        if !ledger.contains("test-status.v1.json") {
            failures.push("workspace: .test-status.json schema-v1 ledger missing".to_string());
        }

        let standards = read(workspace.join("crates/standards/src/lib.rs"));
        if !standards.contains("fn tdd_ratchet_gatekeeper()") || !standards.contains("TDD_RATCHET")
        {
            failures.push("workspace: standards gatekeeper test missing".to_string());
        }

        let devenv = read(workspace.join("devenv.nix"));
        for required in [
            "writeShellScriptBin \"cargo-ratchet\"",
            "tools/tdd-ratchet/Cargo.toml",
            "cargo ratchet",
        ] {
            if !devenv.contains(required) {
                failures.push(format!(
                    "workspace: devenv.nix missing pinned ratchet entrypoint `{required}`"
                ));
            }
        }
        if devenv.lines().any(|line| {
            let line = line.trim();
            !line.starts_with('#') && line.starts_with("cargo test")
        }) {
            failures.push(
                "workspace: devenv.nix runs cargo test directly instead of cargo ratchet"
                    .to_string(),
            );
        }

        match Command::new("cargo")
            .args(["test", "-p", "standards", "tdd_ratchet_gatekeeper"])
            .env_remove("TDD_RATCHET")
            .current_dir(workspace)
            .output()
        {
            Ok(output)
                if !output.status.success()
                    && (String::from_utf8_lossy(&output.stdout)
                        .contains("Run cargo ratchet instead of cargo test.")
                        || String::from_utf8_lossy(&output.stderr)
                            .contains("Run cargo ratchet instead of cargo test.")) => {}
            Ok(output) => failures.push(format!(
                "workspace: plain cargo test did not fail through the ratchet gatekeeper{}",
                output_detail(&output)
            )),
            Err(error) => failures.push(format!(
                "workspace: failed to exercise plain cargo test gatekeeper: {error}"
            )),
        }

        failures
    }

    fn read(path: impl AsRef<Path>) -> String {
        std::fs::read_to_string(path).unwrap_or_default()
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

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
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

        let installs_source = ci.contains("cargo install --path")
            && (tool == "tdd-ratchet" || ci.contains("tdd-ratchet"));
        if !installs_source {
            failures.push(format!(
                "{tool}: ci.yml must install cargo-ratchet from source (cargo install --path ...tdd-ratchet)"
            ));
        }
        let installs_nextest = ci.contains("cargo install cargo-nextest")
            || ci.contains("taiki-e/install-action@nextest");
        if !installs_nextest {
            failures.push(format!(
                "{tool}: ci.yml must install cargo-nextest from source or the maintained prebuilt action (the ratchet shells out to nextest)"
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

        match run_workspace_ratchet(tool_dir) {
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
            .env_remove("TDD_RATCHET")
            .current_dir(tool_dir)
            .output()
    }

    fn run_workspace_ratchet(tool_dir: &Path) -> std::io::Result<Output> {
        Command::new("cargo")
            .args(["run", "--quiet", "--manifest-path"])
            .arg(tools_dir().join("tdd-ratchet/Cargo.toml"))
            .arg("--")
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
    fn ci_pattern_accepts_tdd_ratchet_installing_itself() {
        let ci = r#"
      - name: Install cargo-ratchet and cargo-nextest
        run: |
          cargo install --path . --locked
          cargo install cargo-nextest --locked
      - name: Run tests (ratchet)
        run: cargo ratchet
"#;

        assert!(ci_pattern_failures_from_text("tdd-ratchet", ci).is_empty());
    }

    #[test]
    fn ci_pattern_accepts_prebuilt_nextest_installer() {
        let ci = r#"
      - uses: taiki-e/install-action@nextest
      - name: Install cargo-ratchet
        run: cargo install --path ../../ratchet-install/tools/tdd-ratchet --locked
      - name: Run tests (ratchet)
        run: cargo ratchet
"#;

        assert!(ci_pattern_failures_from_text("prebuilt-nextest", ci).is_empty());
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
