use crate::types::common::{ToNix, format_string_list};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(default, rename = "rekeyUsers", skip_serializing_if = "Vec::is_empty")]
    pub rekey_users: Vec<String>,
}

impl ToNix for Secret {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());
        if !self.rekey_users.is_empty() {
            parts.push(format!(
                "    rekeyUsers = {};",
                format_string_list(&self.rekey_users)
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
    fn test_secret_to_nix() {
        let secret = Secret {
            rekey_users: vec!["admin".to_string()],
        };
        assert_eq!(secret.to_nix(), "{\n    rekeyUsers = [ \"admin\" ];\n  }");
    }

    #[test]
    fn test_secret_from_json() {
        let json = r#"{"rekeyUsers": ["admin"]}"#;
        let secret: Secret = serde_json::from_str(json).unwrap();
        assert_eq!(secret.rekey_users, vec!["admin"]);
    }
}
