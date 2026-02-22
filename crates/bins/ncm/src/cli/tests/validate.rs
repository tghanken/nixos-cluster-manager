use super::*;

#[test]
fn test_validate_default() {
    let cmd_args = ["ncm", "validate"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_validate_with_inventory() {
    let cmd_args = ["ncm", "--inventory", "custom/path", "validate"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_validate_with_global_flags() {
    let cmd_args = ["ncm", "--no-input", "--no-tui", "validate"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
