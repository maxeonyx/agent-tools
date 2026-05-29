//! Workspace standards enforcement.
//!
//! This crate has no runtime purpose. Its concern modules define cross-cutting
//! standards and their mechanical tests fail until every applicable tool
//! complies. Run with `cargo test -p standards`.

use std::path::{Path, PathBuf};

pub mod concerns;

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn tools_dir() -> PathBuf {
    workspace_root().join("tools")
}

pub const TOOLS: &[&str] = &["trunc", "tb", "dotsync", "tdd-ratchet", "oc"];
