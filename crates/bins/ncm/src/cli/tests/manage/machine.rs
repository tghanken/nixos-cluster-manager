use super::super::*;

#[test]
fn test_manage_machine_create() {
    let cmd_args = ["ncm", "manage", "machine", "create", "new-machine"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_machine_list() {
    let cmd_args = ["ncm", "manage", "machine", "list"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_machine_remove() {
    let cmd_args = ["ncm", "manage", "machine", "remove", "old-machine"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
