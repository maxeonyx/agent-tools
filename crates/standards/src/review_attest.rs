use crate::concerns::{concern_spec, latest_substantive_commit, ConcernSpec, ReviewAttestation};
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
    if spec.review_instructions.trim().is_empty() || spec.review_file_name.is_none() {
        return Err(format!(
            "concern `{id}` is not an agentic/manual review concern"
        ));
    }
    Ok(spec)
}

pub fn render_prompt(target: &ReviewTarget, spec: &ConcernSpec, reviewed_commit: &str) -> String {
    format!(
        "Review target: {repo}\nPath: {path}\nConcern: {concern}\nAttestation file: {file}\nReviewed commit: {commit}\n\nProcess:\n1. Start a fresh review session.\n2. Evaluate the target against the instructions below.\n3. Produce findings and implement/fix them before recording an attestation.\n4. Only record an attestation when the re-review is clean.\n\nInstructions:\n{instructions}",
        repo = target.repo_name,
        path = target.repo_dir.display(),
        concern = spec.id,
        file = spec.review_file_name.unwrap_or(""),
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
            let review_file = target.repo_dir.join(
                spec.review_file_name
                    .expect("agentic concern must have review file"),
            );
            if let Some(parent) = review_file.parent() {
                std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            let attestation = render_attestation(&target, spec, &reviewed_commit);
            std::fs::write(&review_file, &attestation).map_err(|error| error.to_string())?;
            Ok(format!(
                "Recorded attestation for {repo} {concern} at {path}\n{attestation}",
                repo = target.repo_name,
                concern = spec.id,
                path = review_file.display()
            ))
        }
    }
}

pub fn usage(program: &str) -> String {
    format!(
        "Usage:\n  {program} prompt <workspace|tool> <agentic-concern>\n  {program} record <workspace|tool> <agentic-concern>\n\nAgentic concerns: code-review, error-messages, help-text, injectable-io",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concerns::parse_review_attestation;
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
        assert!(prompt.contains("docs/reviews/code-quality.json"));
        assert!(prompt.contains("abc123"));
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
        let path = repo.join(spec.review_file_name.unwrap());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let json = render_attestation(&target, spec, &reviewed_commit);
        fs::write(&path, &json).unwrap();

        let parsed = parse_review_attestation(&fs::read_to_string(&path).unwrap()).unwrap();
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
