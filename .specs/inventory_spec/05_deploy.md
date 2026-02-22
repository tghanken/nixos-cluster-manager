# Inventory Specification: Deploy

Contains SSH keys and deployment-specific configurations.

**Attributes:**

-   `sshKeys`: (Required, List of Strings) List of SSH public keys authorized to deploy to machines.

**Note**: All `sshKeys` are automatically considered rekey keys.

**Example Structure:**

```nix
{
  sshKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
  ];
}
```
