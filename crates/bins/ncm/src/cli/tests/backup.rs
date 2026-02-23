use super::*;

proptest! {
    #[test]
    fn test_backup_proptest(args in prop_oneof![
        // Case 1: --all
        arb_cli_args(
            vec!["ncm".to_string(), "backup".to_string()],
            vec!["--target"],
            vec!["--all"],
            false
        ),
        // Case 2: specific targets
        arb_cli_args(
            vec!["ncm".to_string(), "backup".to_string()],
            vec!["--service", "--node", "--target"],
            vec![],
            false
        ),
    ]) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_backup_poisoned(args in arb_cli_args(
        vec!["ncm".to_string(), "backup".to_string()],
        vec!["--service", "--node", "--target"],
        vec!["--all"],
        true
    )) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
