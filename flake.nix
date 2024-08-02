{
  description = "Development shell for Tasker";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
      with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            cargo-dist
            cargo-mutants
            cargo-nextest
            just
            mold
            release-plz
            sccache
            taplo
            valgrind

            (rust-bin.stable.latest.default.override {
              extensions = [
                "cargo"
                "clippy"
                "rust-analyzer"
                "rustc"
                "rustfmt"
                "rust-src"
                "rust-std"
              ];
            })
          ];

          shellHook = ''
            export RUSTC_WRAPPER=sccache
          '';
        };
      });
}
