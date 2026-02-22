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

Please see the following documents for detailed specifications of each inventory type:

1.  [Users](inventory_spec/01_users.md)
2.  [Machines](inventory_spec/02_machines.md)
3.  [Services](inventory_spec/03_services.md)
4.  [Secrets](inventory_spec/04_secrets.md)
5.  [Deploy](inventory_spec/05_deploy.md)
