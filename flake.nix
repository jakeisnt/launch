{
  inputs = {
    nixpkgs.url      = github:nixos/nixpkgs/release-22.05;
    utils.url        = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
    naersk.url       = github:nix-community/naersk;

    # Used for shell.nix
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, utils, naersk, ... } @ inputs:
    let
      name = "launch";
      description = "basic program launcher";
      overlays = [ rust-overlay.overlays.default ];
      # Our supported systems are the same supported systems as the Rust binaries
      systems = builtins.attrNames inputs.rust-overlay.packages;
    in utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs { inherit overlays system; };
        rust_channel = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = naersk.lib."${system}".override {
          cargo = rust_channel;
          rustc = rust_channel;
        };
        xDeps = with pkgs; with xorg; [
          xorgserver

          # development lib
          libX11

          # xorg input modules
          xf86inputevdev
          xf86inputsynaptics
          xf86inputlibinput

          # dyn libs
          libXrandr
          libXcursor
          libXi
          mesa

          # just for gui lib?
          glib
          pango
          cairo
          gdk-pixbuf
          atk
          gtk3
          libGL

          # support executing gpu-supported programs
          vulkan-tools
        ];
      in {
        defaultPackage = naersk-lib.buildPackage {
          pname = name;
          root = ./.;

          buildInputs = with pkgs; [
            cmake
            pkg-config
            fontconfig
            freetype
          ] ++ xDeps;
        };

        devShells.default = pkgs.mkShell rec {
          inherit name description;
          buildInputs = with pkgs; [
            rust_channel
            rust-analyzer
            cargo
            # lld
            # How do I use mold? https://discourse.nixos.org/t/using-mold-as-linker-prevents-libraries-from-being-found/18530/4
            llvmPackages.bintools
            pkg-config
            fontconfig
            freetype
            clang
            cmake
            # build and ship a wasm app
            trunk

            libstdcxx5
          ] ++ xDeps;

          RUST_BACKTRACE = "1";
          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
          # "${pkgs.lib.makeLibraryPath (xDeps) }:$LD_LIBRARY_PATH";
          PKG_CONFIG_PATH = "${pkgs.lib.makeLibraryPath (xDeps) }:$PKG_CONFIG_PATH";
          # for rust-analyzer; the target dir of the compiler for the project
          OUT_DIR = "./target";
        };

        # For compatibility with older versions of the `nix` binary
        devShell = self.devShells.${system}.default;
      });
}
