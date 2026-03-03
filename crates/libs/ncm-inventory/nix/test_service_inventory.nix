{
  "database-service" = {
    enable = true;
    replicas = 3;
    stateDirs = [
      "/var/lib/postgresql"
    ];
    dependsOn = [];
    secrets = [
      "db-password"
    ];
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
