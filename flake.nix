{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
        };

        llvm = pkgs.llvmPackages_18.llvm;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            llvm
            pkgs.libffi
            pkgs.libxml2
            pkgs.pkg-config
            pkgs.clang_18
            pkgs.mold
            pkgs.bugstalker
          ];

          LLVM_SYS_181_PREFIX = "${llvm.dev}";
        };
      }
    );
}
