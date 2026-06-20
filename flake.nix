{
  description = "roBa-rmk firmware development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            gcc
            libclang
            pkg-config

            rustup
            cargo-make
            cargo-binutils
            flip-link
            probe-rs-tools
          ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

          shellHook = ''
            export CARGO_NET_GIT_FETCH_WITH_CLI=true
          '';
        };
      });
}
