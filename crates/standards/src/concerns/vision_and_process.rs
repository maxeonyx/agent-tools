//! # Vision and Process Docs
//!
//! Each tool should have `VISION.md` and a proper `AGENTS.md`.
//!
//! A tool without a vision statement drifts. Agents and humans make feature
//! decisions without knowing what the tool is for, who it serves, and what it
//! should avoid. `AGENTS.md` is the process counterpart: how to develop the
//! tool, how it is organized, and what constraints matter.
//!
//! The mechanical floor here is that both files exist.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "vision-and-process",
    definition_summary: "Repos should have clear vision and development-process documents.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: None,
    applies_to_workspace: true,
    applicability_note: "Applies to tool repos and to the workspace itself because both need explicit product and process guidance.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};

    #[test]
    fn vision_and_process() {
        let mut failures = Vec::new();

        let workspace = workspace_root();
        if !workspace.join("VISION.md").exists() {
            failures.push("workspace: VISION.md missing".to_string());
        }
        if !workspace.join("AGENTS.md").exists() {
            failures.push("workspace: AGENTS.md missing".to_string());
        }

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_path = tools_dir().join(tool);
            if !tool_path.join("VISION.md").exists() {
                failures.push(format!("{tool}: VISION.md missing"));
            }
            if !tool_path.join("AGENTS.md").exists() {
                failures.push(format!("{tool}: AGENTS.md missing"));
            }
        }

        if !failures.is_empty() {
            panic!(
                "vision-and-process non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
