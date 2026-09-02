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
        let mut failures = repository_failures("workspace", &workspace_root(), false);

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

    fn repository_failures(_repo: &str, _repo_dir: &Path, _self_hosting: bool) -> Vec<String> {
        panic!("red: trusted ledger workflow validation is not implemented")
    }

    fn fixture(name: &str) -> PathBuf {
        workspace_root().join(format!(
            "crates/standards/src/concerns/trusted_tdd_ledger/fixtures/{name}"
        ))
    }
}
