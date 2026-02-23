use serde::{Deserialize, Serialize};
use crate::types::common::{ToNix, format_string_list};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeployConfig {
    #[serde(rename = "sshKeys")]
    pub ssh_keys: Vec<String>,
}

impl ToNix for DeployConfig {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        if !self.ssh_keys.is_empty() {
            parts.push(format!("  sshKeys = {};", format_string_list(&self.ssh_keys)));
        }
        parts.push("}".to_string());
        parts.join("\n")
    }
}
