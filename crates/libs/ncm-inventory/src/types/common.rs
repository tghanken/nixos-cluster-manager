use std::collections::HashMap;

/// Trait for converting a struct to a Nix attribute set string.
pub trait ToNix {
    fn to_nix(&self) -> String;
}

// Helper to escape strings for Nix.
// Note: This is a very basic implementation. Nix strings can be complex.
// For now, we escape backslashes and double quotes.
// TODO: Handle more complex cases if needed (e.g. newlines, multiline strings).
pub fn escape_nix_string(s: &str) -> String {
    let mut escaped = String::new();
    escaped.push('"');
    for c in s.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            _ => escaped.push(c),
        }
    }
    escaped.push('"');
    escaped
}

impl<T: ToNix> ToNix for HashMap<String, T> {
    fn to_nix(&self) -> String {
        let mut parts = Vec::new();
        parts.push("{".to_string());

        // Sort keys for deterministic output
        let mut keys: Vec<&String> = self.keys().collect();
        keys.sort();

        for key in keys {
            let value = self.get(key).unwrap();
            // Nix attribute names can be unquoted if simple identifiers, but quoting them is always safe.
            parts.push(format!(
                "  {} = {};",
                escape_nix_string(key),
                value.to_nix()
            ));
        }
        parts.push("}".to_string());
        parts.join("\n")
    }
}

pub fn format_string_list(list: &[String]) -> String {
    if list.is_empty() {
        return "[]".to_string();
    }
    // Items in the list are Nix strings, so escape each string.
    let items: Vec<String> = list.iter().map(|s| escape_nix_string(s)).collect();
    format!("[ {} ]", items.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockToNix(String);
    impl ToNix for MockToNix {
        fn to_nix(&self) -> String {
            self.0.clone()
        }
    }

    #[test]
    fn test_escape_nix_string() {
        assert_eq!(escape_nix_string("foo"), "\"foo\"");
        assert_eq!(escape_nix_string("foo \"bar\""), "\"foo \\\"bar\\\"\"");
        assert_eq!(escape_nix_string("foo \\ bar"), "\"foo \\\\ bar\"");
    }

    #[test]
    fn test_hashmap_to_nix() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), MockToNix("val1".to_string()));
        map.insert("key with spaces".to_string(), MockToNix("val2".to_string()));

        // This fails if keys are sorted differently. Let's check for containment.
        let nix = map.to_nix();

        // We expect:
        // {
        //   "key1" = val1;
        //   "key with spaces" = val2;
        // }

        // Check parts
        assert!(nix.contains("\"key1\" = val1;"));
        assert!(nix.contains("\"key with spaces\" = val2;"));
    }

    #[test]
    fn test_format_string_list() {
        let list = vec!["foo".to_string(), "bar \"baz\"".to_string()];
        let nix = format_string_list(&list);
        // The list elements should be properly quoted and space separated
        assert_eq!(nix, "[ \"foo\" \"bar \\\"baz\\\"\" ]");
    }
}
