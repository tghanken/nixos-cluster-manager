use super::*;

proptest! {
    #[test]
    fn test_restore_proptest(args in prop_oneof![
        // Case 1: --all
        arb_cli_args(
            vec!["ncm".to_string(), "restore".to_string()],
            vec!["--target"],
            vec!["--all", "--force"],
            false
        ),
        // Case 2: specific targets
        arb_cli_args(
            vec!["ncm".to_string(), "restore".to_string()],
            vec!["--service", "--node", "--target"],
            vec!["--force"],
            false
        ),
    ]) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_restore_poisoned(args in arb_cli_args(
        vec!["ncm".to_string(), "restore".to_string()],
        vec!["--service", "--node", "--target"],
        vec!["--all", "--force"],
        true
    )) {
        // If it was poisoned with an unknown flag, it should fail
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
