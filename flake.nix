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

        inherit (pkgs) rustPlatform mkShell stdenv lib;
        buildInputs = [];
        nativeBuildInputs = [];
      in
      rec {
        # `nix build`
        packages.${pname} = rustPlatform.buildRustPackage {
          inherit pname version buildInputs nativeBuildInputs;
          src = ./.;
          cargoSha256 = "";
        };
        defaultPackage = packages.${pname};

        # `nix run`
        apps.${pname} = utils.lib.mkApp {
          drv = packages.${pname};
        };
        defaultApp = apps.${pname};

        # `nix develop`
        devShell = mkShell {
          inherit nativeBuildInputs;
          buildInputs = with pkgs; [ rust-bin.nightly.latest.default ];
        };
      }
    );
}
