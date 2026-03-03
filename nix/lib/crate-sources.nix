{
  lib,
  root,
}: let
  commonFiles = lib.fileset.unions [
    (root + /Cargo.toml)
    (root + /Cargo.lock)
  ];

  # Helper to create a source for a crate
  makeCrate = path: {
    inherit path;
    fileset = lib.fileset.unions [
      (path + /Cargo.toml)
      (lib.fileset.fileFilter (file: file.hasExt "rs") path)
    ];
  };

  # Define crates
  bins = let
    dir = root + /crates/bins;
  in {
    ncm = makeCrate (dir + /ncm);
  };

  libs = let
    dir = root + /crates/libs;
  in {
    ncm-core = makeCrate (dir + /ncm-core);
    ncm-tui = makeCrate (dir + /ncm-tui);
    ncm-inventory = makeCrate (dir + /ncm-inventory);
  };

  allCrates = bins // libs;

  # Combined fileset for the workspace
  workspaceFileset = lib.fileset.unions (
    [commonFiles] ++ (lib.mapAttrsToList (_: crate: crate.fileset) allCrates)
  );
in {
  inherit
    bins
    libs
    allCrates
    workspaceFileset
    ;
}
