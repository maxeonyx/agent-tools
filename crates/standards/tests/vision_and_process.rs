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

use standards::{tools_dir, TOOLS};

const NOT_APPLICABLE: &[&str] = &[];

#[test]
fn vision_and_process() {
    let mut failures = Vec::new();

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
