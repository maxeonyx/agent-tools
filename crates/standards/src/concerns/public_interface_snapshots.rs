//! # Public Interface Snapshots
//!
//! Tests for public interfaces should make the reviewed contract visible in the
//! test source. Command output, public function return values, HTTP responses,
//! and rendered components should prefer snapshot-style assertions over scattered
//! substring checks when the shape of the public output matters.
//!
//! A snapshot can be an inline multiline expected string, a named fixture file,
//! or a project snapshot-testing library. The important property is that review
//! shows the whole public artifact and updates are deliberate.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &[];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = r#"
Review the tool's tests for public-interface snapshot quality.

Public interfaces include CLI stdout/stderr/help text, public library API return
values, HTTP API requests/responses, generated files meant for users or machines,
and rendered UI/component output where applicable.

Required review method:
1. Identify tests that assert public output or public return values.
2. Check whether each assertion makes the reviewed artifact visible as a whole.
3. For dynamic output, check that stable structure remains snapshot-like and only
   genuinely dynamic values are interpolated, normalized, or asserted separately.
4. Produce findings with test file/line references. If there are no findings,
   list the public interfaces whose tests use snapshot-style assertions.

Check for:
1. Public CLI output assertions prefer inline multiline expected strings, named
   fixtures, or a snapshot-testing library over fragmented `contains` checks.
2. Public API/HTTP/function/component output assertions show the full public
   shape, not just one or two fields that happen to prove the current test.
3. Snapshot updates are deliberate and reviewable. Automatic update tooling is
   acceptable only when the resulting diff is what reviewers approve.
4. Dynamic values are handled explicitly: interpolate them into an expected
   multiline value, normalize them before comparison, or assert them separately
   while keeping the stable surrounding output snapshot-like.
5. Fragment assertions are reserved for cases where the full output is not the
   contract, for example checking absence of a secret, tolerance of unrelated
   diagnostics, or a deliberately partial property.
6. The snapshot style improves reviewability rather than hiding expectations in
   opaque generated blobs.
"#;

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "public-interface-snapshots",
    definition_summary: "Public interface tests must make expected output and public return shapes reviewable with snapshot-style assertions.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to shipped tools and libraries with public command, API, file, or rendered output surfaces; the workspace root is not itself a shipped public interface.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::concerns;

    #[test]
    fn public_interface_snapshots() {
        let failures =
            concerns::review_attestation_failures("public-interface-snapshots", NOT_APPLICABLE);

        if !failures.is_empty() {
            panic!(
                "public-interface-snapshots non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }
}
