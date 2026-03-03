use crate::cli::{Cli, Commands};

mod backup;
mod bootstrap;
mod deploy;
mod manage;
mod restore;
mod validate;

pub async fn handle(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Validate => validate::handle(cli.inventory, cli.no_input, cli.no_tui).await?,
        Commands::Bootstrap { machine } => {
            bootstrap::handle(machine, cli.no_input, cli.no_tui).await?
        }
        Commands::Deploy {
            node,
            exclude,
            all,
            accept_disk_changes,
        } => {
            deploy::handle(
                node,
                exclude,
                all,
                accept_disk_changes,
                cli.no_input,
                cli.no_tui,
            )
            .await?
        }
        Commands::Backup {
            service,
            node,
            all,
            target,
        } => backup::handle(service, node, all, target, cli.no_input, cli.no_tui).await?,
        Commands::Restore {
            service,
            node,
            all,
            target,
            force,
        } => restore::handle(service, node, all, target, force, cli.no_input, cli.no_tui).await?,
        Commands::Manage { action } => manage::handle(action).await?,
    }

    Ok(())
}
