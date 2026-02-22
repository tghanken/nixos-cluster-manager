use super::*;

#[test]
fn test_backup_all_default() {
    let cmd_args = ["ncm", "backup"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_backup_services_1_2() {
    // 1 service
    let cmd_args = ["ncm", "backup", "--service", "svc1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 services
    let cmd_args = ["ncm", "backup", "--service", "svc1", "--service", "svc2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_backup_nodes_1_2() {
    // 1 node
    let cmd_args = ["ncm", "backup", "--node", "node1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 nodes
    let cmd_args = ["ncm", "backup", "--node", "node1", "--node", "node2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_backup_target() {
    let cmd_args = ["ncm", "backup", "--target", "remote-target"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_backup_combinations() {
    let cmd_args = [
        "ncm",
        "--no-tui",
        "backup",
        "--service",
        "svc1",
        "--node",
        "node1",
        "--target",
        "s3",
    ];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
