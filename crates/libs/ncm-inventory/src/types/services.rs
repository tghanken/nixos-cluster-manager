use serde::{Deserialize, Serialize};
use crate::types::common::{ToNix, format_string_list};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackupStrategy {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub command: String,
    pub restore: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl ToNix for BackupStrategy {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        parts.push(format!("      name = \"{}\";", self.name));
        parts.push(format!("      type = \"{}\";", self.type_));
        parts.push(format!("      command = \"{}\";", self.command));
        parts.push(format!("      restore = \"{}\";", self.restore));
        if let Some(p) = self.priority {
            parts.push(format!("      priority = {};", p));
        }
        parts.push("    }".to_string());
        parts.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(default = "default_true")]
    pub enable: bool,
    #[serde(default = "default_one")]
    pub replicas: i32,
    #[serde(default, rename = "stateDirs", skip_serializing_if = "Vec::is_empty")]
    pub state_dirs: Vec<String>,
    #[serde(default, rename = "dependsOn", skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub backups: Vec<BackupStrategy>,
}

fn default_true() -> bool { true }
fn default_one() -> i32 { 1 }

impl ToNix for Service {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        parts.push(format!("    enable = {};", self.enable));
        parts.push(format!("    replicas = {};", self.replicas));
        if !self.state_dirs.is_empty() {
            parts.push(format!("    stateDirs = {};", format_string_list(&self.state_dirs)));
        }
        if !self.depends_on.is_empty() {
            parts.push(format!("    dependsOn = {};", format_string_list(&self.depends_on)));
        }
        if !self.secrets.is_empty() {
            parts.push(format!("    secrets = {};", format_string_list(&self.secrets)));
        }
        if !self.backups.is_empty() {
            let backups_str: Vec<String> = self.backups.iter().map(|b| b.to_nix()).collect();
            parts.push(format!("    backups = [\n{}\n    ];", backups_str.join("\n")));
        }
        parts.push("  }".to_string());
        parts.join("\n")
    }
}
