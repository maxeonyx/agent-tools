//! # Disposable Experiments
//!
//! Repositories may use an `experiments/` directory for focused design spikes.
//! An experiment is evidence, not architecture: it should make one thesis,
//! flow, protocol shape, or code path excellent in isolation, explicitly state
//! what it does not support, and produce an outcome saying what to integrate and
//! what not to integrate.
//!
//! Core code must not depend on experiment internals. Integration should
//! reimplement the proven behavior deliberately, using the experiment as
//! reference material.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "experiments",
    definition_summary: "Repos with experiments/ must keep experiments disposable, focused, and out of core imports.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note: "Applies to any repo in the workspace that has an experiments/ directory.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn experiments() {
        let mut failures = Vec::new();

        failures.extend(experiment_failures_for_repo("workspace", &workspace_root()));

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(experiment_failures_for_repo(tool, &tools_dir().join(tool)));
        }

        if !failures.is_empty() {
            panic!("experiments non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    #[test]
    fn compliant_experiments_fixture_passes() {
        let repo = temp_repo("experiments-pass");
        write(
            &repo.join("experiments/README.md"),
            "\
# Experiments

Disposable experiments each name one focused thesis, state what they do not support,
act as reference material only, and produce an outcome document covering what to
integrate and what not to integrate.
",
        );
        write(&repo.join("src/lib.rs"), "pub fn run() {}\n");

        assert_eq!(
            experiment_failures_for_repo("fixture", &repo),
            Vec::<String>::new()
        );
        std::fs::remove_dir_all(repo).unwrap();
    }

    #[test]
    fn missing_guidance_and_core_references_fail() {
        let repo = temp_repo("experiments-fail");
        write(&repo.join("experiments/README.md"), "# Experiments\n");
        write(
            &repo.join("src/lib.rs"),
            "pub const BAD: &str = \"experiments/prototype\";\n",
        );

        let failures = experiment_failures_for_repo("fixture", &repo);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("missing disposable guidance")),
            "{failures:?}"
        );
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("references experiments/ from core source")),
            "{failures:?}"
        );
        std::fs::remove_dir_all(repo).unwrap();
    }

    fn experiment_failures_for_repo(repo: &str, repo_path: &Path) -> Vec<String> {
        let experiments_dir = repo_path.join("experiments");
        if !experiments_dir.is_dir() {
            return Vec::new();
        }

        let mut failures = Vec::new();
        failures.extend(readme_failures(repo, &experiments_dir.join("README.md")));
        failures.extend(core_reference_failures(repo, repo_path));
        failures
    }

    fn readme_failures(repo: &str, readme_path: &Path) -> Vec<String> {
        let mut failures = Vec::new();
        let Ok(readme) = std::fs::read_to_string(readme_path) else {
            return vec![format!("{repo}: experiments/README.md missing")];
        };
        let normalized = readme.to_lowercase();

        for (label, alternatives) in [
            ("disposable", &["disposable"][..]),
            ("focused thesis", &["thesis", "one aspect", "focused"][..]),
            (
                "explicit non-support",
                &["does not support", "not support", "non-goal"][..],
            ),
            (
                "reference-only",
                &["reference", "not port", "do not port"][..],
            ),
            (
                "outcome document",
                &["outcome", "what to integrate", "what not to integrate"][..],
            ),
        ] {
            if !alternatives
                .iter()
                .any(|alternative| normalized.contains(alternative))
            {
                failures.push(format!(
                    "{repo}: experiments/README.md missing {label} guidance"
                ));
            }
        }

        failures
    }

    fn core_reference_failures(repo: &str, repo_path: &Path) -> Vec<String> {
        let src_dir = repo_path.join("src");
        if !src_dir.is_dir() {
            return Vec::new();
        }

        let mut failures = Vec::new();
        for path in rust_files_under(&src_dir) {
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            if content.contains("experiments") {
                failures.push(format!(
                    "{repo}: {} references experiments/ from core source",
                    path.strip_prefix(repo_path).unwrap_or(&path).display()
                ));
            }
        }
        failures
    }

    fn rust_files_under(root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut stack = vec![root.to_path_buf()];

        while let Some(path) = stack.pop() {
            let entries = std::fs::read_dir(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            for entry in entries {
                let entry = entry.unwrap_or_else(|error| {
                    panic!("failed to read entry under {}: {error}", path.display())
                });
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }

        files
    }

    fn temp_repo(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let repo = std::env::temp_dir().join(format!("standards-{name}-{unique}"));
        std::fs::create_dir_all(&repo).unwrap();
        repo
    }

    fn write(path: &Path, content: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }
}
