# Inventory Specification: Secrets

Manages secrets using `agenix`. This directory contains the secret definitions.

**Attributes:**

-   `file`: (Required, Path) Path to the encrypted `.age` file.

**Note**: Rekeying rules are derived from the users and deploy keys specified in the inventory. All deploy keys are implicitly considered rekey keys.

**Example Structure:**

```nix
{
  "db-password" = {
    file = ./secrets/db-password.age;
  };
}
```
