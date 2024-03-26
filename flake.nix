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
            mpv
            rust-analyzer
            # install rust with rust_analyzer
            (rust-bin.stable.latest.default.override {
                extensions = [ "rust-src" ];
            })
          ];
          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}
