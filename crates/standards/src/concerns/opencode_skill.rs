//! # OpenCode Skill Published
//!
//! Each tool should be usable by agents via its OpenCode skill.
//!
//! These tools are built for agents. A skill gives usage patterns, gotchas, and
//! examples at the moment an agent needs them. Without a skill, agents have to
//! infer everything from `--help` or ask the user.
//!
//! The mechanical floor here is that `docs/SKILL.md` exists in the tool repo.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{TOOLS, tools_dir};

    #[test]
    fn opencode_skill() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let skill = tools_dir().join(tool).join("docs/SKILL.md");
            if !skill.exists() {
                failures.push(format!("{tool}: docs/SKILL.md missing"));
            }
        }

        if !failures.is_empty() {
            panic!("opencode-skill non-compliant:\n  {}", failures.join("\n  "));
        }
    }
}
