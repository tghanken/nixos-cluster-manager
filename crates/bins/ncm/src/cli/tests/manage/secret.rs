use super::super::*;

proptest! {
    #[test]
    fn test_manage_secret_proptest(
        args in prop_oneof![
            arb_manage_args("secret", "create", ManageNameMode::Required, false),
            arb_manage_args("secret", "list", ManageNameMode::None, false),
            arb_manage_args("secret", "remove", ManageNameMode::Required, false),
            arb_manage_args("secret", "rekey", ManageNameMode::None, false),
        ]
    ) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_manage_secret_poisoned(
        args in arb_manage_args("secret", "rekey", ManageNameMode::None, true)
    ) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
