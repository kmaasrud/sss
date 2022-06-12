{
  description = "Super Simple Slides";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust }:
    {
      overlays.default = nixpkgs.lib.composeManyExtensions [
        rust.overlay
        (final: prev: {
          customRustToolchain = final.rust-bin.selectLatestNightlyWith
            (toolchain:
              toolchain.default.override {
                extensions = [ "rust-std" "rust-src" ];
              });
        })
      ];
    } // utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      in rec {
        # `nix develop`
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # rust
            customRustToolchain
            rust-analyzer
          ];
        };
      });
}
