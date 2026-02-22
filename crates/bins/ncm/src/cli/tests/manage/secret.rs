use super::super::*;

#[test]
fn test_manage_secret_create() {
    let cmd_args = ["ncm", "manage", "secret", "create", "new-secret"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_secret_list() {
    let cmd_args = ["ncm", "manage", "secret", "list"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_secret_remove() {
    let cmd_args = ["ncm", "manage", "secret", "remove", "old-secret"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_secret_rekey() {
    let cmd_args = ["ncm", "manage", "secret", "rekey"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
