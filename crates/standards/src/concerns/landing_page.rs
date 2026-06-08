//! # Website with Standards
//!
//! Each tool should have a landing page with a consistent aesthetic and content.
//!
//! These tools are public. People and agents discover them via their websites.
//! A consistent design language signals that they are a cohesive suite, and the
//! site is where install instructions and ecosystem links stay current.
//!
//! The mechanical floor here is live reachability plus suite link graph
//! consistency: README/GitHub metadata point at the public tool site, the
//! umbrella site links to every tool site, and every tool site links back to the
//! umbrella family site.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "landing-page",
    definition_summary: "Each tool repo must ship a landing page package in docs/index.html.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note:
        "Applies to individual tool websites; the umbrella site has separate root packaging checks.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence::{self, EvidenceKey};
    use crate::{tools_dir, workspace_root, TOOLS};

    #[test]
    fn landing_page() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let Some(site_url) = tool_site_url(tool) else {
                failures.push(format!("{tool}: README/docs missing public tool site URL"));
                continue;
            };

            let head = evidence::context().command(
                EvidenceKey::new("live-site-head", &site_url).tool(tool),
                "curl",
                &["-fsSIL", "--max-time", "10", &site_url],
                &workspace_root(),
            );
            if !head.status_success {
                failures.push(format!(
                    "{tool}: live site {site_url} unreachable: {}",
                    head.stderr.trim()
                ));
            }

            let readme = read(tools_dir().join(tool).join("README.md"));
            if !readme.contains(&site_url) {
                failures.push(format!("{tool}: README does not point to {site_url}"));
            }

            check_github_homepage(tool, &site_url, &mut failures);

            let umbrella = read(workspace_root().join("docs/index.html"));
            if !umbrella.contains(&site_url) {
                failures.push(format!(
                    "workspace: umbrella site does not link to {tool} site {site_url}"
                ));
            }

            let tool_site = read(tools_dir().join(tool).join("docs/index.html"));
            if !tool_site.contains("https://tools.maxeonyx.com") {
                failures.push(format!(
                    "{tool}: tool site does not link back to https://tools.maxeonyx.com"
                ));
            }
        }

        if !failures.is_empty() {
            panic!("landing-page non-compliant:\n  {}", failures.join("\n  "));
        }
    }

    fn check_github_homepage(tool: &str, site_url: &str, failures: &mut Vec<String>) {
        let cargo_toml = read(tools_dir().join(tool).join("Cargo.toml"));
        let Some(repository) = evidence::package_field(&cargo_toml, "repository") else {
            failures.push(format!("{tool}: Cargo.toml missing repository"));
            return;
        };
        let Some(repo) = repository.strip_prefix("https://github.com/") else {
            failures.push(format!("{tool}: repository is not a GitHub URL"));
            return;
        };

        let output = evidence::context().command(
            EvidenceKey::new("github-homepage", repo).tool(tool),
            "gh",
            &[
                "repo",
                "view",
                repo,
                "--json",
                "homepageUrl",
                "-q",
                ".homepageUrl",
            ],
            &workspace_root(),
        );
        if !output.status_success {
            failures.push(format!(
                "{tool}: failed to read GitHub homepage metadata: {}",
                output.stderr.trim()
            ));
            return;
        }
        if output.stdout.trim().trim_end_matches('/') != site_url.trim_end_matches('/') {
            failures.push(format!(
                "{tool}: GitHub homepage is `{}`, expected {site_url}",
                output.stdout.trim()
            ));
        }
    }

    fn tool_site_url(tool: &str) -> Option<String> {
        for file in [
            tools_dir().join(tool).join("docs/index.html"),
            tools_dir().join(tool).join("README.md"),
        ] {
            let content = read(file);
            if let Some(url) = release_site_url(&content) {
                return Some(url);
            }
            for token in content.split(|character: char| {
                character.is_whitespace() || matches!(character, '<' | '"' | '\'' | '`' | ')' | '(')
            }) {
                let url = token.trim_end_matches(|character: char| {
                    matches!(character, ',' | ';' | '.' | '\\')
                });
                if url.starts_with("https://")
                    && url.ends_with(".maxeonyx.com")
                    && !url.contains("tools.maxeonyx.com")
                {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    fn release_site_url(content: &str) -> Option<String> {
        for token in content.split(|character: char| {
            character.is_whitespace() || matches!(character, '<' | '"' | '\'' | '`' | ')' | '(')
        }) {
            let url = token
                .trim_end_matches(|character: char| matches!(character, ',' | ';' | '.' | '\\'));
            if url.starts_with("https://")
                && url.contains(".maxeonyx.com/releases/")
                && !url.contains("${")
            {
                let (site, _) = url.split_once("/releases/")?;
                return Some(site.to_string());
            }
        }
        None
    }

    fn read(path: impl AsRef<std::path::Path>) -> String {
        std::fs::read_to_string(path).unwrap_or_default()
    }
}
