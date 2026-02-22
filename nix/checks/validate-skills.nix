{
  pkgs,
  perSystem,
  ...
}:
pkgs.stdenv.mkDerivation {
  name = "validate-skills";
  src = pkgs.lib.fileset.toSource {
    root = ../..;
    fileset = pkgs.lib.fileset.unions [
      ../../.agent
      (pkgs.lib.fileset.maybeMissing (../../AGENTS.md))
    ];
  };

  dontBuild = true;

  installPhase = ''
    mkdir -p $out
    ${perSystem.agent-persona-manager.persona-cli}/bin/persona check
    touch $out/pass
  '';
}
