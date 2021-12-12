{
  description = "Solutions of Advent of Code in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, fenix, crate2nix, ... }:
    let
      name = "adventofcode-rust";
    in
      utils.lib.eachDefaultSystem
        (system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                fenix.overlay
                (self: super: {
                  rustc = self.fenix.stable.minimalToolchain;
                  cargo = self.fenix.stable.minimalToolchain;
                })
              ];
            };

            inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
              generatedCargoNix;

            project = pkgs.callPackage
              (generatedCargoNix {
                inherit name;
                src = ./.;
              })
              {
                defaultCrateOverrides = pkgs.defaultCrateOverrides // {
                };
              };

          in
            rec {
              packages.${name} = project.rootCrate.build;

              # `nix build`
              defaultPackage = packages.${name};

              # `nix run`
              apps.${name} = utils.lib.mkApp {
                inherit name;
                drv = packages.${name};
              };
              defaultApp = apps.${name};

              # `nix develop`
              devShell = pkgs.mkShell
                {
                  nativeBuildInputs = [
                    pkgs.fenix.stable.toolchain
                    pkgs.rust-analyzer
                  ];
                  RUST_SRC_PATH = "${pkgs.fenix.stable.rust-src}/bin/rust-lib/src";
                };
            }
        );
}
