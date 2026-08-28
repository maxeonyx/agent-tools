//! # Prominent TDD Ratchet Guidance
//!
//! Agents must understand the ratchet before they write or run a test. Every
//! maintained tool guide and the umbrella guide therefore explain, near the
//! beginning, that an expected red new test is a green CI state while a new
//! test that starts green is a ratchet violation.

pub const NOT_APPLICABLE: &[&str] = &[];

pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "tdd-ratchet-guidance",
    definition_summary:
        "The umbrella and every maintained tool must prominently explain the red-test, pending-commit, green-CI, and later-promotion semantics of tdd-ratchet.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note:
        "Applies to the umbrella context and every maintained tool because agents may begin work from either context.",
};

#[cfg(test)]
mod tests {
    use crate::{checked_tools, tools_dir, workspace_root};
    use std::path::PathBuf;

    const PROMINENT_LINE_LIMIT: usize = 40;

    #[test]
    fn tdd_ratchet_guidance() {
        let mut failures =
            guidance_failures("workspace", &read(workspace_root().join("AGENTS.md")));

        for tool in checked_tools() {
            failures.extend(guidance_failures(
                tool,
                &read(tools_dir().join(tool).join("AGENTS.md")),
            ));
        }

        if !failures.is_empty() {
            panic!(
                "tdd-ratchet-guidance non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn guidance_failures(repo: &str, guide: &str) -> Vec<String> {
        let prominent = guide
            .lines()
            .take(PROMINENT_LINE_LIMIT)
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        let required = [
            (
                "cargo ratchet",
                "name `cargo ratchet` as the test entrypoint",
            ),
            ("new test must be red", "say a new test must start red"),
            (
                "committed as `pending`",
                "require the red test to be committed as pending",
            ),
            (
                "expected red test keeps ci green",
                "explain that expected red keeps CI green",
            ),
            (
                "new test must not pass",
                "say a newly introduced test must not pass",
            ),
            (
                "promotion to `passing`",
                "put promotion to passing after implementation",
            ),
        ];

        required
            .iter()
            .filter_map(|(needle, explanation)| {
                (!prominent.contains(needle)).then(|| {
                    format!("{repo}: first {PROMINENT_LINE_LIMIT} lines must {explanation}")
                })
            })
            .collect()
    }

    fn read(path: PathBuf) -> String {
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
    }

    fn fixture(name: &str) -> String {
        read(
            workspace_root()
                .join("crates/standards/src/concerns/tdd_ratchet_guidance/fixtures")
                .join(name)
                .join("AGENTS.md"),
        )
    }

    #[test]
    fn compliant_guidance_fixture_passes() {
        assert!(guidance_failures("fixture", &fixture("pass")).is_empty());
    }

    #[test]
    fn incomplete_guidance_fixture_is_rejected() {
        let failures = guidance_failures("fixture", &fixture("fail-incomplete"));

        assert!(failures.iter().any(|failure| failure.contains("CI green")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("must not pass")));
        assert!(failures.iter().any(|failure| failure.contains("promotion")));
    }
}
