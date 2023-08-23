{ self, lib, inputs, flake-parts-lib, ... }:

let
  inherit (flake-parts-lib)
    mkPerSystemOption;
in
{
  options = {
    perSystem = mkPerSystemOption
      ({ config, self', inputs', pkgs, system, ... }: {
        options = {
          leptos-fullstack.overrideCraneArgs = lib.mkOption {
            type = lib.types.functionTo lib.types.attrs;
            default = _: { };
            description = "Override crane args for the leptos-fullstack package";
          };
        };
        config =
          let
            cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
            inherit (cargoToml.package) name version;

            rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile (self + /rust-toolchain.toml)).override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "clippy"
              ];
            };
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

            # When filtering sources, we want to allow assets other than .rs files
            # TODO: Don't hardcode these!
            src = lib.cleanSourceWith {
              src = self; # The original, unfiltered source
              filter = path: type:
                (lib.hasSuffix "\.html" path) ||
                (lib.hasSuffix "tailwind.config.js" path) ||
                # Example of a folder for images, icons, etc
                (lib.hasInfix "/assets/" path) ||
                (lib.hasInfix "/css/" path) ||
                # Default filter from crane (allow .rs files)
                (craneLib.filterCargoSources path type)
              ;
            };

            # Crane builder for cargo-leptos projects
            craneBuild = rec {
              crateName = lib.replaceStrings [ "-" ] [ "_" ] name;
              args = {
                inherit src;
                pname = name;
                version = version;
                buildInputs = [
                  pkgs.cargo-leptos
                  pkgs.binaryen # Provides wasm-opt
                  tailwindcss
                ];
                nativeBuildInputs = [
                  pkgs.makeWrapper
                ];
              };
              dummySrc =
                let
                  cargoOnly = builtins.path {
                    name = "dummy-src";
                    path = src;
                    filter = path: _: baseNameOf path == "Cargo.toml" || baseNameOf path == "Cargo.lock";
                  };
                in
                pkgs.runCommand "dummy-src"
                  {
                    ASSETS = cargoToml.package.metadata.leptos.assets-dir;
                    TAILWIND_INPUT_FILE = cargoToml.package.metadata.leptos.tailwind-input-file;
                  } ''
                  mkdir -p $out
                  cp -r ${./dummy}/src $out/
                  mkdir -p $out/$ASSETS $out/''$(dirname $TAILWIND_INPUT_FILE)
                  touch $out/$TAILWIND_INPUT_FILE
                  cp -r ${cargoOnly}/* $out/
                '';
              cargoArtifacts = craneLib.buildDepsOnly (args // {
                src = null;
                inherit dummySrc;
                cargoVendorDir = craneLib.vendorCargoDeps args;
                buildPhaseCargoCommand = ''
                  cat Cargo.toml
                  cargo leptos build --release -vvv
                  # Get rid of the dummy src artifacts, as it can break `cargo leptos build` later.
                  find target/server -name \*${crateName}\*lib | xargs rm -rf
                  find target/server -name \*${name}\*lib | xargs rm -rf
                '';
                cargoTestCommand = ''
                  cargo leptos test --release -vvv
                '';
              });
              buildArgs = args // {
                inherit cargoArtifacts;
                buildPhaseCargoCommand = ''
                  cargo leptos build --release -vvv;
                '';
                cargoTestCommand = ''
                  cargo leptos test --release -vvv
                '';
                installPhaseCommand = ''
                  mkdir -p $out/bin
                  cp target/server/release/${name} $out/bin/
                  cp -r target/site $out/bin/
                  wrapProgram $out/bin/${name} \
                    --set LEPTOS_SITE_ROOT $out/bin/site
                '';
              };
              package = craneLib.buildPackage (buildArgs // config.leptos-fullstack.overrideCraneArgs buildArgs);

              check = craneLib.cargoClippy (args // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
              });
            };

            rustDevShell = pkgs.mkShell {
              shellHook = ''
                # For rust-analyzer 'hover' tooltips to work.
                export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
              '';
              buildInputs = [
                pkgs.libiconv
              ];
              nativeBuildInputs = [
                rustToolchain
              ];
            };

            tailwindcss = pkgs.nodePackages.tailwindcss.overrideAttrs
              (oa: {
                plugins = [
                  pkgs.nodePackages."@tailwindcss/aspect-ratio"
                  pkgs.nodePackages."@tailwindcss/forms"
                  pkgs.nodePackages."@tailwindcss/language-server"
                  pkgs.nodePackages."@tailwindcss/line-clamp"
                  pkgs.nodePackages."@tailwindcss/typography"
                ];
              });
          in
          {
            # Rust package
            packages.${name} = craneBuild.package;
            packages."${name}-deps" = craneBuild.cargoArtifacts;

            checks."${name}-clippy" = craneBuild.check;

            # Rust dev environment
            devShells.${name} = pkgs.mkShell {
              inputsFrom = [
                rustDevShell
              ];
              nativeBuildInputs = with pkgs; [
                tailwindcss
                cargo-leptos
                binaryen # Provides wasm-opt
              ];
            };
          };
      });
  };
}
