//! # Website with Standards
//!
//! Each tool should have a landing page with a consistent aesthetic and content.
//!
//! These tools are public. People and agents discover them via their websites.
//! A consistent design language signals that they are a cohesive suite, and the
//! site is where install instructions and ecosystem links stay current.
//!
//! The mechanical floor here is simple: each tool must publish `docs/index.html`.

use standards::{tools_dir, TOOLS};

const NOT_APPLICABLE: &[&str] = &[];

#[test]
fn landing_page() {
    let mut failures = Vec::new();

    for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
        let index_html = tools_dir().join(tool).join("docs/index.html");
        if !index_html.exists() {
            failures.push(format!("{tool}: docs/index.html missing"));
        }
    }

    if !failures.is_empty() {
        panic!("landing-page non-compliant:\n  {}", failures.join("\n  "));
    }
}
