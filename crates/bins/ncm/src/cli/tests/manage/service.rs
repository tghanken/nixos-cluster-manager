use super::super::*;

proptest! {
    #[test]
    fn test_manage_service_proptest(
        args in prop_oneof![
            arb_manage_args("service", "create", ManageNameMode::Required, false),
            arb_manage_args("service", "list", ManageNameMode::None, false),
            arb_manage_args("service", "remove", ManageNameMode::Required, false),
        ]
    ) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_manage_service_poisoned(
        args in arb_manage_args("service", "create", ManageNameMode::Required, true)
    ) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
