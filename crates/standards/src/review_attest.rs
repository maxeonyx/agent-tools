use crate::concerns::{
    concern_spec, latest_substantive_commit, load_state_file, parse_review_attestation,
    state_file_path, ConcernSpec, ReviewAttestation, StateFile, AGENTIC_CONCERNS,
};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReviewAction {
    Prompt,
    Record,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewTarget {
    pub repo_name: String,
    pub repo_dir: PathBuf,
}

pub fn resolve_target(name: &str) -> Result<ReviewTarget, String> {
    if name == "workspace" {
        return Ok(ReviewTarget {
            repo_name: "workspace".to_string(),
            repo_dir: crate::workspace_root(),
        });
    }

    if crate::TOOLS.contains(&name) {
        return Ok(ReviewTarget {
            repo_name: name.to_string(),
            repo_dir: crate::tools_dir().join(name),
        });
    }

    Err(format!(
        "unknown review target {name}; use `workspace` or one of: {}",
        crate::TOOLS.join(", ")
    ))
}

pub fn resolve_agentic_concern(id: &str) -> Result<&'static ConcernSpec, String> {
    let spec = concern_spec(id).ok_or_else(|| format!("unknown concern `{id}`"))?;
    if spec.review_instructions.trim().is_empty() {
        return Err(format!(
            "concern `{id}` is not an agentic/manual review concern"
        ));
    }
    Ok(spec)
}

pub fn render_prompt(target: &ReviewTarget, spec: &ConcernSpec, reviewed_commit: &str) -> String {
    format!(
        "Review target: {repo}\nPath: {path}\nConcern: {concern}\nState file: {file}\nReviewed commit: {commit}\n\nProcess:\n1. Start a fresh review session independent from any implementer that changed this target.\n2. Evaluate the target against the instructions below.\n3. If the target satisfies the concern, record the attestation with `cargo run -p standards --bin review-attest -- record {repo} {concern}` and report what you reviewed.\n4. If the target does not satisfy the concern, do not record an attestation. Return detailed findings by concern with concrete files, commands, observed behavior, and expected behavior.\n5. Do not implement fixes in the review session. After fixes, a fresh independent review is required before attestation.\n\nInstructions:\n{instructions}",
        repo = target.repo_name,
        path = target.repo_dir.display(),
        concern = spec.id,
        file = state_file_path().display(),
        commit = reviewed_commit,
        instructions = spec.review_instructions.trim()
    )
}

pub fn render_attestation(
    target: &ReviewTarget,
    spec: &ConcernSpec,
    reviewed_commit: &str,
) -> String {
    let attestation = ReviewAttestation {
        reviewed_commit: reviewed_commit.to_string(),
        concern: spec.id.to_string(),
        repo: target.repo_name.clone(),
        attested_via: "review-attest".to_string(),
    };
    serde_json::to_string_pretty(&attestation).expect("attestation JSON should serialize") + "\n"
}

pub fn perform(
    action: ReviewAction,
    target_name: &str,
    concern_id: &str,
) -> Result<String, String> {
    let target = resolve_target(target_name)?;
    let spec = resolve_agentic_concern(concern_id)?;
    let reviewed_commit = latest_substantive_commit(&target.repo_dir)?;

    match action {
        ReviewAction::Prompt => Ok(render_prompt(&target, spec, &reviewed_commit)),
        ReviewAction::Record => {
            let state_path = state_file_path();
            let mut state = match load_state_file() {
                Ok(state) => state,
                Err(error) if !state_path.exists() => {
                    let _ = error;
                    StateFile::default()
                }
                Err(error) => {
                    return Err(format!("failed to load {}: {error}", state_path.display()))
                }
            };
            state
                .reviews
                .retain(|entry| !(entry.repo == target.repo_name && entry.concern == spec.id));
            let attestation = render_attestation(&target, spec, &reviewed_commit);
            let parsed = parse_review_attestation(&attestation)?;
            state.reviews.push(parsed);
            let content =
                serde_json::to_string_pretty(&state).map_err(|error| error.to_string())? + "\n";
            std::fs::write(&state_path, content).map_err(|error| error.to_string())?;
            Ok(format!(
                "Recorded attestation for {repo} {concern} in {path}\n{attestation}",
                repo = target.repo_name,
                concern = spec.id,
                path = state_path.display()
            ))
        }
    }
}

pub fn usage(program: &str) -> String {
    format!(
        "Usage:\n  {program} prompt <workspace|tool> <agentic-concern>\n  {program} record <workspace|tool> <agentic-concern>\n\nAgentic concerns: {}",
        AGENTIC_CONCERNS.join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    #[test]
    fn resolve_agentic_concern_rejects_non_agentic() {
        let error = resolve_agentic_concern("version-artifacts").unwrap_err();
        assert!(error.contains("not an agentic/manual review concern"));
    }

    #[test]
    fn render_prompt_includes_review_file_and_commit() {
        let spec = resolve_agentic_concern("code-review").unwrap();
        let target = ReviewTarget {
            repo_name: "workspace".to_string(),
            repo_dir: PathBuf::from("/tmp/workspace"),
        };
        let prompt = render_prompt(&target, spec, "abc123");
        assert!(prompt.contains("state.json"));
        assert!(prompt.contains("abc123"));
        assert!(prompt.contains("Do not implement fixes in the review session"));
        assert!(prompt.contains("fresh independent review is required"));
    }

    #[test]
    fn render_attestation_is_machine_readable_json() {
        let spec = resolve_agentic_concern("help-text").unwrap();
        let target = ReviewTarget {
            repo_name: "trunc".to_string(),
            repo_dir: PathBuf::from("/tmp/trunc"),
        };
        let json = render_attestation(&target, spec, "deadbeef");
        let parsed = parse_review_attestation(&json).unwrap();
        assert_eq!(parsed.reviewed_commit, "deadbeef");
        assert_eq!(parsed.concern, "help-text");
        assert_eq!(parsed.repo, "trunc");
        assert_eq!(parsed.attested_via, "review-attest");
    }

    #[test]
    fn record_attestation_writes_tool_schema() {
        let repo = test_repo_dir("record_attestation");
        init_git_repo(&repo);

        fs::write(repo.join("src.txt"), "v1\n").unwrap();
        git(&repo, &["add", "src.txt"]);
        git(&repo, &["commit", "-m", "substantive"]);
        let reviewed_commit = latest_substantive_commit(&repo).unwrap();

        let target = ReviewTarget {
            repo_name: "workspace".to_string(),
            repo_dir: repo.clone(),
        };
        let spec = resolve_agentic_concern("code-review").unwrap();
        let json = render_attestation(&target, spec, &reviewed_commit);
        let parsed = parse_review_attestation(&json).unwrap();
        assert_eq!(parsed.reviewed_commit, reviewed_commit);
        assert_eq!(parsed.concern, "code-review");
        assert_eq!(parsed.repo, "workspace");
        assert_eq!(parsed.attested_via, "review-attest");
    }

    fn test_repo_dir(name: &str) -> PathBuf {
        let path = crate::workspace_root()
            .join("target")
            .join("standards-fixtures")
            .join(name);
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn init_git_repo(path: &std::path::Path) {
        git(path, &["init", "-b", "main"]);
        git(path, &["config", "user.name", "Fixture User"]);
        git(path, &["config", "user.email", "fixture@example.com"]);
    }

    fn git(path: &std::path::Path, args: &[&str]) {
        let status = Command::new("git")
            .args(args)
            .current_dir(path)
            .status()
            .unwrap();
        assert!(
            status.success(),
            "git {:?} failed in {}",
            args,
            path.display()
        );
    }
}
