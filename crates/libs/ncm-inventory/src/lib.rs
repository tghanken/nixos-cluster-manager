pub mod parser;
pub mod types;

pub use parser::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::env;

    fn get_nix_dir() -> PathBuf {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let path = PathBuf::from(manifest_dir).join("nix");
        if !path.exists() {
            fs::create_dir_all(&path).expect("Failed to create nix directory");
        }
        path
    }

    #[test]
    fn test_user_inventory() -> anyhow::Result<()> {
        let content = r#"
        {
          "test-user" = {
            sshKeys = [ "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ..." ];
          };
        }
        "#;

        let nix_dir = get_nix_dir();
        let path = nix_dir.join("test_user_inventory.nix");
        fs::write(&path, content)?;

        let users: HashMap<String, User> = parse_nix_file(&path)?;

        assert!(users.contains_key("test-user"));
        let user = &users["test-user"];
        assert_eq!(user.ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...");

        // Serialize back
        let output = users.to_nix();
        let out_path = nix_dir.join("test_user_inventory_out.nix");
        fs::write(&out_path, &output)?;

        // Verify re-parsing
        let users2: HashMap<String, User> = parse_nix_file(&out_path)?;
        assert_eq!(users, users2);

        // Cleanup
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(out_path);
        Ok(())
    }

    #[test]
    fn test_machine_inventory() -> anyhow::Result<()> {
        let content = r#"
        {
          "web-server-01" = {
            sshKeys = [ "ssh-ed25519 ..." ];
            users = [ "adminUser" "deployUser" ];
            node_services = [ "nginx-service" "database-service" ];
            bootDisks = [ "/dev/sda" "/dev/sdb" ];
            deployment = {
              ipv4 = "192.168.1.10";
              ipv6 = "2001:db8::1";
            };
          };
        }
        "#;

        let nix_dir = get_nix_dir();
        let path = nix_dir.join("test_machine_inventory.nix");
        fs::write(&path, content)?;

        let machines: HashMap<String, Machine> = parse_nix_file(&path)?;

        assert!(machines.contains_key("web-server-01"));
        let machine = &machines["web-server-01"];
        assert_eq!(machine.ssh_keys[0], "ssh-ed25519 ...");
        assert_eq!(machine.users.len(), 2);
        assert_eq!(machine.deployment.ipv4, Some("192.168.1.10".to_string()));
        assert_eq!(machine.deployment.ipv6, Some("2001:db8::1".to_string()));

        // Serialize back
        let output = machines.to_nix();
        let out_path = nix_dir.join("test_machine_inventory_out.nix");
        fs::write(&out_path, &output)?;

        // Verify re-parsing
        let machines2: HashMap<String, Machine> = parse_nix_file(&out_path)?;
        assert_eq!(machines, machines2);

        let _ = fs::remove_file(path);
        let _ = fs::remove_file(out_path);
        Ok(())
    }

    #[test]
    fn test_service_inventory() -> anyhow::Result<()> {
        let content = r#"
        {
          "database-service" = {
            enable = true;
            replicas = 3;
            stateDirs = [ "/var/lib/postgresql" ];
            dependsOn = [];
            secrets = [ "db-password" ];
            backups = [
                {
                    name = "local-snap";
                    type = "local";
                    command = "zfs snapshot ...";
                    restore = "zfs restore ...";
                    priority = 10;
                }
            ];
          };
        }
        "#;

        let nix_dir = get_nix_dir();
        let path = nix_dir.join("test_service_inventory.nix");
        fs::write(&path, content)?;

        let services: HashMap<String, Service> = parse_nix_file(&path)?;

        assert!(services.contains_key("database-service"));
        let service = &services["database-service"];
        assert!(service.enable);
        assert_eq!(service.replicas, 3);
        assert_eq!(service.state_dirs[0], "/var/lib/postgresql");
        assert_eq!(service.backups.len(), 1);
        assert_eq!(service.backups[0].priority, Some(10));

        // Serialize back
        let output = services.to_nix();
        let out_path = nix_dir.join("test_service_inventory_out.nix");
        fs::write(&out_path, &output)?;

        // Verify re-parsing
        let services2: HashMap<String, Service> = parse_nix_file(&out_path)?;
        assert_eq!(services, services2);

        let _ = fs::remove_file(path);
        let _ = fs::remove_file(out_path);
        Ok(())
    }
}
