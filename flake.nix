{
  description = "Development environment for roBa-rmk firmware";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSystem = f:
        builtins.listToAttrs (map (system: {
          name = system;
          value = f system;
        }) systems);
    in {
      devShells = forEachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          rustToolchain = pkgs.rust-bin.stable."1.93.0".default.override {
            extensions = [ "rust-src" "rustfmt" "llvm-tools-preview" ];
            targets = [ "thumbv7em-none-eabihf" ];
          };
          cargoHexToUf2 = pkgs.rustPlatform.buildRustPackage rec {
            pname = "cargo-hex-to-uf2";
            version = "0.1.2";
            src = pkgs.fetchurl {
              name = "${pname}-${version}.tar.gz";
              url = "https://crates.io/api/v1/crates/${pname}/${version}/download";
              hash = "sha256-z5YSymOl5xdkiYxPeLp3iXtgF2ZY7iFeafsC2zfoc6k=";
            };
            cargoHash = "sha256-4bNL5W0OF8NbRTFhDrArdpjRsSWoTwnnTDcPcOvPSNU=";
          };
        in {
          default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.cargo-make
              pkgs.flip-link
              pkgs.cargo-binutils
              cargoHexToUf2
              pkgs.clang
              pkgs.git
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };
        });
    };
}
