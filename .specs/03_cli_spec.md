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
-   Ensures required attributes (e.g., `stateDirs`, `backups`) are present where necessary.
-   **Warning**: Issues a warning if any services are disabled (`enable = false`) but are still assigned to a machine (either manually via `node_services` or automatically).

**Usage**: `ncm validate [path/to/inventory]`

### 2. `bootstrap`

**Description**: Bootstraps a new node or the entire cluster.

**Steps**:
1.  **Clear Known Hosts**: Clear known hosts for the destination IP(s).
2.  **Generate Facter Report**: Generate a NixOS facter report for the remote host.
3.  **Disk Setup Prompt**: If multiple disks are present and no configuration is specified, prompt the user to select a boot disk and/or RAID level (mirror, ZFS RAIDZ, etc.).
4.  **Run NixOS Anywhere**: Provision the machine using `nixos-anywhere`.
5.  **Clear Known Hosts**: Clear known hosts again.
6.  **Add Host Key**: Retrieve the host SSH key and add it to the machine manifest.
7.  **Rekey Secrets**: Rekey all agenix secrets required by the machine.
8.  **Run Deploy Script**: Execute the final deployment script for the host.

**Usage**: `ncm bootstrap <target-machine> [options]`

### 3. `deploy`

**Description**: Deploys configurations to machines.

**Behavior**:
-   Can target a single machine (`--node <name>`) or multiple machines (multiple flags or exclusion).
-   Wraps `nixos-rebuild switch` (or similar deployment command) over SSH.
-   Supports rollback on failure.
-   Optional TUI to visualize deployment progress across multiple nodes.

**Steps**:
1.  **Validate**: Run the `validate` command.
2.  **Hash Services**: Hash each service definition from the current commit.
3.  **Check Remote State**: SSH into each remote, determine the hash of running services, and store the current NixOS generation number (for rollback).
4.  **Build Host List**: Identify hosts that need updates (service hash mismatch).
5.  **Create DAG**: Create a dependency graph for deployment order.
    -   **Rule 1**: Max 1 replica disrupted at a time.
    -   **Rule 2**: Deploy dependency before dependent.
6.  **Deploy in Order**: Deploy to hosts in DAG order (parallel where possible).
    -   **Drain**: Drain Tailscale services (via systemd one-shot).
    -   **Remote Backup**: If stateful, run remote backup.
    -   **NixOS Rebuild**: Run `nixos-rebuild switch`.
    -   **Serve**: Serve Tailscale services again.
    -   **On Failure**: Rollback host to previous generation, trigger full cluster rollback to previous state, cancel pending deployments. If stateful, restore from backup.

**Usage**: `ncm deploy [--node <name> | --exclude <name> | --all] [options]`

### 4. `backup`

**Description**: Runs backup commands for services.

**Behavior**:
-   Can target a specific service (`--service <name>`) or all services on a machine/cluster.
-   Executes the backup commands defined in the `backups` list attribute of the service configuration.
-   Can stream output or save to a local/remote location.
-   **Target**: specific backup target (local/remote) can be specified. If not specified, the highest priority backup strategy is used.

**Usage**: `ncm backup [--service <name> | --node <name> | --all] [--target <local|remote>] [options]`

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
-   `machine create <name>`: Scaffolds a new machine definition.
-   `machine list`: Lists all machines in the inventory.
-   `machine remove <name>`: Removes a machine definition.
-   `service create <name>`: Scaffolds a new service definition.
-   `service list`: Lists all services and their status.
-   `service remove <name>`: Removes a service definition.
-   `user create <name>`: Scaffolds a new user definition.
-   `user list`: Lists all users.
-   `user remove <name>`: Removes a user definition.
-   `secret create <name>`: Scaffolds a new secret.
-   `secret list`: Lists all secrets.
-   `secret remove <name>`: Removes a secret.
-   `secret rekey`: Triggers a secret rekey using `agenix` and the keys in `deploy/`.

**Usage**: `ncm manage <object> <action> [args]`
