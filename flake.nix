{
  description = "An empty project that uses Zig.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-22.05";
    flake-utils.url = "github:numtide/flake-utils";
    zig.url = "github:mitchellh/zig-overlay";

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
      })
    ];

    # Our supported systems are the same supported systems as the Zig binaries
    systems = builtins.attrNames inputs.zig.packages;


    gyro = nixpkgs.mkDerivation rec {
      pname = "gyro";
      version = "0.7.0";
      src = builtins.fetchurl {
        url = "https://github.com/mattnite/gyro/releases/download/${version}/gyro-${version}-linux-x86_64.tar.gz";
        sha256 = "1wnv15y5ccwqnbsr93npf31g9r7pjlqzmbl4q217gzyfvz6l3gdd";
      };

      # nativeBuildInputs = [ autoPatchelfHook ];
      installPhase = ''
        install -m755 -D gyro $out/bin/gyro
      '';
    };
  in
    flake-utils.lib.eachSystem systems (
      system: let
        pkgs = import nixpkgs {inherit overlays system;};
      in rec {
        defaultPackage = pkgs.zigpkgs.master;
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            zigpkgs.master
            gyro
          ];
        };

        # For compatibility with older versions of the `nix` binary
        devShell = self.devShells.${system}.default;
      }
    );
}
