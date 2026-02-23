use serde::{Deserialize, Serialize};
use crate::types::common::{ToNix, format_string_list};

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
            parts.push(format!("    rekeyUsers = {};", format_string_list(&self.rekey_users)));
        }
        parts.push("  }".to_string());
        parts.join("\n")
    }
}
