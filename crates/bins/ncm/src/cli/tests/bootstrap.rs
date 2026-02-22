use super::*;

#[test]
fn test_bootstrap_0_machines_fails() {
    let cmd_args = ["bootstrap"];
    let cli = Cli::try_parse_from(cmd_args);
    assert!(cli.is_err());
}

#[test]
fn test_bootstrap_1_machine() {
    let cmd_args = ["ncm", "bootstrap", "machine1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_bootstrap_2_machines() {
    let cmd_args = ["ncm", "bootstrap", "machine1", "machine2"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}

#[test]
fn test_bootstrap_with_global_flags() {
    let cmd_args = ["ncm", "--no-input", "--no-tui", "bootstrap", "machine1"];
    assert!(Cli::try_parse_from(cmd_args).is_ok());
}
