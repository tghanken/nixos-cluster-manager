# 03 - CLI Specification

This document details the architecture, technical stack, and command structure of the `ncm` CLI.

## Technical Stack

The CLI will be written in **Rust** and utilize the following crates:

-   **Runtime**: `tokio` (for asynchronous operations).
-   **CLI Framework**: `clap` (for argument parsing and command structure).
-   **TUI Framework**: `ratatui` (for interactive terminal user interfaces, optional for some commands).
-   **Error Handling**: `anyhow` and `thiserror` (for robust error management).
-   **Logging**: `tracing` (for structured logging).

## Architecture

The `ncm` CLI acts as a wrapper around existing tools, primarily `nixos-rebuild` and `ssh`. It orchestrates these tools to perform cluster management tasks based on the declarative configuration in the inventory.

## Commands

### 1. `validate`

**Description**: Validates the integrity of the inventory.

**Behavior**:
-   Parses all Nix files in the inventory.
-   Verifies that all string references (e.g., machine -> user links) resolve to existing definitions.
-   Checks for circular dependencies in services.
-   Ensures required attributes (e.g., `stateDirs`, `backupCmd`) are present where necessary.

**Usage**: `ncm validate [path/to/inventory]`

### 2. `bootstrap`

**Description**: Bootstraps a new node or the entire cluster.

**Behavior**:
-   Can be interactive (TUI) or non-interactive.
-   Provisions the initial NixOS installation on target machines (e.g., via `nixos-anywhere` or manual SSH bootstrap).
-   Sets up initial SSH keys and secrets.

**Usage**: `ncm bootstrap <target-machine> [options]`

### 3. `deploy`

**Description**: Deploys configurations to machines.

**Behavior**:
-   Can target a single machine (`--node <name>`) or the entire cluster (`--all`).
-   Wraps `nixos-rebuild switch` (or similar deployment command) over SSH.
-   Supports rollback on failure.
-   Optional TUI to visualize deployment progress across multiple nodes.

**Usage**: `ncm deploy [--node <name> | --all] [options]`

### 4. `backup`

**Description**: runs backup commands for services.

**Behavior**:
-   Can target a specific service (`--service <name>`) or all services on a machine/cluster.
-   Executes the `backupCmd` defined in the service configuration.
-   Can stream output or save to a local/remote location.

**Usage**: `ncm backup [--service <name> | --node <name> | --all] [options]`

### 5. `restore`

**Description**: Restores service state from backups.

**Behavior**:
-   Can target a specific service (`--service <name>`) or all services on a machine/cluster.
-   Executes the `restoreCmd` defined in the service configuration.
-   Requires confirmation (interactive) or a `--force` flag.

**Usage**: `ncm restore [--service <name> | --node <name> | --all] [options]`

### 6. `manage`

**Description**: Helper commands to manage inventory objects.

**Subcommands**:
-   `create machine <name>`: Scaffolds a new machine definition.
-   `list machines`: Lists all machines in the inventory.
-   `list services`: Lists all services and their status (enabled/disabled).
-   `rekey`: Triggers a secret rekey using `agenix` and the keys in `deploy/`.

**Usage**: `ncm manage <subcommand> [args]`
