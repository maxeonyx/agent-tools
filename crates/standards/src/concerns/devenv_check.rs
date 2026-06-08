//! # Devenv Build Reproducibility
//!
//! Every repository should have a reproducible Nix/devenv shell that can build
//! the outputs developers and CI depend on.
//!
//! Local machines and agents should not depend on ad hoc system packages like
//! `cc`. Compliance keeps the structural devenv files and `enterTest` entrypoint
//! checks, and adds outcome evidence by running a build inside the repo's devenv
//! shell.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "devenv-check",
    definition_summary: "The workspace and each tool repo must provide a reproducible devenv shell that can build repo outputs.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to the workspace and to each tool repo because all development and CI setup depends on reproducible build environments.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn devenv_check() {
        let mut failures = Vec::new();

        check_repo(&workspace_root(), true, "workspace", &mut failures);
        check_devenv_build(
            &workspace_root(),
            "workspace",
            &["cargo", "check", "-p", "standards", "--tests"],
            &mut failures,
        );

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            check_repo(&tool_dir, false, tool, &mut failures);
            check_devenv_build(
                &tool_dir,
                tool,
                &["cargo", "build", "--locked", "--bins"],
                &mut failures,
            );
        }

        if !failures.is_empty() {
            panic!("devenv-check non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn check_devenv_build(
        path: &Path,
        name: &str,
        build_args: &[&str],
        failures: &mut Vec<String>,
    ) {
        if !path.join("devenv.nix").exists() {
            failures.push(format!(
                "{name}: cannot run devenv build evidence without devenv.nix"
            ));
            return;
        }

        let mut args = vec!["shell", "--"];
        args.extend_from_slice(build_args);

        let output = Command::new("devenv")
            .args(args)
            .current_dir(path)
            .output()
            .unwrap_or_else(|error| {
                panic!(
                    "failed to run devenv build evidence for {} at {}: {error}",
                    name,
                    path.display()
                )
            });

        if !output.status.success() {
            failures.push(format!(
                "{name}: devenv build evidence failed for `{}`\n{}",
                build_args.join(" "),
                command_output(&output)
            ));
        }
    }

    fn check_repo(path: &Path, is_workspace: bool, name: &str, failures: &mut Vec<String>) {
        for required in [
            "devenv.nix",
            "devenv.yaml",
            "devenv.lock",
            ".envrc",
            ".gitignore",
        ] {
            if !path.join(required).exists() {
                failures.push(format!("{name}: {required} missing"));
            }
        }

        let gitignore = read(path.join(".gitignore"));
        for ignored in [".devenv", "devenv.local.nix", ".direnv"] {
            if !gitignore.contains(ignored) {
                failures.push(format!("{name}: .gitignore missing {ignored}"));
            }
        }

        let envrc = read(path.join(".envrc"));
        if !envrc.contains("use devenv") {
            failures.push(format!("{name}: .envrc does not use devenv"));
        }
        if !envrc.contains("devenv direnvrc") {
            failures.push(format!("{name}: .envrc missing devenv direnvrc"));
        }

        let yaml = read(path.join("devenv.yaml"));
        if !yaml.contains("devenv.schema.json") || !yaml.contains("nixpkgs") {
            failures.push(format!(
                "{name}: devenv.yaml missing schema or nixpkgs input"
            ));
        }

        let nix = read(path.join("devenv.nix"));
        for package in [
            "pkgs.cargo",
            "pkgs.rustc",
            "pkgs.rustfmt",
            "pkgs.clippy",
            "pkgs.gcc",
        ] {
            if !nix.contains(package) {
                failures.push(format!("{name}: devenv.nix missing {package}"));
            }
        }
        if !nix.contains("enterTest") {
            failures.push(format!("{name}: devenv.nix missing enterTest"));
        }
        if is_workspace {
            for command in [
                "cargo fmt --check --all",
                "cargo check -p standards --tests",
                "cargo test -p standards",
            ] {
                if !nix.contains(command) {
                    failures.push(format!("{name}: enterTest missing `{command}`"));
                }
            }
        } else {
            for command in [
                "cargo fmt --check",
                "cargo clippy -- -D warnings",
                "cargo test",
            ] {
                if !nix.contains(command) {
                    failures.push(format!("{name}: enterTest missing `{command}`"));
                }
            }
        }
    }

    fn read(path: impl AsRef<Path>) -> String {
        std::fs::read_to_string(path).unwrap_or_default()
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
