{
  description = "OSD for the COSMIC desktop environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nix-filter.url = "github:numtide/nix-filter";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      # keep-sorted start
      crane,
      fenix,
      flake-utils,
      nix-filter,
      nixpkgs,
      # keep-sorted end
      ...
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = (crane.mkLib pkgs).overrideToolchain (fenix.packages.${system}.stable.toolchain);

        pkgDef = {
          src = nix-filter.lib.filter {
            root = ./.;
            include = [
              # keep-sorted start
              ./Cargo.lock
              ./Cargo.toml
              ./i18n
              ./i18n.toml
              ./src
              # keep-sorted end
            ];
          };

          nativeBuildInputs = with pkgs; [
            makeWrapper
            pkg-config
          ];

          buildInputs = with pkgs; [
            # keep-sorted start
            cairo
            gdk-pixbuf
            glib
            graphene
            gtk4
            libinput
            libpulseaudio
            libxkbcommon
            polkit
            systemd
            wayland
            # keep-sorted end
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly pkgDef;
        cosmic-osd = craneLib.buildPackage (
          pkgDef
          // {
            inherit cargoArtifacts;

            postFixup = ''
              wrapProgram $out/bin/cosmic-osd \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath pkgDef.buildInputs}
            '';
          }
        );
      in
      {
        checks = {
          inherit cosmic-osd;
        };

        packages.default = cosmic-osd;

        apps.default = flake-utils.lib.mkApp {
          drv = cosmic-osd;
        };

        devShells.default = craneLib.devShell {
          inputsFrom = [ cosmic-osd ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath pkgDef.buildInputs;

          packages = with pkgs; [ rust-analyzer ];
        };
      }
    );

  nixConfig = {
    # Cache for the Rust toolchain in fenix
    extra-substituters = [ "https://nix-community.cachix.org" ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
