//! # Auto-update Mechanism
//!
//! Users install these tools once and forget. Without auto-update, they run
//! stale versions indefinitely.
//!
//! The intended mechanism is silent and best-effort: check for an already
//! downloaded update at startup, replace self if present, and check GitHub
//! Releases in the background for something newer. Failures in the update path
//! should never block normal tool use.
//!
//! Compliance here means the tool depends on the shared
//! `agent-tools-updater` crate. `tdd-ratchet` is not applicable because it is a
//! developer tool installed via `cargo install`.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["tdd-ratchet"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "auto-update",
    definition_summary: "Applicable CLI tools must include the shared updater dependency.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to installed CLI tools; not to the workspace itself or cargo-install-first tools like tdd-ratchet.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn auto_update() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let cargo_toml = tools_dir().join(tool).join("Cargo.toml");
            let cargo_toml_contents = std::fs::read_to_string(&cargo_toml)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", cargo_toml.display()));

            if !cargo_toml_contents.contains("agent-tools-updater") {
                failures.push(format!("{tool}: Cargo.toml missing agent-tools-updater"));
            }
        }

        if !failures.is_empty() {
            panic!("auto-update non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
