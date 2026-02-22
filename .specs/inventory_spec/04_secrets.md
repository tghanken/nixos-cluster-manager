# Inventory Specification: Secrets

Manages secrets using `agenix`. This directory contains the secret definitions.

**Attributes:**

-   `rekeyUsers`: (Optional, List of Strings) Additional users authorized to rekey this secret.

**Note**: The file path is derived and not needed as an attribute. All deploy keys and machines where the service is deployed are implicitly considered rekey keys.

**Example Structure:**

```nix
{
  "db-password" = {
    rekeyUsers = [ "admin" ];
  };
}
```
