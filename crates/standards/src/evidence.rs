//! Shared evidence gathering for standards concerns.
//!
//! Concerns express policy. This module observes reusable facts once per
//! standards process so several concerns can interpret the same evidence without
//! repeating slow or remote work. Cache keys include commit fields where they are
//! known so they can be reused for persistent commit-keyed caching later.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvidenceKey {
    pub repo: String,
    pub tool: Option<String>,
    pub kind: String,
    pub target: String,
    pub version: Option<String>,
    pub commit: Option<String>,
}

impl EvidenceKey {
    pub fn new(kind: impl ToString, target: impl ToString) -> Self {
        Self {
            repo: "workspace".to_string(),
            tool: None,
            kind: kind.to_string(),
            target: target.to_string(),
            version: None,
            commit: None,
        }
    }

    pub fn repo(mut self, repo: impl ToString) -> Self {
        self.repo = repo.to_string();
        self
    }

    pub fn tool(mut self, tool: impl ToString) -> Self {
        self.tool = Some(tool.to_string());
        self
    }

    pub fn version(mut self, version: impl ToString) -> Self {
        self.version = Some(version.to_string());
        self
    }

    pub fn commit(mut self, commit: impl ToString) -> Self {
        self.commit = Some(commit.to_string());
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandEvidence {
    pub status_success: bool,
    pub status: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Default, Debug)]
pub struct EvidenceContext {
    commands: Mutex<BTreeMap<EvidenceKey, CommandEvidence>>,
}

impl EvidenceContext {
    pub fn command(
        &self,
        key: EvidenceKey,
        program: &str,
        args: &[&str],
        current_dir: &Path,
    ) -> CommandEvidence {
        self.command_with_runner(key, || {
            let output = Command::new(program)
                .args(args)
                .current_dir(current_dir)
                .output()
                .unwrap_or_else(|error| panic!("failed to run {program}: {error}"));

            CommandEvidence {
                status_success: output.status.success(),
                status: output.status.to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            }
        })
    }

    pub fn command_with_runner(
        &self,
        key: EvidenceKey,
        runner: impl FnOnce() -> CommandEvidence,
    ) -> CommandEvidence {
        let mut commands = self
            .commands
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(cached) = commands.get(&key) {
            return cached.clone();
        }

        let evidence = runner();
        commands.insert(key, evidence.clone());
        evidence
    }
}

pub fn context() -> &'static EvidenceContext {
    static CONTEXT: OnceLock<EvidenceContext> = OnceLock::new();
    CONTEXT.get_or_init(EvidenceContext::default)
}

pub fn tool_commit(tool_dir: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(tool_dir)
        .output()
        .map_err(|error| format!("git failed to start: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn package_field<'a>(cargo_toml: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("{field} = \"");
    cargo_toml.lines().find_map(|line| {
        line.trim()
            .strip_prefix(&prefix)?
            .split_once('"')
            .map(|(value, _)| value)
    })
}

pub fn binary_name<'a>(cargo_toml: &'a str) -> Option<&'a str> {
    let mut in_bin = false;
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed == "[[bin]]" {
            in_bin = true;
            continue;
        }
        if in_bin && trimmed.starts_with('[') {
            in_bin = false;
        }
        if in_bin {
            if let Some(value) = trimmed.strip_prefix("name = \"") {
                return value.split_once('"').map(|(name, _)| name);
            }
        }
    }
    None
}

pub fn target_debug_binary(binary: &str) -> PathBuf {
    crate::workspace_root().join("target/debug").join(binary)
}

#[cfg(test)]
mod tests {
    use super::{CommandEvidence, EvidenceContext, EvidenceKey};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn same_evidence_request_runs_once() {
        let context = EvidenceContext::default();
        let calls = AtomicUsize::new(0);
        let key = EvidenceKey::new("unit-test", "same-target")
            .tool("trunc")
            .commit("abc123");

        let first = context.command_with_runner(key.clone(), || {
            calls.fetch_add(1, Ordering::SeqCst);
            CommandEvidence {
                status_success: true,
                status: "exit status: 0".to_string(),
                stdout: "first".to_string(),
                stderr: String::new(),
            }
        });
        let second = context.command_with_runner(key, || {
            calls.fetch_add(1, Ordering::SeqCst);
            CommandEvidence {
                status_success: true,
                status: "exit status: 0".to_string(),
                stdout: "second".to_string(),
                stderr: String::new(),
            }
        });

        assert_eq!(first.stdout, "first");
        assert_eq!(second.stdout, "first");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
