{
  description = "An empty project that uses Zig.";

  inputs = {
    nixpkgs.url     = github:nixos/nixpkgs/release-22.05;
    flake-utils.url = github:numtide/flake-utils;
    zig.url         = github:mitchellh/zig-overlay;
    gyro.url        = github:jakeisnt/gyro;

    # Used for shell.nix
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = {
    self,
      nixpkgs,
      flake-utils,
      ...
  } @ inputs: let
    overlays = [
      # Other overlays
      (final: prev: {
        zigpkgs = inputs.zig.packages.${prev.system};
        gyro = inputs.gyro.packages.${prev.system}.default;
      })
    ];

    # Our supported systems are the same supported systems as the Zig binaries
    systems = builtins.attrNames inputs.zig.packages;

  in
    flake-utils.lib.eachSystem systems (
      system: let
        pkgs = import nixpkgs {inherit overlays system;};
      in rec {
        defaultPackage = pkgs.zigpkgs.master;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            zigpkgs.master
            pkgs.gyro
          ];

          buildInputs = with pkgs; [
            wayland-protocols
            wlroots
            pixman
            libxkbcommon
            libevdev
            pkg-config

            cairo
            pango
            zls
            SDL2
            SDL2_ttf
          ];

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [ SDL2 SDL2_ttf cairo pango pixman ])}:$LD_LIBRARY_PATH";
        };

        # For compatibility with older versions of the `nix` binary
        devShell = self.devShells.${system}.default;
      }
    );
}
