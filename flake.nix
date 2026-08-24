{
  description = "PlumJam's Website Nix Flake";

  nixConfig = {
    builders-use-substitutes = true;
    flake-registry = "";
    show-trace = true;

    experimental-features = [
      "flakes"
      "nix-command"
      "pipe-operators"
    ];

    extra-substituters = [
      "https://cache.garnix.io/"
      "https://nix-community.cachix.org/"
    ];

    extra-trusted-public-keys = [
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      advisory-db,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let

        pkgs = import nixpkgs { inherit system; };

        inherit (pkgs) lib;

        # Pinned to 1.86.0 to fix this issue: https://github.com/DioxusLabs/dioxus/discussions/4183
        rustToolchain =
          let
            toolchain = fenix.packages.${system}.toolchainOf {
              channel = "1.86.0";
              sha256 = "sha256-X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
            };
            wasm-target = fenix.packages.${system}.targets.wasm32-unknown-unknown.toolchainOf {
              channel = "1.86.0";
              sha256 = "sha256-X/4ZBHO3iW0fOenQ3foEvscgAPJYl2abspaBThDOukI=";
            };
          in
          fenix.packages.${system}.combine [
            toolchain.rustc
            toolchain.cargo
            toolchain.clippy
            fenix.packages.${system}.latest.rustfmt # Nightly only for rustfmt.
            toolchain.rust-src
            toolchain.rust-analyzer
            wasm-target.rust-std # wasm32-unknown-unknown for Dioxus web.
          ];

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        tailwindNormal = ''
          					bunx tailwindcss -i normal/input.css -o normal/gen-tailwind.css --minify --content 'normal/**/*.rs'
        '';
        tailwindNerd = ''
          					bunx tailwindcss -i nerd/input.css -o nerd/gen-tailwind.css --minify --content 'nerd/**/*.rs'
        '';

        tailwindInputs = [
          pkgs.bun
          pkgs.tailwindcss_4
        ];

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build only the cargo dependencies.
        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          doCheck = false; # No tests for WASM target.
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./libs)
              (craneLib.fileset.commonCargoSources ./normal)
              (craneLib.fileset.commonCargoSources ./nerd)
              ./libs/assets
              ./normal/input.css
              ./normal/favicon.ico
              ./normal/posts
              ./nerd/input.css
              ./nerd/favicon.ico
            ];
          };

        # Common library doesn't need WASM target or Tailwind.
        libs = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "libs";
            cargoExtraArgs = "--package libs";

            src = fileSetForCrate ./libs;

            CARGO_BUILD_TARGET = null; # Remove WASM target for libs.
          }
        );

        normal = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "normal";
            cargoExtraArgs = "--package normal";

            src = fileSetForCrate ./normal;

            preBuild = tailwindNormal;
            nativeBuildInputs = tailwindInputs;
          }
        );

        nerd = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "nerd";
            cargoExtraArgs = "--package nerd";

            src = fileSetForCrate ./nerd;

            preBuild = tailwindNerd;
            nativeBuildInputs = tailwindInputs;
          }
        );
      in
      {
        checks = lib.mapAttrs' (n: v: lib.nameValuePair "package-${n}" v) self.packages.${system} // {

          rust-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;

              src = fileSetForCrate ./.;

              preBuild = tailwindNormal + tailwindNerd;
              nativeBuildInputs = tailwindInputs;

              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          rust-doc = craneLib.cargoDoc (
            commonArgs
            // {
              inherit cargoArtifacts;

              src = fileSetForCrate ./.;

              env.RUSTDOCFLAGS = "--deny warnings";

              preBuild = tailwindNormal + tailwindNerd;
              nativeBuildInputs = tailwindInputs;
            }
          );

          rust-fmt = craneLib.cargoFmt {
            inherit cargoArtifacts;

            src = fileSetForCrate ./.;

            rustFmtExtraArgs = "--config-path ${./.rustfmt.toml}";
          };

          toml-fmt = craneLib.taploFmt {
            inherit cargoArtifacts;

            src = lib.sources.sourceFilesBySuffices src [ ".toml" ];

            taploExtraArgs = "--config ${./.taplo.toml}";
          };

          rust-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
        };

        packages = {
          inherit libs normal nerd;
          default = normal;
        };

        apps = {
          libs = flake-utils.lib.mkApp {
            drv = libs;
          };
          normal = flake-utils.lib.mkApp {
            drv = normal;
          };
          nerd = flake-utils.lib.mkApp {
            drv = nerd;
          };
          default = flake-utils.lib.mkApp {
            drv = normal;
          };
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          buildInputs = [
            pkgs.bun
            pkgs.dioxus-cli
            pkgs.tailwindcss_4
            pkgs.wasm-bindgen-cli_0_2_100
            pkgs.watchman

            pkgs.pkg-config
            pkgs.glib
            pkgs.gtk3
            pkgs.cairo
            pkgs.pango
            pkgs.atk
            pkgs.gdk-pixbuf
            pkgs.libsoup_3
            pkgs.webkitgtk_4_1
          ];
        };
      }
    );
}
