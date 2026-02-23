use super::super::*;

proptest! {
    #[test]
    fn test_manage_user_proptest(
        args in prop_oneof![
            arb_manage_args("user", "create", ManageNameMode::Required, false),
            arb_manage_args("user", "list", ManageNameMode::None, false),
            arb_manage_args("user", "remove", ManageNameMode::Required, false),
        ]
    ) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_manage_user_poisoned(
        args in arb_manage_args("user", "create", ManageNameMode::Required, true)
    ) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
