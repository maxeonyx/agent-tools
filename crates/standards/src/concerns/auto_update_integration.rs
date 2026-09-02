//! # Auto-update Integration
//!
//! Auto-update is not real until an already-installed released binary can
//! replace itself and the replacement still runs. Dependency and call-site
//! checks are useful structural backstops, but they cannot prove that users
//! actually receive a working update.
//!
//! Compliance therefore installs the previous Linux release in a temporary
//! prefix, points its forced-update hook at the binary built from the pinned
//! source, runs the old binary, verifies its bytes were replaced, and executes
//! the installed binary again to verify the new version. Tagged release assets
//! are cached under `target/standards-evidence`; the replacement source is
//! injected locally so the update boundary is deterministic and diagnosable.

/// Tools where this concern does not apply.
pub const NOT_APPLICABLE: &[&str] = &["tdd-ratchet"];

/// Instructions for an agent performing this review.
pub const REVIEW_INSTRUCTIONS: &str = "";

pub const SPEC: crate::concerns::ConcernSpec = crate::concerns::ConcernSpec {
    id: "auto-update-integration",
    definition_summary: "Applicable CLIs must replace a previous released binary through their forced-update path and successfully execute the replacement.",
    review_instructions: REVIEW_INSTRUCTIONS,
    applies_to_workspace: false,
    applicability_note: "Applies to release-binary-installed CLI tools; not to the workspace itself or cargo-install-first tools like tdd-ratchet.",
};

#[cfg(test)]
mod tests {
    use super::NOT_APPLICABLE;
    use crate::evidence;
    use crate::{checked_tools, libraries_dir, tools_dir, workspace_root};
    use serde_json::Value;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    const FORCE_ENV: &str = "AGENT_TOOLS_UPDATE_FORCE";
    const SOURCE_ENV: &str = "AGENT_TOOLS_UPDATE_SOURCE";

    #[test]
    fn auto_update_integration() {
        let mut failures = Vec::new();

        check_shared_updater(&mut failures);

        for tool in checked_tools().filter(|tool| !NOT_APPLICABLE.contains(tool)) {
            let tool_dir = tools_dir().join(tool);
            let cargo_toml = std::fs::read_to_string(tool_dir.join("Cargo.toml"))
                .unwrap_or_else(|error| panic!("failed to read {tool} Cargo.toml: {error}"));
            if !cargo_toml.contains("agent-tools-updater") {
                failures.push(format!("{tool}: Cargo.toml missing agent-tools-updater"));
                continue;
            }

            let source = tree_contents(&tool_dir.join("src"));
            if !source.contains("agent_tools_updater::") {
                failures.push(format!(
                    "{tool}: no structural call site for agent_tools_updater"
                ));
            }

            check_released_binary_update(tool, &cargo_toml, &mut failures);
        }

        if !failures.is_empty() {
            panic!(
                "auto-update-integration non-compliant:\n  {}",
                failures.join("\n  ")
            );
        }
    }

    fn check_shared_updater(failures: &mut Vec<String>) {
        let updater = libraries_dir().join("agent-tools-updater/Cargo.toml");
        check_shared_updater_at(&updater, failures);
    }

    fn check_shared_updater_at(updater: &Path, failures: &mut Vec<String>) {
        if !updater.is_file() {
            failures.push(
                "agent-tools-updater: standalone library missing from libraries/agent-tools-updater"
                    .to_string(),
            );
        }
    }

    #[test]
    fn missing_updater_is_reported_instead_of_panicking() {
        let updater = workspace_root()
            .join("target/standards-fixtures/missing-agent-tools-updater/Cargo.toml");
        let mut failures = Vec::new();
        check_shared_updater_at(&updater, &mut failures);
        assert_eq!(
            failures,
            vec![
                "agent-tools-updater: standalone library missing from libraries/agent-tools-updater"
            ]
        );
    }

    fn check_released_binary_update(tool: &str, cargo_toml: &str, failures: &mut Vec<String>) {
        let package = required_package_field(tool, cargo_toml, "name");
        let version = required_package_field(tool, cargo_toml, "version");
        let repository = required_package_field(tool, cargo_toml, "repository");
        let Some(repo) = repository.strip_prefix("https://github.com/") else {
            failures.push(format!("{tool}: repository is not a GitHub URL"));
            return;
        };
        let binary = evidence::binary_name(cargo_toml).unwrap_or(package);

        let current_binary = match build_current_binary(tool, binary) {
            Ok(path) => path,
            Err(error) => {
                failures.push(format!("{tool}: {error}"));
                return;
            }
        };
        let previous_tag = match previous_release_tag(repo, version) {
            Ok(tag) => tag,
            Err(error) => {
                failures.push(format!("{tool}: {error}"));
                return;
            }
        };
        let released_binary = match cached_release_binary(repo, &previous_tag, binary) {
            Ok(path) => path,
            Err(error) => {
                failures.push(format!("{tool}: {error}"));
                return;
            }
        };

        let fixture = workspace_root().join(format!(
            "target/standards-fixtures/auto-update-{tool}-{}",
            std::process::id()
        ));
        let installed = fixture.join(binary);
        let result = (|| -> Result<(), String> {
            remove_dir_if_present(&fixture)?;
            std::fs::create_dir_all(&fixture)
                .map_err(|error| format!("could not create temporary install prefix: {error}"))?;
            std::fs::copy(&released_binary, &installed)
                .map_err(|error| format!("could not install previous release: {error}"))?;
            make_executable(&installed)?;
            verify_forced_update(&installed, &current_binary, &format!("{binary} {version}"))
        })();
        let cleanup = remove_dir_if_present(&fixture);

        if let Err(error) = result {
            failures.push(format!(
                "{tool}: previous release {previous_tag} failed black-box update: {error}"
            ));
        }
        if let Err(error) = cleanup {
            failures.push(format!("{tool}: {error}"));
        }
    }

    fn build_current_binary(tool: &str, binary: &str) -> Result<PathBuf, String> {
        let tool_dir = tools_dir().join(tool);
        let output = Command::new("cargo")
            .args(["build", "--quiet", "--locked", "--bin", binary])
            .current_dir(&tool_dir)
            .output()
            .map_err(|error| format!("cargo build could not start: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "cargo build failed with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        Ok(tool_dir.join("target/debug").join(binary))
    }

    fn previous_release_tag(repo: &str, current_version: &str) -> Result<String, String> {
        let output = Command::new("gh")
            .args([
                "release",
                "list",
                "--repo",
                repo,
                "--limit",
                "10",
                "--exclude-drafts",
                "--exclude-pre-releases",
                "--json",
                "tagName",
            ])
            .output()
            .map_err(|error| format!("GitHub release query could not start: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "GitHub release query failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }

        let releases: Value = serde_json::from_slice(&output.stdout)
            .map_err(|error| format!("GitHub release query returned invalid JSON: {error}"))?;
        let current_tag = format!("v{current_version}");
        releases
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|release| release.get("tagName").and_then(Value::as_str))
            .find(|tag| *tag != current_tag)
            .map(str::to_string)
            .ok_or_else(|| format!("no release before {current_tag} is available"))
    }

    fn cached_release_binary(repo: &str, tag: &str, binary: &str) -> Result<PathBuf, String> {
        let asset = format!("{binary}-x86_64-linux");
        let cache = workspace_root()
            .join("target/standards-evidence/auto-update")
            .join(repo.replace('/', "--"))
            .join(tag)
            .join(&asset);
        if cache.metadata().is_ok_and(|metadata| metadata.len() > 0) {
            return Ok(cache);
        }

        let parent = cache
            .parent()
            .ok_or_else(|| "release cache path has no parent".to_string())?;
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("could not create release cache: {error}"))?;
        let partial = cache.with_extension("partial");
        let url = format!("https://github.com/{repo}/releases/download/{tag}/{asset}");
        let output = Command::new("curl")
            .args(["-fL", "--retry", "2", "--max-time", "60", "-o"])
            .arg(&partial)
            .arg(&url)
            .output()
            .map_err(|error| format!("release download could not start: {error}"))?;
        if !output.status.success() {
            let _ = std::fs::remove_file(&partial);
            return Err(format!(
                "could not download {url}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        std::fs::rename(&partial, &cache)
            .map_err(|error| format!("could not commit cached release asset: {error}"))?;
        Ok(cache)
    }

    fn verify_forced_update(
        _installed: &Path,
        _replacement: &Path,
        _expected_version: &str,
    ) -> Result<(), String> {
        panic!("red: forced-update evidence boundary is not implemented")
    }

    #[test]
    fn forced_update_proof_accepts_replaced_and_executable_binary() {
        let fixture = fixture_dir("success");
        let installed = fixture.join("fixture");
        let replacement = fixture.join("replacement");
        prepare_fixture(&fixture);
        write_executable(
            &installed,
            r#"#!/bin/sh
if [ "${AGENT_TOOLS_UPDATE_FORCE:-}" = "1" ] && [ -n "${AGENT_TOOLS_UPDATE_SOURCE:-}" ]; then
  cp "$AGENT_TOOLS_UPDATE_SOURCE" "$0.update"
  chmod +x "$0.update"
  mv "$0.update" "$0"
fi
printf '%s\n' 'fixture 1.0.0'
"#,
        );
        write_executable(&replacement, "#!/bin/sh\nprintf '%s\\n' 'fixture 2.0.0'\n");

        let result = verify_forced_update(&installed, &replacement, "fixture 2.0.0");
        remove_dir_if_present(&fixture).expect("remove success fixture");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn forced_update_proof_rejects_unchanged_binary() {
        let fixture = fixture_dir("unchanged");
        let installed = fixture.join("fixture");
        let replacement = fixture.join("replacement");
        prepare_fixture(&fixture);
        write_executable(&installed, "#!/bin/sh\nprintf '%s\\n' 'fixture 1.0.0'\n");
        write_executable(&replacement, "#!/bin/sh\nprintf '%s\\n' 'fixture 2.0.0'\n");

        let error = verify_forced_update(&installed, &replacement, "fixture 2.0.0")
            .expect_err("an unchanged installed binary must fail");
        remove_dir_if_present(&fixture).expect("remove unchanged fixture");
        assert!(
            error.contains("did not replace"),
            "unexpected error: {error}"
        );
    }

    fn fixture_dir(name: &str) -> PathBuf {
        workspace_root().join(format!(
            "target/standards-fixtures/auto-update-{name}-{}",
            std::process::id()
        ))
    }

    fn prepare_fixture(path: &Path) {
        remove_dir_if_present(path).expect("remove stale fixture");
        std::fs::create_dir_all(path).expect("create fixture");
    }

    fn write_executable(path: &Path, content: &str) {
        std::fs::write(path, content).expect("write fixture executable");
        make_executable(path).expect("make fixture executable");
    }

    fn make_executable(path: &Path) -> Result<(), String> {
        let mut permissions = std::fs::metadata(path)
            .map_err(|error| format!("could not inspect {}: {error}", path.display()))?
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions)
            .map_err(|error| format!("could not make {} executable: {error}", path.display()))
    }

    fn remove_dir_if_present(path: &Path) -> Result<(), String> {
        match std::fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(format!("could not remove {}: {error}", path.display())),
        }
    }

    fn required_package_field<'a>(tool: &str, cargo_toml: &'a str, field: &str) -> &'a str {
        evidence::package_field(cargo_toml, field)
            .unwrap_or_else(|| panic!("{tool}: Cargo.toml missing package {field}"))
    }

    fn tree_contents(root: &Path) -> String {
        let mut content = String::new();
        collect_rs_contents(root, &mut content);
        content
    }

    fn collect_rs_contents(path: &Path, content: &mut String) {
        let Ok(metadata) = std::fs::metadata(path) else {
            return;
        };

        if metadata.is_file() {
            if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
                content.push_str(&std::fs::read_to_string(path).unwrap_or_default());
            }
            return;
        }

        let entries = std::fs::read_dir(path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for entry in entries.filter_map(Result::ok) {
            collect_rs_contents(&entry.path(), content);
        }
    }
}
