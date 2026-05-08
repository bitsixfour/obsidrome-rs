{
  description = "Navidrome/Last.fm to Obsidian scrobbler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      nixosModules.default = import ./nix/module.nix { inherit self; };
    }
    // flake-utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs { inherit system; };
        obsidianfm = pkgs.rustPlatform.buildRustPackage {
          pname = "obsidianfm";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.pkg-config ];
        };
      in
      {
        packages.default = obsidianfm;
        packages.obsidianfm = obsidianfm;

        apps.default = flake-utils.lib.mkApp {
          drv = obsidianfm;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            clippy
            pkg-config
            rustc
            rustfmt
          ];
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
