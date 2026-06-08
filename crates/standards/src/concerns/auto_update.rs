//! # Auto-update Mechanism (Deprecated)
//!
//! Users install these tools once and forget. Without auto-update, they run
//! stale versions indefinitely.
//!
//! This dependency-only concern is deprecated. An unused dependency does not
//! prove users receive updates, so enforcement is folded into
//! `auto-update-integration`, which checks dependency wiring, call sites, tests,
//! and the eventual forced-update proof.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["trunc", "tb", "dotsync", "tdd-ratchet", "oc"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "auto-update",
    definition_summary: "Deprecated; auto-update enforcement is folded into auto-update-integration.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "No direct applicability remains; keep this concern as a historical registry alias until the registry is cleaned up.",
};

#[cfg(test)]
mod tests {
    #[test]
    fn auto_update() {
        // Dependency-only enforcement intentionally lives in
        // auto_update_integration now, alongside the stronger behavior checks.
    }
}
