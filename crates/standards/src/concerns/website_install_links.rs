//! # Website Install Links
//!
//! Public install commands should resolve to downloadable assets.
//!
//! Pages can deploy successfully while still advertising a binary path that was
//! never produced. Compliance means every documented `https://*.maxeonyx.com`
//! release URL in the umbrella site and tool docs returns a successful response.
//! When `STANDARDS_DOWNLOAD_INSTALL_BINARIES=1` is set, advertised binaries are
//! also downloaded and executed with `--version --json`.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "website-install-links",
    definition_summary: "Documented public install links on published websites must resolve successfully.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to published tool-install links gathered from site packages, not to generic workspace docs alone.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
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
            let output = evidence::context().command(
                EvidenceKey::new("install-url-head", &url),
                "curl",
                &["-fsSIL", "--max-time", "10", &url],
                &workspace_root(),
            );

            if !output.status_success {
                failures.push(format!("{url}: {}", output.stderr.trim()));
                continue;
            }

            if std::env::var_os("STANDARDS_DOWNLOAD_INSTALL_BINARIES").is_some() {
                verify_downloaded_binary(&url, &mut failures);
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

    fn verify_downloaded_binary(url: &str, failures: &mut Vec<String>) {
        let binary = url
            .rsplit('/')
            .next()
            .unwrap_or("downloaded-tool")
            .trim_end_matches("</span>");
        let path = std::env::temp_dir().join(format!("standards-{binary}"));
        let path_string = path.to_string_lossy().to_string();
        let download = evidence::context().command(
            EvidenceKey::new("install-url-download", url),
            "curl",
            &["-fsSL", "--max-time", "30", "-o", &path_string, url],
            &workspace_root(),
        );
        if !download.status_success {
            failures.push(format!(
                "{url}: download failed: {}",
                download.stderr.trim()
            ));
            return;
        }

        let chmod = evidence::context().command(
            EvidenceKey::new("install-url-chmod", &path_string),
            "chmod",
            &["+x", &path_string],
            &workspace_root(),
        );
        if !chmod.status_success {
            failures.push(format!("{url}: chmod failed: {}", chmod.stderr.trim()));
            return;
        }

        let version = evidence::context().command(
            EvidenceKey::new("downloaded-binary-version-json", url),
            &path_string,
            &["--version", "--json"],
            &workspace_root(),
        );
        if !version.status_success {
            failures.push(format!(
                "{url}: downloaded binary --version --json failed: {}",
                version.stderr.trim()
            ));
            return;
        }
        if let Err(error) = serde_json::from_str::<serde_json::Value>(&version.stdout) {
            failures.push(format!(
                "{url}: downloaded binary --version --json invalid JSON: {error}"
            ));
        }
    }
}
