//! # Code Standards
//!
//! Formatting and lint rules should stay shared across the suite.
//!
//! Inconsistent tool-local config creates unnecessary diffs and divergent idioms.
//! Shared config means one set of rules and one muscle memory when moving across
//! tools.
//!
//! The lightweight mechanical check here is that no tool overrides workspace
//! formatting with a local `rustfmt.toml`.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "code-standards",
    definition_summary:
        "Tools should inherit shared formatting standards rather than overriding them locally.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "This check is about tool repos not drifting from workspace rustfmt configuration.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn code_standards() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let rustfmt_toml = tools_dir().join(tool).join("rustfmt.toml");
            if rustfmt_toml.exists() {
                failures.push(format!("{tool}: local rustfmt.toml present"));
            }
        }

        if !failures.is_empty() {
            panic!("code-standards non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
