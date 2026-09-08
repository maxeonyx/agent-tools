//! # CLAUDE.md Aliases AGENTS.md
//!
//! Agent instructions in this ecosystem live in `AGENTS.md`. Anthropic's tools
//! do not read that name — they read `CLAUDE.md` — so instructions a repository
//! has written down are invisible to Claude Code until a `CLAUDE.md` points at
//! them. Max's user-level rule states it directly: whenever a repository has an
//! `AGENTS.md`, it also gets a `CLAUDE.md` containing `@AGENTS.md`.
//!
//! Compliance means every directory holding an `AGENTS.md` also holds a
//! `CLAUDE.md` with `@AGENTS.md` on a line of its own. That is Claude Code's
//! import syntax, resolved relative to the file it appears in, so the same
//! one-line body works at any depth. One line and no prose is the point: the
//! alias cannot drift from what it aliases.
//!
//! The rule follows the instruction files, not the repository roots. A nested
//! `AGENTS.md` exists because work in that directory needs different
//! instructions, and nested instruction files are discovered by name in the
//! directory being worked in — so a nested `AGENTS.md` without a sibling
//! `CLAUDE.md` is exactly as invisible as a root one.
//!
//! The fix is mechanical. Run, from the directory holding `AGENTS.md`:
//!
//! ```text
//! printf '@AGENTS.md\n' > CLAUDE.md
//! ```

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "claude-md-alias",
    definition_summary:
        "Every directory holding an AGENTS.md must hold a CLAUDE.md containing the line @AGENTS.md.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: true,
    applicability_note:
        "Applies to every tool repo, every checked library, and the workspace's own tree, because the workspace's AGENTS.md is read by the same agents as tool instructions. Excludes fixture directories, whose AGENTS.md files are deliberately malformed test input.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use std::path::{Path, PathBuf};

    /// Directory names never worth walking. `fixtures` holds deliberately
    /// malformed test input, including this concern's own fail cases.
    const SKIP_DIRS: &[&str] = &[".git", ".devenv", "target", "node_modules", "fixtures"];

    #[test]
    fn claude_md_alias() {
        let mut failures = Vec::new();

        let workspace = crate::workspace_root();
        // Tools and libraries are separate repositories, checked in their own right.
        let nested_repos = [workspace.join("tools"), workspace.join("libraries")];
        failures.extend(labelled("workspace", &workspace, &nested_repos));

        for tool in crate::checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            failures.extend(labelled(tool, &crate::tools_dir().join(tool), &[]));
        }

        for library in crate::checked_libraries() {
            failures.extend(labelled(
                library,
                &crate::libraries_dir().join(library),
                &[],
            ));
        }

        if !failures.is_empty() {
            panic!(
                "claude-md-alias non-compliant:\n  {}\n\nFix with, from the directory holding AGENTS.md:\n  printf '@AGENTS.md\\n' > CLAUDE.md",
                failures.join("\n  ")
            );
        }
    }

    #[test]
    fn fixture_aliased_instructions_are_accepted() {
        assert!(
            missing_aliases(&fixture("pass"), &[]).is_empty(),
            "the canonical fixture should be accepted"
        );
    }

    #[test]
    fn fixture_missing_claude_md_is_rejected() {
        assert_eq!(
            missing_aliases(&fixture("fail-missing-claude-md"), &[]),
            vec![".: CLAUDE.md missing".to_string()]
        );
    }

    #[test]
    fn fixture_nested_instructions_need_their_own_alias() {
        assert_eq!(
            missing_aliases(&fixture("fail-nested-missing-claude-md"), &[]),
            vec!["docs/design: CLAUDE.md missing".to_string()]
        );
    }

    #[test]
    fn fixture_claude_md_without_the_import_is_rejected() {
        assert_eq!(
            missing_aliases(&fixture("fail-unaliased-claude-md"), &[]),
            vec![".: CLAUDE.md does not import @AGENTS.md".to_string()]
        );
    }

    // ---- the checker -----------------------------------------------------

    /// Why each `AGENTS.md` directory under `root` has no working alias,
    /// reported as `<directory relative to root>: <reason>`.
    fn missing_aliases(root: &Path, excluded: &[PathBuf]) -> Vec<String> {
        let mut instruction_dirs = Vec::new();
        collect_instruction_dirs(root, excluded, &mut instruction_dirs);

        let mut failures = Vec::new();
        for dir in instruction_dirs {
            let relative = dir.strip_prefix(root).unwrap_or(&dir).display().to_string();
            let shown = if relative.is_empty() {
                ".".to_string()
            } else {
                relative
            };

            let alias = dir.join("CLAUDE.md");
            if !alias.is_file() {
                failures.push(format!("{shown}: CLAUDE.md missing"));
                continue;
            }

            let content = std::fs::read_to_string(&alias)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", alias.display()));
            if !content.lines().any(|line| line.trim() == "@AGENTS.md") {
                failures.push(format!("{shown}: CLAUDE.md does not import @AGENTS.md"));
            }
        }

        failures.sort();
        failures
    }

    fn collect_instruction_dirs(dir: &Path, excluded: &[PathBuf], out: &mut Vec<PathBuf>) {
        if dir.join("AGENTS.md").is_file() {
            out.push(dir.to_path_buf());
        }

        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if skip_dir_name(&name) || excluded.contains(&path) {
                continue;
            }
            collect_instruction_dirs(&path, excluded, out);
        }
    }

    fn skip_dir_name(name: &str) -> bool {
        // `*.ignore*` names are globally gitignored scratch space, including
        // whole clones of other repositories.
        SKIP_DIRS.contains(&name) || name.starts_with(".devenv.") || name.contains(".ignore")
    }

    fn labelled(repo: &str, root: &Path, excluded: &[PathBuf]) -> Vec<String> {
        missing_aliases(root, excluded)
            .into_iter()
            .map(|failure| format!("{repo}: {failure}"))
            .collect()
    }

    fn fixture(case: &str) -> PathBuf {
        crate::workspace_root()
            .join("crates/standards/src/concerns/claude_md_alias/fixtures")
            .join(case)
    }
}
