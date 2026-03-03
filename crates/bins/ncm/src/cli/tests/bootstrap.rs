use super::*;

// For bootstrap, we need to generate at least one positional machine name.
fn arb_bootstrap_args(poison: bool) -> impl Strategy<Value = Vec<String>> {
    (
        arb_cli_args(
            vec!["ncm".to_string(), "bootstrap".to_string()],
            vec![],
            vec![],
            poison,
        ),
        prop::collection::vec("[a-zA-Z0-9._][a-zA-Z0-9._-]{0,20}", 1..5),
    )
        .prop_map(|(mut args, machines)| {
            for machine in machines {
                args.push(machine);
            }
            args
        })
}

proptest! {
    #[test]
    fn test_bootstrap_proptest(args in arb_bootstrap_args(false)) {
        prop_assert!(Cli::try_parse_from(args).is_ok());
    }

    #[test]
    fn test_bootstrap_poisoned(args in arb_bootstrap_args(true)) {
        prop_assert!(Cli::try_parse_from(args).is_err());
    }
}
