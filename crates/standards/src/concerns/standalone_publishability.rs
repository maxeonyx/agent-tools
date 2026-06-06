//! # Standalone Publishability
//!
//! Tool repositories are standalone release repos. Their CI checks out the tool
//! repo, not this umbrella workspace, so release builds must not depend on
//! workspace-relative path crates that are missing in the standalone checkout.
//!
//! Compliance means each tool repo can be built from its own checkout without
//! dependencies that point outside the repo.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "standalone-publishability",
    definition_summary: "Standalone tool repos must not depend on workspace-relative path crates for release builds.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: None,
    applies_to_workspace: false,
    applicability_note: "Applies to standalone tool publishing, not to the umbrella workspace checkout.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn standalone_publishability() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let cargo_toml_path = tools_dir().join(tool).join("Cargo.toml");
            let cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap_or_else(|error| {
                panic!("failed to read {}: {error}", cargo_toml_path.display())
            });

            for line in cargo_toml.lines() {
                if line.contains("path = \"../") || line.contains("path = \"../../") {
                    failures.push(format!(
                        "{tool}: Cargo.toml has workspace-relative dependency: {}",
                        line.trim()
                    ));
                }
            }
        }

        if !failures.is_empty() {
            panic!(
                "standalone-publishability non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
