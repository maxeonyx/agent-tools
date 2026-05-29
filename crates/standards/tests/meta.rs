//! # Meta: Concern System Integrity
//!
//! Every enforced concern has a test file. Test files must map to real concern
//! IDs. Concerns without a mechanical check are intentionally unenforced for now
//! and should remain visible until a concrete checker exists.

use standards::workspace_root;
use std::collections::HashSet;

const ALL_CONCERNS: &[&str] = &[
    "workspace-routing",
    "tdd-ratchet",
    "black-box-tests",
    "code-standards",
    "help-text",
    "error-messages",
    "landing-page",
    "release-pipeline",
    "auto-update",
    "vision-and-process",
    "opencode-skill",
    "fast-slow-checks",
    "injectable-io",
    "code-review",
];

const ENFORCED_CONCERNS: &[&str] = &[
    "workspace-routing",
    "tdd-ratchet",
    "black-box-tests",
    "code-standards",
    "landing-page",
    "release-pipeline",
    "auto-update",
    "vision-and-process",
    "opencode-skill",
];

const UNENFORCED_CONCERNS: &[&str] = &[
    "help-text",
    "error-messages",
    "fast-slow-checks",
    "injectable-io",
    "code-review",
];

fn concern_id_from_test_file(file_name: &str) -> Option<String> {
    if file_name == "meta.rs" {
        return None;
    }

    Some(file_name.trim_end_matches(".rs").replace('_', "-"))
}

#[test]
fn every_test_file_maps_to_a_real_concern() {
    let tests_dir = workspace_root().join("crates/standards/tests");
    let known_concerns: HashSet<_> = ALL_CONCERNS.iter().copied().collect();
    let mut unknown = Vec::new();

    for entry in std::fs::read_dir(&tests_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", tests_dir.display()))
    {
        let path = entry
            .unwrap_or_else(|error| {
                panic!(
                    "failed to read dir entry in {}: {error}",
                    tests_dir.display()
                )
            })
            .path();
        if !path.extension().is_some_and(|extension| extension == "rs") {
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy();
        let Some(concern_id) = concern_id_from_test_file(&file_name) else {
            continue;
        };

        if !known_concerns.contains(concern_id.as_str()) {
            unknown.push(format!("{} -> {}", file_name, concern_id));
        }
    }

    if !unknown.is_empty() {
        panic!("unknown concern test files:\n  {}", unknown.join("\n  "));
    }
}

#[test]
fn enforced_concern_list_matches_test_files() {
    let tests_dir = workspace_root().join("crates/standards/tests");
    let mut actual = HashSet::new();

    for entry in std::fs::read_dir(&tests_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", tests_dir.display()))
    {
        let path = entry
            .unwrap_or_else(|error| {
                panic!(
                    "failed to read dir entry in {}: {error}",
                    tests_dir.display()
                )
            })
            .path();
        if !path.extension().is_some_and(|extension| extension == "rs") {
            continue;
        }

        let file_name = path.file_name().unwrap().to_string_lossy();
        let Some(concern_id) = concern_id_from_test_file(&file_name) else {
            continue;
        };
        actual.insert(concern_id);
    }

    let expected: HashSet<_> = ENFORCED_CONCERNS
        .iter()
        .map(|concern| concern.to_string())
        .collect();
    assert_eq!(
        actual, expected,
        "enforced concern list drifted from test files"
    );
}

#[test]
fn unenforced_concerns_are_exactly_the_known_non_mechanical_set() {
    let all: HashSet<_> = ALL_CONCERNS.iter().copied().collect();
    let enforced: HashSet<_> = ENFORCED_CONCERNS.iter().copied().collect();
    let expected_unenforced: HashSet<_> = all.difference(&enforced).copied().collect();
    let declared_unenforced: HashSet<_> = UNENFORCED_CONCERNS.iter().copied().collect();

    assert_eq!(
        expected_unenforced, declared_unenforced,
        "unenforced concerns changed; update UNENFORCED_CONCERNS"
    );
}
