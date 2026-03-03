use super::*;
use proptest::prelude::*;

mod backup;
mod bootstrap;
mod deploy;
mod manage;
mod restore;
mod validate;

pub fn arb_global_flags() -> impl Strategy<Value = Vec<String>> {
    let global_flags = vec!["--no-input", "--no-tui"];
    prop::sample::subsequence(global_flags, 0..=2)
        .prop_map(|flags| flags.into_iter().map(|s| s.to_string()).collect())
}

pub fn arb_boolean_flags(flags: Vec<&'static str>) -> impl Strategy<Value = Vec<String>> {
    if flags.is_empty() {
        Just(vec![]).boxed()
    } else {
        let len = flags.len();
        prop::sample::subsequence(flags, 0..=len)
            .prop_map(|f| f.into_iter().map(|s| s.to_string()).collect())
            .boxed()
    }
}

pub fn arb_value_flags(flags: Vec<&'static str>) -> impl Strategy<Value = Vec<String>> {
    if flags.is_empty() {
        Just(vec![]).boxed()
    } else {
        prop::collection::vec(
            (
                prop::sample::select(flags),
                "[a-zA-Z0-9._][a-zA-Z0-9._-]{0,20}",
            ),
            0..5,
        )
        .prop_map(|pairs| {
            let mut args = Vec::new();
            let non_repeatable = ["--target"];
            let mut seen = std::collections::HashSet::new();

            for (flag, val) in pairs {
                if non_repeatable.contains(&flag) {
                    if seen.contains(&flag) {
                        continue;
                    }
                    seen.insert(flag);
                }
                args.push(flag.to_string());
                args.push(val);
            }
            args
        })
        .boxed()
    }
}

pub fn arb_inventory_flag() -> impl Strategy<Value = Vec<String>> {
    prop::option::weighted(
        0.5,
        (Just("--inventory"), "[a-zA-Z0-9/_][a-zA-Z0-9/_.-]{0,30}"),
    )
    .prop_map(|opt| {
        if let Some((flag, val)) = opt {
            vec![flag.to_string(), val.to_string()]
        } else {
            vec![]
        }
    })
}

/// Generates a random vector of CLI arguments.
///
/// * `base_args`: The required base arguments (e.g., ["ncm", "restore"])
/// * `value_flags`: Flags that take a value (e.g., ["--service", "--node"])
/// * `boolean_flags`: Flags that do NOT take a value (e.g., ["--all"])
/// * `poison`: If true, an unknown flag might be added.
pub fn arb_cli_args(
    base_args: Vec<String>,
    value_flags: Vec<&'static str>,
    boolean_flags: Vec<&'static str>,
    poison: bool,
) -> impl Strategy<Value = Vec<String>> {
    (
        arb_global_flags(),
        arb_inventory_flag(),
        arb_boolean_flags(boolean_flags),
        arb_value_flags(value_flags),
    )
        .prop_map(move |(globals, inventory, booleans, values)| {
            let mut args = vec![base_args[0].clone()];
            args.extend(globals);
            args.extend(inventory);
            args.extend_from_slice(&base_args[1..]);
            args.extend(booleans);
            args.extend(values);

            if poison {
                args.push("--unknown-flag-123".to_string());
            }

            args
        })
}

#[derive(Clone, Copy)]
pub enum ManageNameMode {
    None,
    Required,
    #[allow(dead_code)]
    Optional,
}

/// Generates random arguments for `manage` subcommands.
pub fn arb_manage_args(
    object: &'static str,
    action: &'static str,
    name_mode: ManageNameMode,
    poison: bool,
) -> impl Strategy<Value = Vec<String>> {
    let name_strategy = match name_mode {
        ManageNameMode::None => Just(None).boxed(),
        ManageNameMode::Required => "[a-zA-Z0-9._][a-zA-Z0-9._-]{0,20}".prop_map(Some).boxed(),
        ManageNameMode::Optional => {
            prop::option::weighted(0.5, "[a-zA-Z0-9._][a-zA-Z0-9._-]{0,20}").boxed()
        }
    };

    (
        arb_cli_args(
            vec![
                "ncm".to_string(),
                "manage".to_string(),
                object.to_string(),
                action.to_string(),
            ],
            vec![],
            vec![],
            poison,
        ),
        name_strategy,
    )
        .prop_map(|(mut args, name)| {
            if let Some(n) = name {
                args.push(n);
            }
            args
        })
}
