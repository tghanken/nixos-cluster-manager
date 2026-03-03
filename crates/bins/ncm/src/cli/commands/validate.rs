use std::path::PathBuf;

pub async fn handle(path: PathBuf, no_input: bool, no_tui: bool) -> anyhow::Result<()> {
    todo!(
        "Validate inventory at {:?} (no_input: {}, no_tui: {})",
        path,
        no_input,
        no_tui
    )
}
