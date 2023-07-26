{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    proc-flake.url = "github:srid/proc-flake";
    flake-root.url = "github:srid/flake-root";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.proc-flake.flakeModule
        inputs.flake-root.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
          # When filtering sources, we want to allow assets other than .rs files
          src = lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source
            filter = path: type:
              (lib.hasSuffix "\.html" path) ||
              (lib.hasSuffix "\.scss" path) ||
              # Example of a folder for images, icons, etc
              (lib.hasInfix "/assets/" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type)
            ;
          };

          buildArgs = rec {
            # Arguments to be used by both the client and the server
            # When building a workspace with crane, it's a good idea
            # to set "pname" and "version".
            common = {
              inherit src;
              pname = "leptos-fullstack";
              version = "0.1.0";
            };
            native = common // {
              pname = "leptos-fullstack-native";
            };
            # it's not possible to build the server on the
            # wasm32 target, so we only build the client.
            wasm = common // {
              pname = "leptos-fullstack-wasm";
              cargoExtraArgs = "--package=frontend";
              CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            };
          };

          rustPackages = rec {
            backend = rec {
              # Build *just* the cargo dependencies, so we can reuse
              # all of that work (e.g. via cachix) when running in CI
              cargoArtifacts = craneLib.buildDepsOnly buildArgs.native;
              package = craneLib.buildPackage (buildArgs.native // {
                inherit cargoArtifacts;
                # The server needs to know where the client's dist dir is to
                # serve it, so we pass it as an environment variable at build time
                CLIENT_DIST = frontend.package;
              });
            };

            frontend = rec {
              cargoArtifacts = craneLib.buildDepsOnly (buildArgs.wasm // {
                doCheck = false;
              });
              # Build the frontend of the application.
              # This derivation is a directory you can put on a webserver.
              package = craneLib.buildTrunkPackage (buildArgs.wasm // {
                pname = "frontend";
                cargoArtifacts = cargoArtifacts;
                trunkIndexPath = "frontend/index.html";
              });
            };
          };

          nonRustDeps = [
            pkgs.libiconv
          ];
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };

          checks = {
            # Run clippy (and deny all warnings) on the crate source,
            # again, reusing the dependency artifacts from above.
            #
            # Note that this is done as a separate derivation so that
            # we can block the CI if there are issues here, but not
            # prevent downstream consumers from building our crate by itself.
            leptos-fullstack-clippy = craneLib.cargoClippy (buildArgs.common // {
              inherit (rustPackages.backend) cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              # Here we don't care about serving the frontend
              CLIENT_DIST = "";
            });
          };

          # Rust package
          packages = {
            backend = rustPackages.backend.package;
            frontend = rustPackages.frontend.package;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = builtins.attrValues self'.checks ++ [
              config.treefmt.build.devShell
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
            '';
            buildInputs = nonRustDeps;
            nativeBuildInputs = with pkgs; [
              just
              rustToolchain
              cargo-watch
              trunk

              config.proc.groups.watch-project.package
            ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };

          proc.groups.watch-project = {
            processes = {
              frontend.command = lib.getExe (pkgs.writeShellApplication {
                name = "frontend-watch";
                text = ''
                  set -x
                  cd ./frontend
                  trunk serve --open
                '';
              });
              backend.command = lib.getExe (pkgs.writeShellApplication {
                name = "backend-watch";
                text = ''
                  set -x
                  cd ./backend
                  cargo watch -x run
                '';
              });
            };
          };
        };
    };
}
