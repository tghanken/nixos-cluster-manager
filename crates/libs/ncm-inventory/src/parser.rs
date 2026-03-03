use anyhow::{Context, Result};
use duct::cmd;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn parse_nix_file<T: DeserializeOwned>(path: &Path) -> Result<T> {
    // `duct` handles process cancellation and cleanup better than std::process::Command.
    // If the thread is killed or the handle dropped, the child process is terminated.
    // Here we use `read()` which blocks and captures stdout.
    // If an error occurs (non-zero exit), `read()` returns an error.
    let json_str = cmd!("nix", "eval", "--json", "-f", path)
        .read()
        .context("Failed to execute nix eval or it returned non-zero exit code")?;

    serde_json::from_str(&json_str).context("Failed to parse JSON output from nix eval")
}
