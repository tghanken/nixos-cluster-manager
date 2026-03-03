use ncm_inventory::{DeployConfig, Machine, Secret, Service, ToNix, User, parse_nix_file};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_nix_dir() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    PathBuf::from(manifest_dir).join("nix")
}

fn check_nix_present() -> bool {
    std::process::Command::new("nix")
        .arg("--version")
        .status()
        .is_ok()
}

#[test]
fn test_user_inventory() -> anyhow::Result<()> {
    if !check_nix_present() {
        return Ok(());
    }
    let nix_dir = get_nix_dir();
    let path = nix_dir.join("test_user_inventory.nix");

    let users: HashMap<String, User> = parse_nix_file(&path)?;

    assert!(users.contains_key("test-user"));
    let user = &users["test-user"];
    assert_eq!(
        user.ssh_keys[0],
        "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ..."
    );

    // Serialize back
    let output = users.to_nix();
    let out_path = nix_dir.join("test_user_inventory_out.nix");
    fs::write(&out_path, &output)?;

    // Verify re-parsing
    let users2: HashMap<String, User> = parse_nix_file(&out_path)?;
    assert_eq!(users, users2);

    // Cleanup
    let _ = fs::remove_file(out_path);
    Ok(())
}

#[test]
fn test_machine_inventory() -> anyhow::Result<()> {
    if !check_nix_present() {
        return Ok(());
    }
    let nix_dir = get_nix_dir();
    let path = nix_dir.join("test_machine_inventory.nix");

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

    // Cleanup
    let _ = fs::remove_file(out_path);
    Ok(())
}

#[test]
fn test_service_inventory() -> anyhow::Result<()> {
    if !check_nix_present() {
        return Ok(());
    }
    let nix_dir = get_nix_dir();
    let path = nix_dir.join("test_service_inventory.nix");

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

    // Cleanup
    let _ = fs::remove_file(out_path);
    Ok(())
}

#[test]
fn test_secret_inventory() -> anyhow::Result<()> {
    if !check_nix_present() {
        return Ok(());
    }
    let nix_dir = get_nix_dir();
    let path = nix_dir.join("test_secret_inventory.nix");

    let secrets: HashMap<String, Secret> = parse_nix_file(&path)?;

    assert!(secrets.contains_key("db-password"));
    let secret = &secrets["db-password"];
    assert_eq!(secret.rekey_users.len(), 2);
    assert_eq!(secret.rekey_users[0], "admin");

    // Serialize back
    let output = secrets.to_nix();
    let out_path = nix_dir.join("test_secret_inventory_out.nix");
    fs::write(&out_path, &output)?;

    // Verify re-parsing
    let secrets2: HashMap<String, Secret> = parse_nix_file(&out_path)?;
    assert_eq!(secrets, secrets2);

    // Cleanup
    let _ = fs::remove_file(out_path);
    Ok(())
}

#[test]
fn test_deploy_inventory() -> anyhow::Result<()> {
    if !check_nix_present() {
        return Ok(());
    }
    let nix_dir = get_nix_dir();
    let path = nix_dir.join("test_deploy_inventory.nix");

    // Deploy config is NOT a HashMap, it's a direct struct based on the specs
    // "Contains SSH keys and deployment-specific configurations... { sshKeys = [ ... ]; }"
    let deploy: DeployConfig = parse_nix_file(&path)?;

    assert_eq!(deploy.ssh_keys.len(), 1);
    assert_eq!(
        deploy.ssh_keys[0],
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI..."
    );

    // Serialize back
    let output = deploy.to_nix();
    let out_path = nix_dir.join("test_deploy_inventory_out.nix");
    fs::write(&out_path, &output)?;

    // Verify re-parsing
    let deploy2: DeployConfig = parse_nix_file(&out_path)?;
    assert_eq!(deploy, deploy2);

    // Cleanup
    let _ = fs::remove_file(out_path);
    Ok(())
}
