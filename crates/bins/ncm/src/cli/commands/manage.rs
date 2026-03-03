use crate::cli::ManageCommands;

pub async fn handle(action: ManageCommands) -> anyhow::Result<()> {
    match action {
        ManageCommands::Machine { action } => todo!("Manage machine: {:?}", action),
        ManageCommands::Service { action } => todo!("Manage service: {:?}", action),
        ManageCommands::User { action } => todo!("Manage user: {:?}", action),
        ManageCommands::Secret { action } => todo!("Manage secret: {:?}", action),
    }
}
