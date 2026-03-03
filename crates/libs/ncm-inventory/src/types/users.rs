use crate::types::common::{ToNix, format_string_list};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default, rename = "sshKeys", skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<String>,
}

impl ToNix for User {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        if !self.ssh_keys.is_empty() {
            parts.push(format!(
                "    sshKeys = {};",
                format_string_list(&self.ssh_keys)
            ));
        }
        parts.push("  }".to_string());
        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_to_nix() {
        let user = User {
            ssh_keys: vec!["ssh-rsa AAA".to_string()],
        };
        assert_eq!(user.to_nix(), "{\n    sshKeys = [ \"ssh-rsa AAA\" ];\n  }");
    }

    #[test]
    fn test_user_from_json() {
        let json = r#"{"sshKeys": ["ssh-rsa AAA"]}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.ssh_keys, vec!["ssh-rsa AAA"]);
    }
}
