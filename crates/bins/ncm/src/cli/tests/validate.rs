use super::*;

proptest! {
    #[test]
    fn test_validate_proptest(args in arb_cli_args(
        vec!["ncm".to_string(), "validate".to_string()],
        vec![],
        vec![],
        false
    )) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_validate_poisoned(args in arb_cli_args(
        vec!["ncm".to_string(), "validate".to_string()],
        vec![],
        vec![],
        true
    )) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
