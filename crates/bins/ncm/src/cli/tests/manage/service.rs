use super::super::*;

#[test]
fn test_manage_service_create() {
    let cmd_args = ["ncm", "manage", "service", "create", "new-service"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_service_list() {
    let cmd_args = ["ncm", "manage", "service", "list"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_manage_service_remove() {
    let cmd_args = ["ncm", "manage", "service", "remove", "old-service"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
