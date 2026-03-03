use crate::types::common::{ToNix, format_string_list};
use serde::{Deserialize, Serialize};

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
            parts.push(format!(
                "  sshKeys = {};",
                format_string_list(&self.ssh_keys)
            ));
        }
        parts.push("}".to_string());
        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deploy_config_to_nix() {
        let config = DeployConfig {
            ssh_keys: vec!["ssh-rsa AAA...".to_string()],
        };
        assert_eq!(config.to_nix(), "{\n  sshKeys = [ \"ssh-rsa AAA...\" ];\n}");
    }

    #[test]
    fn test_deploy_config_from_json() {
        let json = r#"{"sshKeys": ["ssh-rsa AAA..."]}"#;
        let config: DeployConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.ssh_keys, vec!["ssh-rsa AAA..."]);
    }
}
