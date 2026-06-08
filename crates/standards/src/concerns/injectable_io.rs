//! # Injectable IO for In-Process Integration Tests
//!
//! Nontrivial tools with multiple IO boundaries should push side effects to the
//! edges and support fast in-process tests by injecting boundary
//! implementations at construction time.
//!
//! IO boundaries (filesystem, network, subprocess) go through traits. Tests
//! provide in-memory implementations. Full application logic runs in-process at
//! unit-test speed.
//!
//! References: `programming` skill (side effects at the edges), `tests` skill
//! (Tier 2: fast in-process integration tests).

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the tool's architecture for injectable IO and fast in-process tests.

Required review method:
1. Identify every meaningful IO boundary: filesystem, environment, stdin/stdout,
   network, subprocesses, clocks, randomness, and platform state.
2. Trace one successful workflow and one failure workflow to see where those
   boundaries enter the core logic.
3. Read tests to confirm whether core behavior can run in-process with fakes or
   in-memory implementations, independent of real IO.
4. Produce findings by boundary. If there are no findings, name the boundaries
   and tests you inspected.

Check for:
1. Side effects are pushed to the edges and passed into application logic through
   traits, structs, functions, or equivalent explicit seams.
2. Core behavior is testable without real files, subprocesses, network, or global
   process state.
3. Tests cover success and failure behavior through injected boundaries, not only
   slow subprocess tests.
4. Boundary abstractions are small and domain-shaped, not broad mocks of entire
   libraries.
5. The CLI layer remains thin: parse input, build dependencies, call core logic,
   render output.

Reference the `programming` skill and `tests` skill Tier 2 guidance.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "injectable-io",
    definition_summary: "Applicable tools must have a current review attestation for injectable I/O design.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to every tool; even small tools need explicit I/O seams or a review finding explaining why the current shape is insufficient.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn injectable_io() {
        let failures = concerns::review_attestation_failures("injectable-io", NOT_APPLICABLE);

        if !failures.is_empty() {
            panic!("injectable-io non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
