//! # Black-box Test Architecture
//!
//! Tests should spawn the binary and check behavior from outside. No internal
//! imports.
//!
//! Tests coupled to implementation break when you refactor, even if behavior is
//! unchanged. Black-box tests verify the contract — input goes in, correct
//! output comes out. They survive refactoring because they don't know or care
//! about internal structure.
//!
//! For CLI tools, the binary is the interface. Compliance means a `tests/`
//! directory exists and the crate uses `assert_cmd` for subprocess-driven tests.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::{tools_dir, TOOLS};

    #[test]
    fn black_box_tests() {
        let mut failures = Vec::new();

        for tool in TOOLS.iter().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_path = tools_dir().join(tool);
            if !tool_path.join("tests").is_dir() {
                failures.push(format!("{tool}: tests/ directory missing"));
            }

            let cargo_toml = tool_path.join("Cargo.toml");
            let cargo_toml_contents = std::fs::read_to_string(&cargo_toml)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", cargo_toml.display()));
            if !cargo_toml_contents.contains("assert_cmd") {
                failures.push(format!("{tool}: Cargo.toml missing assert_cmd"));
            }
        }

        if !failures.is_empty() {
            panic!(
                "black-box-tests non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
