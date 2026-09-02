{
  description = "tuningplayground";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rustfmt" "clippy"];
        targets = ["wasm32-unknown-unknown"];
      };
    in {
      formatter = pkgs.alejandra;

      devShells.default = pkgs.mkShell {
        packages = [
          rustToolchain
          pkgs.binaryen
          pkgs.nodejs_22
          pkgs.python3
          pkgs.wasm-pack
        ];
      };
    });
}
