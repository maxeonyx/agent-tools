//! # Devenv Check Entrypoint
//!
//! Every repository should have a reproducible Nix/devenv shell with Rust,
//! linker tooling, and a `devenv test` entrypoint.
//!
//! Local machines and agents should not depend on ad hoc system packages like
//! `cc`. Compliance means the umbrella workspace and each standalone tool repo
//! have `devenv.nix`, `devenv.yaml`, `devenv.lock`, `.envrc`, ignored generated
//! state, and an `enterTest` block that runs the repo's combined checks.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};
    use std::path::Path;

    #[test]
    fn devenv_check() {
        let mut failures = Vec::new();

        check_repo(&workspace_root(), true, "workspace", &mut failures);

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            check_repo(&tools_dir().join(tool), false, tool, &mut failures);
        }

        if !failures.is_empty() {
            panic!("devenv-check non-compliant:\n  {}", failures.join("\n  "));
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
}
