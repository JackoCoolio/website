{
  description = "Personal website";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        inputs',
        pkgs,
        ...
      }: let
        rustToolchainFile = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain;
        rustToolchain = (
          inputs'.fenix.packages.fromToolchainName {
            name = rustToolchainFile.channel;
            sha256 = "sha256-VZZnlyP69+Y3crrLHQyJirqlHrTtGTsyiSnZB8jEvVo=";
          }
        );
        rust = rustToolchain.toolchain;
      in {
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
        nixosModules.website = import ./nixos.nix;
      };
    };
}
