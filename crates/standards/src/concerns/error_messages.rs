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
Review user-facing errors by triggering them, not by inspecting strings alone.

Required review method:
1. Trigger representative failures: bad input, missing files, permission or
   environment problems where practical, invalid state, and failed subprocess or
   network boundaries if the tool has them.
2. Capture the exact command, exit status, stderr/stdout split, and message.
3. Produce findings by error path. If there are no findings, list the failure
   classes you exercised.

Check each error for:
1. Business context first: what the tool was trying to do.
2. Concrete facts and values: what path, command, value, status, or response was
   observed, and what was expected.
3. A useful next action that does not require reading source.
4. Preserved causal chain with context at each layer rather than a bare low-level
   error.
5. Correct output channel and nonzero exit behavior for failures.
6. Enough detail for a user with no prior context; errors should be clear before
   they are brief.

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
