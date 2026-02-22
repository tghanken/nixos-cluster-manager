# Inventory Specification: Deploy

Contains SSH keys and deployment-specific configurations.

**Attributes:**

-   `sshKeys`: (Required, List of Strings) List of SSH public keys authorized to deploy to machines.
-   `rekeyKeys`: (Optional, List of Strings) Additional users or keys that are authorized to rekey secrets.

**Note**: All `sshKeys` are automatically considered `rekeyKeys`.

**Example Structure:**

```nix
{
  sshKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
  ];
  rekeyKeys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..." # Additional key
  ];
}
```
