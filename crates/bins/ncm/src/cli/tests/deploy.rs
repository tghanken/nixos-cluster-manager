use super::*;

#[test]
fn test_deploy_all_default() {
    let cmd_args = ["ncm", "deploy"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_deploy_nodes_1_2() {
    // 1 node
    let cmd_args = ["ncm", "deploy", "--node", "node1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 nodes
    let cmd_args = ["ncm", "deploy", "--node", "node1", "--node", "node2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_deploy_exclude_1_2() {
    // 1 exclude
    let cmd_args = ["ncm", "deploy", "--exclude", "node1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());

    // 2 excludes
    let cmd_args = ["ncm", "deploy", "--exclude", "node1", "--exclude", "node2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_deploy_accept_disk_changes() {
    let cmd_args = ["ncm", "deploy", "--accept-disk-changes"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_deploy_combinations() {
    // Cli struct uses #[arg(long)] which defaults to hyphenated-name
    let cmd_args = [
        "ncm",
        "--no-input",
        "deploy",
        "--node",
        "node1",
        "--exclude",
        "node2",
        "--accept-disk-changes",
    ];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
