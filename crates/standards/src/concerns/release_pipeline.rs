//! # Release Pipeline Standards
//!
//! Each tool should have consistent CI: check → build → release → pages.
//!
//! Consistent pipelines mean consistent behavior: debounced runs, predictable
//! release mechanics, and automatic Pages deployment. When you fix a CI issue in
//! one tool, you should be able to apply the same fix across all tools without
//! re-learning each pipeline.
//!
//! Compliance here means a single `.github/workflows/ci.yml` exists and contains
//! the expected concurrency, release, and Pages deployment markers.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "release-pipeline",
    definition_summary:
        "Each tool repo must include the expected release and Pages workflow structure.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: None,
    applies_to_workspace: false,
    applicability_note: "Applies to standalone tool repos where release CI runs.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn release_pipeline() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let ci_path = tools_dir().join(tool).join(".github/workflows/ci.yml");
            if !ci_path.exists() {
                failures.push(format!("{tool}: .github/workflows/ci.yml missing"));
                continue;
            }

            let ci_contents = std::fs::read_to_string(&ci_path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", ci_path.display()));

            if !ci_contents.contains("cancel-in-progress: true") {
                failures.push(format!("{tool}: ci.yml missing cancel-in-progress: true"));
            }
            if !ci_contents.contains("gh release") {
                failures.push(format!("{tool}: ci.yml missing gh release"));
            }
            if !ci_contents.contains("deploy-pages") {
                failures.push(format!("{tool}: ci.yml missing deploy-pages"));
            }
        }

        if !failures.is_empty() {
            panic!(
                "release-pipeline non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
