{
  "web-server-01" = {
    sshKeys = [
      "ssh-ed25519 ..."
    ];
    users = [
      "adminUser"
      "deployUser"
    ];
    node_services = [
      "nginx-service"
      "database-service"
    ];
    bootDisks = [
      "/dev/sda"
      "/dev/sdb"
    ];
    deployment = {
      ipv4 = "192.168.1.10";
      ipv6 = "2001:db8::1";
    };
  };
}
