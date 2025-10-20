{
  description = "eframe devShell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in with pkgs; {
        devShells.default = let
          rust_toolchain = rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in mkShell rec {
          buildInputs = [
            # basic stuff
            git
            curl
            which

            # Rust
            rust_toolchain
            trunk

            # misc. libraries
            openssl
            pkg-config

            # GUI libs
            libxkbcommon
            libGL
            pipewire
            fontconfig

            # wayland libraries
            wayland

            # x11 libraries
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11


          ];

          RUST_SRC_PATH = "${rust_toolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      });
}


# libfreetype6-dev libasound2-dev libexpat1-dev libxcb-composite0-dev \
#    libbz2-dev libsndio-dev freeglut3-dev libxmu-dev libxi-dev libfontconfig1-dev \
#    libxcursor-dev