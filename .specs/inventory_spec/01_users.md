# Inventory Specification: Users

Defines user configurations. Each file in this directory represents a user or a group of users.

Files in this directory must define an attribute set where each key is the name of the user being defined.

**Attributes:**

-   `isNormalUser`: (Required, Boolean) Whether the user is a normal user.
-   `extraGroups`: (Optional, List of Strings) Additional groups for the user.
-   `sshKeys`: (Optional, List of Strings) SSH public keys for the user.

**Example Structure:**

```nix
{
  myUser = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    sshKeys = [ "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..." ];
  };
}
```
