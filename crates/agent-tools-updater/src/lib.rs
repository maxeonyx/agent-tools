//! Auto-update mechanism for agent-tools binaries.
//!
//! Checks GitHub Releases for new versions and downloads updates in the background.
//! Each tool calls this at startup — if an update was previously downloaded,
//! it replaces itself. If not, it kicks off a background check for next time.
