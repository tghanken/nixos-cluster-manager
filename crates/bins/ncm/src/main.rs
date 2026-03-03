use ncm::{
    cli::{Cli, handle},
    init_tracing,
};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let cli = Cli::parse();

    tokio::select! {
        _ = handle(cli) => {
            Ok(())
        }
        _ = ncm::termination_handler() => {
            Ok(())
        }
    }
}
