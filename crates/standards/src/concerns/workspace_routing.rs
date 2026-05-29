//! # Develop from workspace
//!
//! All development happens from the `agent-tools` workspace, not from
//! individual tool clones.
//!
//! This is the foundation concern. When agents clone individual repos, they lose
//! visibility into sibling tools and shared standards. They make decisions that
//! diverge from the ecosystem. The workspace gives them all tools at once and
//! shared enforcement.
//!
//! Nothing else in this standards system works if agents develop outside the
//! workspace. To comply, a tool's `AGENTS.md` should clearly and prominently
//! direct agents to develop from `maxeonyx/agent-tools`.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{TOOLS, tools_dir};

    #[test]
    fn workspace_routing() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let path = tools_dir().join(tool).join("AGENTS.md");
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

            let has_workspace_routing = content.contains("agent-tools")
                && (content.contains("workspace") || content.contains("Workspace"));

            if !has_workspace_routing {
                failures.push(format!(
                    "{tool}: AGENTS.md missing workspace-routing directive"
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "workspace-routing non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
