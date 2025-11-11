{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    rust-overlay,
    ...
  }: let
    inherit (nixpkgs) lib;

    forEachSystem = cb:
      lib.genAttrs
      (lib.intersectLists lib.systems.flakeExposed lib.platforms.linux)
      (system: cb system nixpkgs.legacyPackages.${system});
  in {
    packages = forEachSystem (_: pkgs: let
      craneLib = crane.mkLib pkgs;

      commonCraneArgs = {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;
        doCheck = false;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonCraneArgs;
    in {
      fenster = craneLib.buildPackage (commonCraneArgs
        // {
          cargoExtraArgs = "-p fenster";
          buildInputs = with pkgs; [libxkbcommon];
          inherit cargoArtifacts;
          meta.mainProgram = "fenster";
        });
      fenster-daemon = craneLib.buildPackage (commonCraneArgs
        // {
          cargoExtraArgs = "-p fenster-daemon";
          inherit cargoArtifacts;
          meta.mainProgram = "fenster-daemon";
        });
    });

    devShells = forEachSystem (system: pkgs: let
      craneLib = crane.mkLib pkgs;
    in {
      default = craneLib.devShell {
        buildInputs = with self.packages.${system}; (fenster.buildInputs ++ fenster-daemon.buildInputs);
      };
    });

    formatter = forEachSystem (_: pkgs: let
      pkgs' = pkgs.extend rust-overlay.overlays.default;
    in
      pkgs'.writeShellApplication {
        name = "format";
        runtimeInputs = with pkgs'; [fd alejandra rust-bin.nightly.latest.rustfmt];
        text = ''
          fd "$@" -t f -e nix -x alejandra -q '{}'
          fd "$@" -t f -e rs -x rustfmt '{}'
        '';
      });
  };
}
