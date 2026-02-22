# 02 - Inventory Specification

This document details the structure and content of the `ncm` inventory, specifically the Nix attribute sets used to define users, machines, services, secrets, and deployment configuration.

## Directory Structure

The inventory must contain the following directories:

-   `users/`
-   `machines/`
-   `services/`
-   `secrets/`
-   `deploy/`

## Nix Attribute Sets

### 1. Users (`users/`)

Defines user configurations. Each file in this directory represents a user or a group of users.

**Example Structure:**

```nix
{
  myUser = {
    # NixOS user configuration
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    # ...
  };
}
```

### 2. Machines (`machines/`)

Defines the machines in the cluster. Machines link to login users and services via string references (names).

**Attributes:**

-   `users`: A list of strings referencing keys in the `users` attribute set.
-   `services`: A list of strings referencing keys in the `services` attribute set.
-   `system`: The system architecture (e.g., "x86_64-linux").
-   `deployment`: Specific deployment settings (e.g., target host, SSH options).

**Example Structure:**

```nix
{
  "web-server-01" = {
    system = "x86_64-linux";
    users = [ "adminUser" "deployUser" ];
    services = [ "nginx-service" "database-service" ];
    deployment = {
      targetHost = "192.168.1.10";
    };
  };
}
```

### 3. Services (`services/`)

Defines the services running on the cluster. Services can have dependencies and secrets.

**Attributes:**

-   `enable`: Boolean to enable/disable the service.
-   `replicas`: Number of desired replicas (for orchestration purposes).
-   `stateDirs`: List of strings representing stateful directories that need persistence.
-   `backupCmd`: Shell command string to backup the service state.
-   `restoreCmd`: Shell command string to restore the service state.
-   `dependsOn`: List of strings referencing other services this service depends on.
-   `secrets`: List of strings referencing keys in the `secrets` attribute set.
-   `networking`: Implicitly assumes Tailscale integration.

**Example Structure:**

```nix
{
  "database-service" = {
    enable = true;
    replicas = 1;
    stateDirs = [ "/var/lib/postgresql" ];
    backupCmd = "pg_dumpall > /backup/db.sql";
    restoreCmd = "psql < /backup/db.sql";
    dependsOn = [];
    secrets = [ "db-password" ];
    # Service implementation...
  };
}
```

### 4. Secrets (`secrets/`)

Manages secrets using `agenix`. This directory contains the secret definitions and rekeying rules.

**Attributes:**

-   `rekey`: Configuration for who can rekey secrets (referencing users/keys in `deploy`).
-   `file`: Path to the encrypted `.age` file.

**Example Structure:**

```nix
{
  "db-password" = {
    file = ./secrets/db-password.age;
    rekey = {
      # ... rekeying rules
    };
  };
}
```

### 5. Deploy (`deploy/`)

Contains SSH keys and deployment-specific configurations.

**Attributes:**

-   `sshKeys`: List of SSH public keys authorized to deploy to machines.
-   `rekeyKeys`: List of SSH public keys authorized to rekey secrets.

**Example Structure:**

```nix
{
  sshKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
  ];
  rekeyKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
  ];
}
```

## Verification

The `ncm` CLI `validate` command will verify that all string references (e.g., in `machines` -> `users`, `services` -> `secrets`) point to valid, existing definitions in the corresponding attribute sets.
