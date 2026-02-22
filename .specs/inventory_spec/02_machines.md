# Inventory Specification: Machines

Defines the machines in the cluster. Machines link to login users and services via string references (names).

**Attributes:**

-   `sshKeys`: (Optional, List of Strings) List of SSH public keys for accessing the machine.
-   `users`: (Optional, List of Strings) References to keys in the `users` attribute set.
-   `node_services`: (Optional, List of Strings) Services that **MUST** run on this specific machine (manual assignment).
-   `bootDisks`: (Optional, List of Strings) List of disk devices to be used for the root filesystem. If multiple disks are provided, they will be configured as a mirrored boot pool (ZFS).
-   `deployment`: (Required, Attribute Set) Specific deployment settings.
    -   `ipv4`: (Optional, String) The IPv4 address for initial bootstrap.
    -   `ipv6`: (Optional, String) The IPv6 address for initial bootstrap.
    -   *Note*: At least one of `ipv4` or `ipv6` must be provided for bootstrap. For normal deployments, Tailscale SSH is assumed.

**Note**: The list of all services running on a machine is derived automatically from `node_services` and the cluster scheduler (for auto-assigned services). It is not defined as a static attribute here. The system architecture is also derived.

**Example Structure:**

```nix
{
  "web-server-01" = {
    sshKeys = [ "ssh-ed25519 ..." ];
    users = [ "adminUser" "deployUser" ];
    node_services = [ "nginx-service" "database-service" ];
    deployment = {
      ipv4 = "192.168.1.10";
    };
  };
}
```
