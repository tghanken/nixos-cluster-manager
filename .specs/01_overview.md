# 01 - Overview: NixOS Cluster Manager (ncm)

## Introduction

`ncm` is a CLI tool designed to manage and deploy NixOS configurations to a cluster of machines. It aims to provide a deterministic and declarative way to define the state of a cluster, including users, machines, services, secrets, and deployment keys.

## Goals

1.  **Deterministic Deployment**: Ensure that the cluster state is always consistent with the declared configuration.
2.  **Declarative Configuration**: Define the cluster state using Nix attribute sets.
3.  **Ease of Use**: Provide a CLI and optional TUI for common operations like validation, bootstrapping, deployment, backups, and restoration.
4.  **Security**: Integrate with `agenix` for secret management and assume secure defaults like Tailscale networking.
5.  **Atomic Rollbacks**: Automatically roll back all hosts in the deployment process if any single host fails to deploy.

## Inventory Concept

The core of `ncm` is the **inventory**, a directory containing all the Nix configuration files necessary to define the cluster. The inventory is structured into specific subdirectories, each representing a different aspect of the cluster configuration.

The CLI consumes this inventory to generate the final NixOS configurations and deploy them to the target machines.

## Directory Structure

The inventory must follow a strict directory structure:

-   `users/`: Contains user definitions.
-   `machines/`: Contains machine definitions, linking users and services.
-   `services/`: Contains service definitions, including dependencies, secrets, and state management.
-   `secrets/`: Contains `agenix` secrets and rekeying configurations.
-   `deploy/`: Contains deployment keys and SSH configurations.

Each directory corresponds to a core Nix attribute set that the `ncm` tool understands and processes.
