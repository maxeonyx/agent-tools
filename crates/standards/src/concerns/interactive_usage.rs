//! # Interactive and Non-Interactive CLI Usage
//!
//! CLI tools must treat interactive terminal usage and non-interactive usage as
//! distinct user experiences. Interactive usage with a tty should support color
//! and may sometimes prompt for missing values or open an editor for multiline
//! text. Non-interactive usage must validate all requirements up front, fail
//! before partial work, and explain what was wrong and what should be done.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the CLI's behavior separately in interactive tty usage and non-interactive usage.

Required review method:
1. Exercise representative successful and failing commands through a tty where
   practical. Include workflows that render status, diffs, confirmations, or
   multiline text if the tool has them.
2. Exercise representative successful and failing commands without a tty, for
   example with stdin/stdout/stderr redirected or through a subprocess test.
3. Capture the exact command, whether it had a tty, exit status, stdout/stderr
   split, and any prompt, editor, color, or validation behavior observed.
4. Produce findings by command or workflow. If there are no findings, list the
   tty and non-tty workflows exercised.

Check interactive tty usage for:
1. Output uses color where it helps a human scan status, diffs, warnings, or
   errors, while respecting existing color controls if the tool has them.
2. Prompts or editor launches are used only where they improve the workflow;
   they are not required for every missing value.
3. Prompts clearly say what value is needed and how the answer will be used.
4. Editor-based multiline input has a clear fallback or failure mode if the
   editor cannot be launched.

Check non-interactive usage for:
1. All required inputs and state preconditions are validated up front before
   partial work starts.
2. The tool exits nonzero on invalid input or invalid state.
3. The error message says what was wrong and what should be done next.
4. The tool never hangs waiting for input, silently opens an editor, or prompts
   where no user can answer.
5. Machine-usable output remains stable and is not polluted by tty-only color or
   prompts unless explicitly requested.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "interactive-usage",
    definition_summary: "Applicable CLI tools must have a current review attestation for correct interactive tty and non-interactive behavior.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to every end-user CLI tool; the workspace itself is not an end-user CLI surface.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn interactive_usage() {
        let failures = concerns::review_attestation_failures("interactive-usage", NOT_APPLICABLE);

        if !failures.is_empty() {
            panic!(
                "interactive-usage non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
