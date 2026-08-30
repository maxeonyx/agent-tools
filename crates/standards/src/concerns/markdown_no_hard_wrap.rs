//! # Markdown Is Not Hard Wrapped
//!
//! Markdown in this ecosystem is never line-wrapped. One paragraph is one line.
//!
//! Hard wrapping costs us twice. It makes diffs noisy — editing one word
//! reflows the whole paragraph, so review has to read a block of changed lines
//! to find a one-word change. And it makes editing painful, because every
//! insertion leaves the surrounding lines ragged until something re-wraps them.
//! Every viewer we care about soft-wraps for us, so the wrapping buys nothing.
//! Max's design note `markdown-nowrap-lead-by-example.md` also asks the harness
//! to lead by example, since it injects markdown context and skills of its own.
//!
//! Compliance means no line is a column-wrap continuation of the line above it.
//! The checker looks for the fingerprint a wrapping tool leaves behind: inside
//! one paragraph, every line fills to a consistent width and each break happens
//! exactly where the next word stopped fitting. Deliberate one-line-per-thought
//! writing does not match that fingerprint, and neither does a bare list of
//! URLs, so both stay legal. Fenced and indented code, tables, headings, list
//! markers and YAML frontmatter are never treated as wrapped prose.
//!
//! `docs/source-notes/` is excluded: those files are verbatim copies of an
//! upstream design gist, so their formatting is not ours to police locally.
//! Gitignored `*.ignore.*` scratch files are excluded for the same reason —
//! they are not checked in.
//!
//! The fix is mechanical. Run, from the repo holding the file:
//!
//! ```text
//! npx prettier@3 --write --prose-wrap never --embedded-language-formatting off <file>
//! ```

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "markdown-no-hard-wrap",
    definition_summary:
        "Checked-in markdown must not be hard wrapped; a paragraph is one line, not a filled column.",
    applies_to_workspace: true,
    review_instructions: REVIEW_INSTRUCTIONS,
    applicability_note:
        "Applies to every tool repo and to the workspace's own markdown, because the workspace's AGENTS.md, README.md and docs are read and edited the same way tool docs are. Excludes docs/source-notes/, which is verbatim upstream content.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use std::path::{Path, PathBuf};

    /// Directory names never worth walking. `fixtures` holds deliberately
    /// malformed test input, including this concern's own fail cases.
    const SKIP_DIRS: &[&str] = &[".git", ".devenv", "target", "node_modules", "fixtures"];

    /// The widest line a wrapping tool plausibly fills to. Lines longer than
    /// this are someone writing one thought per line, not a filled column.
    const MAX_FILL_WIDTH: usize = 100;
    /// The narrowest fill column worth believing in. Below this, short adjacent
    /// lines are far more likely to be deliberate (list-like notes).
    const MIN_FILL_WIDTH: usize = 50;
    /// A wrapped prose line has several words. A bare URL list has one.
    const MIN_MEDIAN_WORDS: usize = 4;

    #[test]
    fn markdown_no_hard_wrap() {
        let mut failures = Vec::new();

        for (label, root, files) in policed_markdown() {
            for file in files {
                let content = std::fs::read_to_string(&file)
                    .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()));
                let wrapped = hard_wrapped_lines(&content);
                if wrapped.is_empty() {
                    continue;
                }
                let relative = file.strip_prefix(&root).unwrap_or(&file).display();
                let shown: Vec<String> = wrapped.iter().take(3).map(|n| n.to_string()).collect();
                let more = if wrapped.len() > shown.len() {
                    format!(" (+{} more)", wrapped.len() - shown.len())
                } else {
                    String::new()
                };
                failures.push(format!(
                    "{label}: {relative} is hard wrapped at line(s) {}{more}",
                    shown.join(", ")
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "markdown-no-hard-wrap non-compliant:\n  {}\n\nFix with, from the repo holding the file:\n  npx prettier@3 --write --prose-wrap never --embedded-language-formatting off <file>",
                failures.join("\n  ")
            );
        }
    }

    #[test]
    fn fixture_unwrapped_markdown_is_accepted() {
        for file in fixture_files("pass") {
            let content = std::fs::read_to_string(&file).unwrap();
            assert!(
                hard_wrapped_lines(&content).is_empty(),
                "{} should be accepted, but was flagged at {:?}",
                file.display(),
                hard_wrapped_lines(&content)
            );
        }
    }

    #[test]
    fn fixture_wrapped_paragraph_is_rejected() {
        let file = fixture_files("fail-wrapped-paragraph")
            .into_iter()
            .next()
            .expect("fixture missing");
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(
            hard_wrapped_lines(&content),
            vec![4, 5],
            "the two continuation lines of the wrapped paragraph should be flagged"
        );
    }

    #[test]
    fn fixture_wrapped_list_item_is_rejected() {
        let file = fixture_files("fail-wrapped-list-item")
            .into_iter()
            .next()
            .expect("fixture missing");
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(
            hard_wrapped_lines(&content),
            vec![4, 6],
            "the continuation line of each wrapped list item should be flagged"
        );
    }

    #[test]
    fn fixture_wrapped_blockquote_is_rejected() {
        let file = fixture_files("fail-wrapped-blockquote")
            .into_iter()
            .next()
            .expect("fixture missing");
        let content = std::fs::read_to_string(&file).unwrap();
        assert_eq!(hard_wrapped_lines(&content), vec![4]);
    }

    // ---- the checker -----------------------------------------------------

    /// Line numbers (1-based) that are column-wrap continuations of the line above.
    fn hard_wrapped_lines(text: &str) -> Vec<usize> {
        let (body, offset) = strip_frontmatter(text);

        let mut wrapped = Vec::new();
        let mut fence: Option<(char, usize)> = None;
        // Consecutive prose lines, as (line_number, quote_depth, body).
        let mut group: Vec<(usize, usize, String)> = Vec::new();

        for (index, raw) in body.lines().enumerate() {
            let number = index + 1 + offset;

            if let Some((marker, width)) = fence {
                if closes_fence(raw, marker, width) {
                    fence = None;
                }
                flush(&mut group, &mut wrapped);
                continue;
            }
            if let Some(open) = opening_fence(raw) {
                fence = Some(open);
                flush(&mut group, &mut wrapped);
                continue;
            }
            if raw.trim().is_empty() {
                flush(&mut group, &mut wrapped);
                continue;
            }

            let (depth, content) = strip_quote(raw);
            // An indented run that opens a paragraph is code, not prose.
            if group.is_empty() && starts_indented(content) {
                flush(&mut group, &mut wrapped);
                continue;
            }
            group.push((number, depth, content.to_string()));
        }
        flush(&mut group, &mut wrapped);

        wrapped.sort_unstable();
        wrapped
    }

    /// Judge one paragraph and record its wrapped continuation lines.
    fn flush(group: &mut Vec<(usize, usize, String)>, wrapped: &mut Vec<usize>) {
        let lines = std::mem::take(group);
        if lines.len() < 2 {
            return;
        }

        // Candidate breaks: same quote depth, and the lower line does not open
        // a block of its own.
        let mut pairs = Vec::new();
        for window in lines.windows(2) {
            let (_, prev_depth, prev) = &window[0];
            let (number, depth, current) = &window[1];
            if prev_depth != depth {
                continue;
            }
            if opens_block(current) || ends_with_hard_break(prev) {
                continue;
            }
            pairs.push((*number, prev.trim_end().to_string(), current.to_string()));
        }
        if pairs.is_empty() {
            return;
        }

        if !is_column_wrapped(&pairs) {
            return;
        }
        wrapped.extend(pairs.iter().map(|(number, _, _)| *number));
    }

    /// Does this run of breaks carry a wrapping tool's fingerprint?
    fn is_column_wrapped(pairs: &[(usize, String, String)]) -> bool {
        let mut counts: Vec<usize> = Vec::new();
        for (_, prev, current) in pairs {
            counts.push(prev.split_whitespace().count());
            counts.push(current.split_whitespace().count());
        }
        counts.sort_unstable();
        if counts[counts.len() / 2] < MIN_MEDIAN_WORDS {
            return false; // a list of URLs or similar, not prose
        }

        let width = pairs
            .iter()
            .map(|(_, prev, _)| prev.chars().count())
            .max()
            .unwrap_or(0);
        if !(MIN_FILL_WIDTH..=MAX_FILL_WIDTH).contains(&width) {
            return false;
        }

        // Every break must be explained by the fill column: the next word could
        // not have fitted. One deliberately short line disqualifies the run.
        pairs.iter().all(|(_, prev, current)| {
            let next_word = current
                .split_whitespace()
                .next()
                .unwrap_or("")
                .chars()
                .count();
            prev.chars().count() + 1 + next_word > width
        })
    }

    // ---- markdown shape --------------------------------------------------

    /// Skip YAML frontmatter, returning the body and how many lines were skipped.
    fn strip_frontmatter(text: &str) -> (&str, usize) {
        let Some(rest) = text.strip_prefix("---\n") else {
            return (text, 0);
        };
        let Some(end) = rest.find("\n---\n") else {
            return (text, 0);
        };
        let consumed = &text[..end + 5 + 4];
        (&rest[end + 5..], consumed.lines().count())
    }

    fn opening_fence(line: &str) -> Option<(char, usize)> {
        let trimmed = line.trim_start();
        if line.len() - trimmed.len() > 3 {
            return None;
        }
        for marker in ['`', '~'] {
            let width = trimmed.chars().take_while(|c| *c == marker).count();
            if width >= 3 {
                return Some((marker, width));
            }
        }
        None
    }

    fn closes_fence(line: &str, marker: char, width: usize) -> bool {
        let trimmed = line.trim_start();
        let run = trimmed.chars().take_while(|c| *c == marker).count();
        run >= width && trimmed[run..].trim().is_empty()
    }

    /// Strip leading `>` markers, returning quote depth and the remaining text.
    fn strip_quote(line: &str) -> (usize, &str) {
        let mut rest = line;
        let mut depth = 0;
        loop {
            let trimmed = rest.trim_start();
            match trimmed.strip_prefix('>') {
                Some(after) => {
                    depth += 1;
                    rest = after;
                }
                None => return (depth, if depth == 0 { line } else { rest }),
            }
        }
    }

    fn starts_indented(line: &str) -> bool {
        line.starts_with("    ") || line.starts_with('\t')
    }

    /// Would a reader see this line as opening a new block rather than
    /// continuing the previous one?
    fn opens_block(line: &str) -> bool {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            return true;
        }
        // Heading, table row, or fence.
        if trimmed.starts_with('#') || trimmed.starts_with('|') || opening_fence(line).is_some() {
            return true;
        }
        // Thematic break or setext underline.
        if is_repeated_punctuation(trimmed) {
            return true;
        }
        // Bullet: `- `, `* `, `+ `.
        let mut chars = trimmed.chars();
        if let (Some(first), Some(second)) = (chars.next(), chars.next()) {
            if matches!(first, '-' | '*' | '+') && second.is_whitespace() {
                return true;
            }
        }
        // Ordered marker, including hand-written sub-items like `8a.`.
        opens_ordered_item(trimmed)
    }

    /// `1.`, `2)`, and authored variants such as `8a.` — all read as list items
    /// even where CommonMark would treat them as lazy continuation, because
    /// unwrapping them would destroy numbering the author chose.
    fn opens_ordered_item(trimmed: &str) -> bool {
        let digits = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
        if digits == 0 {
            return false;
        }
        let mut rest = &trimmed[digits..];
        // At most one optional letter suffix.
        if rest.starts_with(|c: char| c.is_ascii_lowercase()) {
            rest = &rest[1..];
        }
        let Some(after) = rest.strip_prefix('.').or_else(|| rest.strip_prefix(')')) else {
            return false;
        };
        after.starts_with(char::is_whitespace)
    }

    fn is_repeated_punctuation(trimmed: &str) -> bool {
        for marker in ['-', '*', '_', '='] {
            let run = trimmed.chars().take_while(|c| *c == marker).count();
            if run >= 2 && trimmed[run..].trim().is_empty() {
                return true;
            }
        }
        false
    }

    /// Markdown's explicit line break: a trailing backslash or two spaces.
    fn ends_with_hard_break(line: &str) -> bool {
        line.ends_with("  ") || line.trim_end().ends_with('\\')
    }

    // ---- what gets policed ----------------------------------------------

    /// (label, root the paths are reported relative to, markdown files)
    fn policed_markdown() -> Vec<(String, PathBuf, Vec<PathBuf>)> {
        let workspace = crate::workspace_root();
        let mut targets = Vec::new();

        // The workspace's own markdown, excluding the tool submodules.
        let mut own = Vec::new();
        collect_markdown(&workspace, &workspace.join("tools"), &mut own);
        own.sort();
        targets.push(("workspace".to_string(), workspace.clone(), own));

        for tool in crate::checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let root = crate::tools_dir().join(tool);
            let mut files = Vec::new();
            collect_markdown(&root, Path::new(""), &mut files);
            files.sort();
            targets.push((tool.to_string(), root, files));
        }

        targets
    }

    fn collect_markdown(dir: &Path, excluded_dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if path.is_dir() {
                if skip_dir_name(&name) || path == excluded_dir {
                    continue;
                }
                // Verbatim upstream copies are not ours to reformat.
                if name == "source-notes" {
                    continue;
                }
                collect_markdown(&path, excluded_dir, out);
                continue;
            }

            if !name.ends_with(".md") || name.contains(".ignore") {
                continue;
            }
            out.push(path);
        }
    }

    fn skip_dir_name(name: &str) -> bool {
        SKIP_DIRS.contains(&name) || name.starts_with(".devenv.")
    }

    #[test]
    fn generated_devenv_profiles_are_skipped() {
        assert!(skip_dir_name(".devenv"));
        assert!(skip_dir_name(".devenv.generated-profile"));
    }

    fn fixture_files(case: &str) -> Vec<PathBuf> {
        let dir = crate::workspace_root()
            .join("crates/standards/src/concerns/markdown_no_hard_wrap/fixtures")
            .join(case);
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", dir.display()))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no fixtures in {}", dir.display());
        files
    }
}
