//! # Trusted TDD Ledger
//!
//! `.test-status.json` controls which failures are accepted, so ordinary pull
//! request code must never be able to edit it or choose the ratchet that
//! validates its transition. A trusted workflow must validate untrusted code
//! with base-controlled, pinned ratchet logic, pass only the proposed hidden
//! ledger to a separately privileged job, revalidate the transition against the
//! current pull-request head, and create a non-force ledger-only bot commit.
//!
//! Compliance is structural because the security boundary is the workflow
//! definition itself. GitHub Actions syntax is independently exercised by the
//! actionlint-backed environment concern.

/// Repositories where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["oc"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "trusted-tdd-ledger",
    definition_summary: "TDD ledger transitions must be validated by base-controlled code and written only by a least-privilege trusted bot workflow.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to the umbrella and every maintained ratcheted tool; archived oc is not migrated.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{checked_tools, tools_dir, workspace_root};
    use std::path::{Path, PathBuf};

    #[test]
    fn trusted_tdd_ledger() {
        let workspace = workspace_root();
        let mut failures = repository_failures("workspace", &workspace, false);
        if let Ok(workflow) =
            std::fs::read_to_string(workspace.join(".github/workflows/ledger.yml"))
        {
            failures.extend(workspace_runtime_failures("workspace", &workflow));
        }

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(repository_failures(
                tool,
                &tools_dir().join(tool),
                tool == "tdd-ratchet",
            ));
        }

        if !failures.is_empty() {
            panic!(
                "trusted-tdd-ledger non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    #[test]
    fn canonical_fixture_passes() {
        let failures = repository_failures("fixture", &fixture("pass"), false);
        assert!(
            failures.is_empty(),
            "canonical fixture should pass: {failures:?}"
        );
    }

    #[test]
    fn unsafe_fixture_is_rejected() {
        let failures = repository_failures("fixture", &fixture("fail-unsafe"), false);
        assert!(failures
            .iter()
            .any(|failure| failure.contains("pull_request_target")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("credentials")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("current pull-request head")));
        assert!(failures.iter().any(|failure| failure.contains("non-force")));
    }

    fn workspace_runtime_failures(repo: &str, workflow: &str) -> Vec<String> {
        let mut failures = Vec::new();
        for (needle, message) in [
            (
                "cachix/install-nix-action@",
                "must install Nix before entering the declared devenv",
            ),
            (
                "cachix/cachix-action@",
                "must use devenv's public binary cache",
            ),
            (
                "github:cachix/devenv/v1.4.1",
                "must install the workspace's exact devenv version",
            ),
            (
                "git submodule foreach --recursive",
                "must make nested public-repository checks credential-free",
            ),
            (
                "git remote set-url origin \"$https_url\"",
                "must replace nested SSH remotes with public HTTPS remotes",
            ),
            (
                "GH_TOKEN: ${{ github.token }}",
                "must provide the read-only GitHub token used by concerns",
            ),
            (
                "devenv shell -- ../trusted-ratchet-source/target/release/cargo-ratchet",
                "must run the ratchet inside the workspace's declared environment",
            ),
        ] {
            require(repo, workflow, needle, message, &mut failures);
        }
        failures
    }

    fn repository_failures(repo: &str, repo_dir: &Path, self_hosting: bool) -> Vec<String> {
        let mut failures = Vec::new();
        let workflow_path = repo_dir.join(".github/workflows/ledger.yml");
        let workflow = match std::fs::read_to_string(&workflow_path) {
            Ok(workflow) => workflow,
            Err(error) => {
                failures.push(format!(
                    "{repo}: trusted ledger workflow missing at {}: {error}",
                    workflow_path.display()
                ));
                return failures;
            }
        };
        let workflow = without_comment_lines(&workflow);

        for (needle, message) in [
            (
                "pull_request_target:",
                "must use the base-controlled pull_request_target event",
            ),
            (
                "github.event.pull_request.head.repo.full_name == github.repository",
                "must reject pull requests from forks with a same-repository restriction",
            ),
            ("permissions: {}", "must deny permissions by default"),
            (
                "group: ledger-pr-${{ github.event.pull_request.number }}",
                "must serialize each pull request independently",
            ),
            (
                "cancel-in-progress: false",
                "must not cancel an in-progress ledger transition",
            ),
            ("  validate:", "must have an unprivileged validate job"),
            ("  write:", "must have a separate privileged write job"),
            (
                "needs: validate",
                "write job must depend on successful validation",
            ),
            (
                "contents: read",
                "validate job must use read-only contents permission",
            ),
            (
                "contents: write",
                "write job must request its contents permission explicitly",
            ),
            (
                "ref: ${{ github.event.pull_request.head.sha }}",
                "must check out the exact untrusted pull-request head",
            ),
            ("path: pull-request", "must isolate the untrusted checkout"),
            (
                "path: trusted-ratchet-source",
                "must isolate the trusted ratchet source",
            ),
            (
                "taiki-e/install-action@nextest",
                "must install cargo-nextest for the canonical ratchet",
            ),
            (
                "cargo build --release --manifest-path trusted-ratchet-source/Cargo.toml",
                "must build the base-controlled ratchet",
            ),
            (
                "../trusted-ratchet-source/target/release/cargo-ratchet",
                "must run the built trusted ratchet against the untrusted checkout",
            ),
            (
                "actions/upload-artifact@",
                "must upload the ratchet-produced ledger",
            ),
            (
                "path: pull-request/.test-status.json",
                "artifact must contain only the proposed ledger",
            ),
            (
                "include-hidden-files: true",
                "hidden ledger artifact must be included explicitly",
            ),
            (
                "if-no-files-found: error",
                "missing proposed ledger must fail validation",
            ),
            (
                "actions/download-artifact@",
                "write job must consume the validated artifact",
            ),
            (
                "test \"$(find ledger-output -type f | wc -l)\" -eq 1",
                "write job must reject extra artifact files",
            ),
            (
                "all(.tests[]; . == \"pending\" or . == \"passing\")",
                "write job must enforce ledger status values",
            ),
            (
                "(has(\"removals\") | not)",
                "ratchet output must not smuggle removal instructions",
            ),
            (
                "(has(\"baseline\") | not)",
                "ratchet output must not smuggle baseline changes",
            ),
            (
                "--slurpfile previous previous-ledger.json",
                "privileged job must load the previous ledger",
            ),
            (
                "--slurpfile instructions instructions.json",
                "privileged job must load developer-owned instructions",
            ),
            (
                "def transition_ok($from; $to):",
                "privileged job must validate semantic state transitions",
            ),
            (
                "new tests must enter pending",
                "privileged job must reject new passing tests",
            ),
            (
                "removed tests require an instruction",
                "privileged job must reject undeclared test removal",
            ),
            (
                "passing tests cannot be downgraded",
                "privileged job must reject passing-to-pending downgrades",
            ),
            (
                ".commit.verification.verified",
                "ordinary ledger commits must have verified signatures",
            ),
            (
                ".author.login",
                "ordinary ledger commits must verify their author",
            ),
            (
                "'github-actions[bot]'",
                "ledger-only commits must be authored by github-actions[bot]",
            ),
            (
                ".committer.login",
                "ordinary ledger commits must verify their committer",
            ),
            (
                "'web-flow'",
                "ledger-only commits must be committed by web-flow",
            ),
            (
                "test \"$changed\" = '.test-status.json'",
                "bot-authored ledger commits must change only the ledger",
            ),
            (
                "current_head=$(gh pr view",
                "write job must query the current pull-request head",
            ),
            (
                "test \"$current_head\" = \"$HEAD_SHA\"",
                "write job must validate the current pull-request head",
            ),
            (
                "path: \".test-status.json\"",
                "created git tree must change only the ledger path",
            ),
            (
                "-F force=false",
                "branch update must be explicitly non-force",
            ),
        ] {
            require(repo, &workflow, needle, message, &mut failures);
        }

        if workflow.matches("persist-credentials: false").count() < 2 {
            failures.push(format!(
                "{repo}: trusted and untrusted checkouts must both disable persisted credentials"
            ));
        }

        if self_hosting {
            require(
                repo,
                &workflow,
                "ref: ${{ github.event.pull_request.base.sha }}",
                "self-hosting ratchet source must come from the exact reviewed base SHA",
                &mut failures,
            );
        } else {
            require(
                repo,
                &workflow,
                "repository: maxeonyx/tdd-ratchet-rs",
                "must check out the canonical ratchet repository",
                &mut failures,
            );
            require(
                repo,
                &workflow,
                &format!("ref: {}", ratchet_tag()),
                "must pin ratchet source to the workspace's exact version",
                &mut failures,
            );
        }

        let guide_path = repo_dir.join("AGENTS.md");
        let guide = match std::fs::read_to_string(&guide_path) {
            Ok(guide) => guide.to_lowercase(),
            Err(error) => {
                failures.push(format!(
                    "{repo}: trusted ledger guidance missing at {}: {error}",
                    guide_path.display()
                ));
                return failures;
            }
        };
        for (needle, message) in [
            ("cargo ratchet", "guidance must name cargo ratchet"),
            (
                "trusted ledger workflow",
                "guidance must name the trusted ledger workflow",
            ),
            (
                "bot commit",
                "guidance must require waiting for the bot commit",
            ),
            ("pending", "guidance must describe the pending state"),
            ("passing", "guidance must describe the passing state"),
        ] {
            require(repo, &guide, needle, message, &mut failures);
        }

        failures
    }

    fn require(repo: &str, content: &str, needle: &str, message: &str, failures: &mut Vec<String>) {
        if !content.contains(needle) {
            failures.push(format!("{repo}: {message}"));
        }
    }

    fn ratchet_tag() -> String {
        let manifest = std::fs::read_to_string(tools_dir().join("tdd-ratchet/Cargo.toml"))
            .expect("read pinned tdd-ratchet Cargo.toml");
        let version = crate::evidence::package_field(&manifest, "version")
            .expect("pinned tdd-ratchet Cargo.toml has a version");
        format!("v{version}")
    }

    fn without_comment_lines(content: &str) -> String {
        content
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn fixture(name: &str) -> PathBuf {
        workspace_root().join(format!(
            "crates/standards/src/concerns/trusted_tdd_ledger/fixtures/{name}"
        ))
    }
}
