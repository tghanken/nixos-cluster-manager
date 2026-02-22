# Inventory Specification: Services

Defines the services running on the cluster. Services can have dependencies and secrets.

**Attributes:**

-   `enable`: (Optional, Boolean) Enable/disable the service. Defaults to `true`.
-   `replicas`: (Optional, Int) Number of desired replicas (for orchestration purposes). Defaults to 1.
-   `stateDirs`: (Optional, List of Strings) List of strings representing stateful directories that need persistence.
-   `dependsOn`: (Optional, List of Strings) List of strings referencing other services this service depends on.
-   `secrets`: (Optional, List of Strings) List of strings referencing keys in the `secrets` attribute set.
-   `backups`: (Optional, List of Attribute Sets) Backup strategies.
    -   `name`: (Required, String) The name of the backup strategy (for targeting via CLI).
    -   `type`: (Required, String) "local" or "remote".
    -   `command`: (Required, String) The backup command to run.
    -   `restore`: (Required, String) The restore command to run.
    -   `priority`: (Optional, Int) Higher numbers take precedence.
    -   *Defaults*:
        -   Local: ZFS snapshot of the disk.
        -   Remote: Restic backup to remote object storage.

**Example Structure:**

```nix
{
  "database-service" = {
    enable = true;
    replicas = 1;
    stateDirs = [ "/var/lib/postgresql" ];
    dependsOn = [];
    secrets = [ "db-password" ];
    backups = [
        {
            type = "local";
            command = "zfs snapshot ...";
            restore = "zfs restore ...";
            priority = 10;
        }
        {
            type = "remote";
            command = "restic backup ...";
            restore = "restic restore ...";
            priority = 1;
        }
    ];
  };
}
```
