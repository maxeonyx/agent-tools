//! # Error Message Quality
//!
//! User-facing errors must be written for someone with no context.
//! Start with business context, state facts and values, preserve the causal
//! chain, and suggest a next action. Be verbose — errors are not the place
//! for brevity.
//!
//! Reference: `error-handling` skill.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
For each error path in the tool:
1. Trigger the error (bad input, missing file, invalid state)
2. Read the message
3. Can you fix the problem from the message alone, without reading source?
4. Does it start with business context (what the tool was trying to do)?
5. Does it state actual values (what was found vs what was expected)?
6. Does it suggest a next action?
7. Is the error chain preserved (context at each layer)?
8. Is it verbose enough for someone with no context?

Reference the `error-handling` skill for the full standard.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "error-messages",
    definition_summary:
        "Each tool must have a current review attestation covering error-message quality.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to tool user interfaces; the workspace is not an end-user CLI surface.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn error_messages() {
        let failures = concerns::review_attestation_failures("error-messages", NOT_APPLICABLE);

        if !failures.is_empty() {
            panic!("error-messages non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
