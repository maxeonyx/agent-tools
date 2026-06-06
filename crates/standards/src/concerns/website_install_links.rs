//! # Website Install Links
//!
//! Public install commands should resolve to downloadable assets.
//!
//! Pages can deploy successfully while still advertising a binary path that was
//! never produced. Compliance means every documented `https://*.maxeonyx.com`
//! release URL in the umbrella site and tool docs returns a successful response.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "website-install-links",
    definition_summary: "Documented public install links on published websites must resolve successfully.",
    review_instructions: REVIEW_INSTRUCTIONS,
    review_file_name: None,
    applies_to_workspace: false,
    applicability_note: "Applies to published tool-install links gathered from site packages, not to generic workspace docs alone.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, workspace_root, TOOLS};
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    #[test]
    fn website_install_links() {
        let mut urls = BTreeSet::new();
        let mut files = vec![workspace_root().join("docs/index.html")];

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            files.extend([
                tool_dir.join("README.md"),
                tool_dir.join("docs/index.html"),
                tool_dir.join("docs/SKILL.md"),
            ]);
        }

        for file in files {
            collect_release_urls(&file, &mut urls);
        }

        let mut failures = Vec::new();
        for url in urls {
            let output = std::process::Command::new("curl")
                .args(["-fsSIL", "--max-time", "10", &url])
                .output()
                .unwrap_or_else(|error| panic!("failed to run curl for {url}: {error}"));

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                failures.push(format!("{url}: {}", stderr.trim()));
            }
        }

        if !failures.is_empty() {
            panic!(
                "website-install-links non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn collect_release_urls(file: &PathBuf, urls: &mut BTreeSet<String>) {
        let Ok(content) = std::fs::read_to_string(file) else {
            return;
        };

        for token in content.split(|character: char| {
            character.is_whitespace() || character == '<' || character == '"' || character == '`'
        }) {
            let url = token
                .trim_end_matches(|character: char| {
                    matches!(character, '\'' | ')' | ';' | ',' | '\\')
                })
                .trim_end_matches("</span>");
            if url.starts_with("https://")
                && url.contains(".maxeonyx.com/releases/")
                && !url.contains("${")
            {
                urls.insert(url.to_string());
            }
        }
    }
}
