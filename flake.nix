{
  description = "Personal website";

  nixConfig.allow-import-from-derivation = false;

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        inputs',
        pkgs,
        ...
      }: let
        rustToolchain = inputs'.fenix.packages.stable;
        rust = rustToolchain.withComponents [
          "cargo"
          "rustc"
          "rust-src"
          "rust-std"
        ];
        cargoTOML = pkgs.lib.importTOML ./Cargo.toml;
      in {
        packages.default =
          (pkgs.makeRustPlatform {
            cargo = rustToolchain.minimalToolchain;
            rustc = rustToolchain.minimalToolchain;
          })
          .buildRustPackage {
            pname = cargoTOML.package.name;
            version = cargoTOML.package.version;

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            postInstall = ''
              cp -r ./public $out
            '';
          };

        devShells.default = pkgs.mkShell {
          # inputsFrom = [ self'.packages.default ];
          nativeBuildInputs =
            [
              rust
            ]
            ++ (with pkgs; [bacon]);
        };

        formatter = pkgs.alejandra;
      };
      flake = {
        nixosModules.website = {
          lib,
          pkgs,
          ...
        }: {
          imports = [./nixos.nix];

          config.wambolt.website.package = lib.mkDefault self.packages.${pkgs.system}.default;
        };
      };
    };
}
