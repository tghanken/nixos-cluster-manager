# Inventory Specification: Machines

Defines the machines in the cluster. Machines link to login users and services via string references (names).

**Attributes:**

-   `system`: (Required, String) The system architecture (e.g., "x86_64-linux").
-   `users`: (Optional, List of Strings) References to keys in the `users` attribute set.
-   `node_services`: (Optional, List of Strings) Services that **MUST** run on this specific machine (manual assignment).
-   `disks`: (Optional, List of Strings) List of disk devices to be used for the root filesystem. If multiple disks are provided, they will be configured as a mirrored boot pool (ZFS).
-   `deployment`: (Required, Attribute Set) Specific deployment settings.
    -   `targetHost`: (Required, String) The IP address or hostname for initial bootstrap (IPv4 or IPv6). For normal deployments, Tailscale SSH is assumed.

**Note**: The list of all services running on a machine is derived automatically from `node_services` and the cluster scheduler (for auto-assigned services). It is not defined as a static attribute here.

**Example Structure:**

```nix
{
  "web-server-01" = {
    system = "x86_64-linux";
    users = [ "adminUser" "deployUser" ];
    node_services = [ "nginx-service" "database-service" ];
    deployment = {
      targetHost = "192.168.1.10"; # Required for bootstrap
    };
  };
}
```
