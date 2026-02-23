use super::*;

proptest! {
    #[test]
    fn test_deploy_proptest(args in prop_oneof![
        // Case 1: --all
        arb_cli_args(
            vec!["ncm".to_string(), "deploy".to_string()],
            vec!["--exclude"],
            vec!["--all", "--accept-disk-changes"],
            false
        ),
        // Case 2: --node
        arb_cli_args(
            vec!["ncm".to_string(), "deploy".to_string()],
            vec!["--node", "--exclude"],
            vec!["--accept-disk-changes"],
            false
        ),
    ]) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_deploy_poisoned(args in arb_cli_args(
        vec!["ncm".to_string(), "deploy".to_string()],
        vec!["--node", "--exclude"],
        vec!["--all", "--accept-disk-changes"],
        true
    )) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
