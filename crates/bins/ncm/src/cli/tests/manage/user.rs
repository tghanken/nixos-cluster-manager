use super::super::*;

#[test]
fn test_manage_user_create() {
    let cmd_args = ["ncm", "manage", "user", "create", "new-user"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_user_list() {
    let cmd_args = ["ncm", "manage", "user", "list"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_user_remove() {
    let cmd_args = ["ncm", "manage", "user", "remove", "old-user"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
