{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            eza
            fd
            ripgrep
            python3
            rust-analyzer
            # install rust with rust_analyzer
            (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
            })
          ];
          shellHook = ''
            export CARGO_HOME=$PWD/.cargo/ #put dependencies inside this project
            alias ls="eza --icons --long"
            alias find=fd
          '';
        };
      }
    );
}
