pub mod commands;
pub use commands::handle;

#[cfg(test)]
mod tests;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ncm")]
#[command(about = "NixOS Cluster Manager", long_about = None)]
pub struct Cli {
    /// Path to the inventory directory
    #[arg(short, long, env = "NCM_INVENTORY", default_value = "./inventory")]
    pub inventory: PathBuf,
    /// Do not prompt for input
    #[arg(long)]
    pub no_input: bool,
    /// Do not use TUI
    #[arg(long)]
    pub no_tui: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate the integrity of the inventory
    Validate,
    /// Bootstrap a new node or the entire cluster
    Bootstrap {
        /// Target machine to bootstrap
        machine: Vec<String>,
    },
    /// Deploy configurations to machines
    Deploy {
        /// Target a single machine
        #[arg(long, short)]
        node: Option<Vec<String>>,
        /// Exclude a machine from deployment
        #[arg(long, short)]
        exclude: Option<Vec<String>>,
        /// Deploy to all machines
        #[arg(long, short, default_value_t = true, conflicts_with = "node")]
        all: bool,
        /// Accept disk configuration changes
        #[arg(long)]
        accept_disk_changes: bool,
    },
    /// Run backup commands for services
    Backup {
        /// Target a specific service
        #[arg(long, short)]
        service: Option<Vec<String>>,
        /// Target all services on a machine
        #[arg(long, short)]
        node: Option<Vec<String>>,
        /// Target all services in the cluster
        #[arg(
            long,
            short,
            default_value_t = true,
            conflicts_with = "node",
            conflicts_with = "service"
        )]
        all: bool,
        /// Named backup target from the inventory
        #[arg(long, short)]
        target: Option<String>,
    },
    /// Restore service state from backups
    Restore {
        /// Target a specific service
        #[arg(long, short)]
        service: Option<Vec<String>>,
        /// Target all services on a machine
        #[arg(long, short)]
        node: Option<Vec<String>>,
        /// Target all services in the cluster
        #[arg(
            long,
            short,
            default_value_t = true,
            conflicts_with = "node",
            conflicts_with = "service"
        )]
        all: bool,
        /// Named backup target from the inventory
        #[arg(long, short)]
        target: Option<String>,
    },
    /// Helper commands to manage inventory objects
    Manage {
        #[command(subcommand)]
        action: ManageCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ManageCommands {
    /// Manage machines
    Machine {
        #[command(subcommand)]
        action: ObjectAction,
    },
    /// Manage services
    Service {
        #[command(subcommand)]
        action: ObjectAction,
    },
    /// Manage users
    User {
        #[command(subcommand)]
        action: ObjectAction,
    },
    /// Manage secrets
    Secret {
        #[command(subcommand)]
        action: SecretAction,
    },
}

#[derive(Subcommand)]
pub enum ObjectAction {
    /// Create a new definition
    Create { name: String },
    /// List all definitions
    List,
    /// Remove a definition
    Remove { name: String },
}

#[derive(Subcommand)]
pub enum SecretAction {
    /// Create a new secret
    Create { name: String },
    /// List all secrets
    List,
    /// Remove a secret
    Remove { name: String },
    /// Trigger a secret rekey using agenix
    Rekey,
}

impl std::fmt::Debug for ObjectAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create { name } => write!(f, "Create({})", name),
            Self::List => write!(f, "List"),
            Self::Remove { name } => write!(f, "Remove({})", name),
        }
    }
}

impl std::fmt::Debug for SecretAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create { name } => write!(f, "Create({})", name),
            Self::List => write!(f, "List"),
            Self::Remove { name } => write!(f, "Remove({})", name),
            Self::Rekey => write!(f, "Rekey"),
        }
    }
}
