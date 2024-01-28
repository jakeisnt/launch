{
  inputs = {
    nixpkgs.url         = github:nixos/nixpkgs/release-22.05;
    utils.url           = github:numtide/flake-utils;
    nixpkgs-mozilla.url = github:mozilla/nixpkgs-mozilla;
    naersk.url          = github:nix-community/naersk;

    # Used for shell.nix
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = { self, nixpkgs, nixpkgs-mozilla, utils, naersk, ... } @ inputs:
    let
      name = "launch";
      description = "basic program launcher";
      overlays = [ (import nixpkgs-mozilla) ];
    in utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit overlays system; };
        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain.toml;
          sha256 = "eMJethw5ZLrJHmoN2/l0bIyQjoTX1NsvalWSscTixpI=";
          #        ^ After you run `nix build`, replace this with the actual
          #          hash from the error message
        }).rust;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
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

          libstdcxx5
        ];

	macDeps = with pkgs; [ ];
      in {
        defaultPackage = naersk'.buildPackage rec {
          pname = name;
          root = ./.;

          buildInputs = with pkgs; [
            cmake
            pkg-config
            fontconfig
            freetype
          ] ++ (if system == "aarch64-darwin" then macDeps else xDeps);

          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
          # "${pkgs.lib.makeLibraryPath (xDeps) }:$LD_LIBRARY_PATH";
          PKG_CONFIG_PATH = "${pkgs.lib.makeLibraryPath (xDeps) }:$PKG_CONFIG_PATH";
        };

        devShells.default = pkgs.mkShell rec {
          inherit name description;

          # TODO: something is missing in `buildInputs` that is needed
          # to make dynamic linking work in isolation. I don't know what.
          nativeBuildInputs = [ toolchain ];
          buildInputs = with pkgs; [
            # How do I use mold? https://discourse.nixos.org/t/using-mold-as-linker-prevents-libraries-from-being-found/18530/4
            llvmPackages.bintools
            pkg-config
            fontconfig
            freetype
            clang
            cmake
            # build and ship a wasm app
            trunk

          ] ++ (if system == "aarch64-darwin" then macDeps else xDeps);

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
