use serde::{Deserialize, Serialize};
use crate::types::common::{ToNix, format_string_list};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MachineDeployment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
}

impl ToNix for MachineDeployment {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        if let Some(ip) = &self.ipv4 {
            parts.push(format!("      ipv4 = \"{}\";", ip));
        }
        if let Some(ip) = &self.ipv6 {
            parts.push(format!("      ipv6 = \"{}\";", ip));
        }
        parts.push("    }".to_string());
        parts.join("\n")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(default, rename = "sshKeys", skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
    #[serde(default, rename = "node_services", skip_serializing_if = "Vec::is_empty")]
    pub node_services: Vec<String>,
    #[serde(default, rename = "bootDisks", skip_serializing_if = "Vec::is_empty")]
    pub boot_disks: Vec<String>,
    pub deployment: MachineDeployment,
}

impl ToNix for Machine {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        if !self.ssh_keys.is_empty() {
            parts.push(format!("    sshKeys = {};", format_string_list(&self.ssh_keys)));
        }
        if !self.users.is_empty() {
            parts.push(format!("    users = {};", format_string_list(&self.users)));
        }
        if !self.node_services.is_empty() {
            parts.push(format!("    node_services = {};", format_string_list(&self.node_services)));
        }
        if !self.boot_disks.is_empty() {
            parts.push(format!("    bootDisks = {};", format_string_list(&self.boot_disks)));
        }
        parts.push(format!("    deployment = {};", self.deployment.to_nix()));
        parts.push("  }".to_string());
        parts.join("\n")
    }
}
