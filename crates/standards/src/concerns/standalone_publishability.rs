//! # Standalone Publishability
//!
//! Tool repositories are standalone release repos. Their CI checks out the tool
//! repo, not this umbrella workspace, so release builds must not depend on
//! workspace-relative path crates that are missing in the standalone checkout.
//!
//! Compliance means each tool repo advertises a standalone CI build and can be
//! cloned and built from its own checkout. Workspace-relative path dependency
//! scanning remains useful diagnostic evidence, but the build outcome is the
//! policy.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "standalone-publishability",
    definition_summary: "Standalone tool repos must build from an isolated checkout and advertise standalone CI builds.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to standalone tool publishing, not to the umbrella workspace checkout.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[test]
    fn standalone_publishability() {
        let mut failures = Vec::new();
        let mut diagnostics = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml_path = tool_dir.join("Cargo.toml");
            let cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap_or_else(|error| {
                panic!("failed to read {}: {error}", cargo_toml_path.display())
            });

            for line in cargo_toml.lines() {
                if line.contains("path = \"../") || line.contains("path = \"../../") {
                    diagnostics.push(format!(
                        "{tool}: diagnostic: Cargo.toml has workspace-relative dependency: {}",
                        line.trim()
                    ));
                }
            }

            check_ci_build_evidence(tool, &tool_dir, &mut failures);
            check_isolated_checkout_build(tool, &tool_dir, &mut failures);
        }

        if !diagnostics.is_empty() {
            eprintln!(
                "standalone-publishability diagnostics:\n  {}",
                diagnostics.join("\n  ")
            );
        }

        if !failures.is_empty() {
            panic!(
                "standalone-publishability non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn check_ci_build_evidence(tool: &str, tool_dir: &Path, failures: &mut Vec<String>) {
        let workflow_dir = tool_dir.join(".github/workflows");
        let workflow_content = tree_contents(&workflow_dir, &["yml", "yaml"]);

        if workflow_content.trim().is_empty() {
            failures.push(format!("{tool}: no GitHub Actions workflow found"));
            return;
        }

        if !workflow_content.contains("actions/checkout@") {
            failures.push(format!(
                "{tool}: workflow missing actions/checkout standalone checkout evidence"
            ));
        }
        if !workflow_content.contains("cargo build --release") {
            failures.push(format!(
                "{tool}: workflow missing cargo build --release evidence"
            ));
        }
    }

    fn check_isolated_checkout_build(tool: &str, tool_dir: &Path, failures: &mut Vec<String>) {
        let clone_dir = temp_clone_dir(tool);
        if clone_dir.exists() {
            std::fs::remove_dir_all(&clone_dir).unwrap_or_else(|error| {
                panic!(
                    "failed to clean temp clone {}: {error}",
                    clone_dir.display()
                )
            });
        }

        let clone_output = Command::new("git")
            .args(["clone", "--quiet", "--no-hardlinks"])
            .arg(tool_dir)
            .arg(&clone_dir)
            .output()
            .unwrap_or_else(|error| panic!("failed to clone {tool} for standalone build: {error}"));

        if !clone_output.status.success() {
            failures.push(format!(
                "{tool}: failed to create isolated local clone\n{}",
                command_output(&clone_output)
            ));
            return;
        }

        let build_output = Command::new("cargo")
            .args(["build", "--locked", "--bins"])
            .current_dir(&clone_dir)
            .output()
            .unwrap_or_else(|error| {
                panic!("failed to run isolated cargo build for {tool}: {error}")
            });

        if !build_output.status.success() {
            failures.push(format!(
                "{tool}: isolated checkout cargo build --locked --bins failed\n{}",
                command_output(&build_output)
            ));
        }

        std::fs::remove_dir_all(&clone_dir).unwrap_or_else(|error| {
            panic!(
                "failed to remove temp clone {}: {error}",
                clone_dir.display()
            )
        });
    }

    fn temp_clone_dir(tool: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "agent-tools-standalone-{tool}-{}",
            std::process::id()
        ))
    }

    fn tree_contents(root: &Path, extensions: &[&str]) -> String {
        let mut content = String::new();
        collect_contents(root, extensions, &mut content);
        content
    }

    fn collect_contents(path: &Path, extensions: &[&str], content: &mut String) {
        let Ok(metadata) = std::fs::metadata(path) else {
            return;
        };

        if metadata.is_file() {
            let extension = path.extension().and_then(|extension| extension.to_str());
            if extension.is_some_and(|extension| extensions.contains(&extension)) {
                content.push_str(&std::fs::read_to_string(path).unwrap_or_default());
                content.push('\n');
            }
            return;
        }

        let entries = std::fs::read_dir(path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for entry in entries.filter_map(Result::ok) {
            collect_contents(&entry.path(), extensions, content);
        }
    }

    fn command_output(output: &std::process::Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut parts = Vec::new();
        if !stdout.trim().is_empty() {
            parts.push(format!("stdout:\n{}", stdout.trim()));
        }
        if !stderr.trim().is_empty() {
            parts.push(format!("stderr:\n{}", stderr.trim()));
        }
        if parts.is_empty() {
            format!("exit status: {}", output.status)
        } else {
            parts.join("\n")
        }
    }
}
