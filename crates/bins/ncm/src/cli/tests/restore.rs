use super::*;

#[test]
fn test_restore_all_default() {
    let cmd_args = ["ncm", "restore"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_restore_services_1_2() {
    // 1 service
    let cmd_args = ["ncm", "restore", "--service", "svc1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 services
    let cmd_args = ["ncm", "restore", "--service", "svc1", "--service", "svc2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_restore_nodes_1_2() {
    // 1 node
    let cmd_args = ["ncm", "restore", "--node", "node1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 nodes
    let cmd_args = ["ncm", "restore", "--node", "node1", "--node", "node2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_restore_target() {
    let cmd_args = ["ncm", "restore", "--target", "backup-source"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_restore_combinations() {
    let cmd_args = [
        "ncm",
        "--no-input",
        "restore",
        "--service",
        "svc1",
        "--node",
        "node1",
        "--target",
        "local",
    ];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
