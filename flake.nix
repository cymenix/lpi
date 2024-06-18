{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs = {
          follows = "nixpkgs";
        };
        flake-utils = {
          follows = "flake-utils";
        };
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        inherit (nixpkgs) lib;
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system lib overlays;
        };
        rustToolchain = with pkgs;
          (pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = ["rust-src" "clippy" "llvm-tools"];
          };
        buildInputs = with pkgs; [
          coreutils
          bash
          openssl
          pkg-config
        ];
        nativeBuildInputs = with pkgs; [
          rustToolchain
          rust-analyzer
        ];
      in
        with pkgs; {
          packages = {
            default = import ./default.nix {inherit pkgs;};
          };
          devShell = mkShell {
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
    );
}
