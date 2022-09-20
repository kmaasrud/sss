{
  description = "CRATE DESCRIPTION";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust }:
    utils.lib.eachDefaultSystem (system:
      let
        toml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        pname = toml.package.name;
        version = toml.package.version;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust) ];
        };

        toolchain =
          pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);

        inherit (pkgs) mkShell stdenv lib;
        buildInputs = [ toolchain ];
        nativeBuildInputs = [ ];

        rustPlatform = pkgs.makeRustPlatform {
          rustc = toolchain;
          cargo = toolchain;
        };
      in rec {
        # `nix build`
        packages.${pname} = rustPlatform.buildRustPackage {
          inherit pname version buildInputs nativeBuildInputs;
          src = lib.cleanSource ./.;
          cargoSha256 = "sha256-i7s5QvY34o3UoMj3qjsomRE0lILh5KRK64Wm6/30v4I=";
        };
        packages.default = packages.${pname};

        # `nix run`
        apps.${pname} = utils.lib.mkApp { drv = packages.${pname}; };
        defaultApp = apps.${pname};

        # `nix develop`
        devShells.default = mkShell { inherit buildInputs nativeBuildInputs; };
      });
}
