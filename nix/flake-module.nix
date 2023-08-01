{ self, lib, inputs, flake-parts-lib, ... }:

let
  inherit (flake-parts-lib)
    mkPerSystemOption;
  inherit (lib)
    mkOption
    types;
in
{
  options = {
    perSystem = mkPerSystemOption
      ({ config, self', inputs', pkgs, system, ... }: {
        options.leptos-app = mkOption {
          type = types.attrsOf types.any;
          default = { };
          description = "Configuration for the Leptos app";
        };
        config =
          let
            cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
            rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile (self + /rust-toolchain.toml)).override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            };
            craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;
            # When filtering sources, we want to allow assets other than .rs files
            src = lib.cleanSourceWith {
              src = self; # The original, unfiltered source
              filter = path: type:
                (lib.hasSuffix "\.html" path) ||
                (lib.hasSuffix "tailwind.config.js" path) ||
                # Example of a folder for images, icons, etc
                (lib.hasInfix "/assets/" path) ||
                # Default filter from crane (allow .rs files)
                (craneLib.filterCargoSources path type)
              ;
            };

            rustPackages = {
              default = rec {
                args = {
                  inherit src;
                  pname = cargoToml.package.name;
                  version = cargoToml.package.version;
                  doCheck = false;
                  buildInputs = [
                    pkgs.cargo-leptos
                    pkgs.binaryen # Provides wasm-opt
                    tailwindcss
                  ];
                };
                cargoArtifacts = craneLib.buildDepsOnly args;
                package = craneLib.buildPackage (args // {
                  inherit cargoArtifacts;
                  buildPhaseCargoCommand = "cargo leptos build --release -vvv";
                  nativeBuildInputs = [
                    pkgs.makeWrapper
                  ];
                  installPhaseCommand = ''
                    mkdir -p $out/bin
                    cp target/server/release/${cargoToml.package.name} $out/bin/
                    cp -r target/site $out/bin/
                    wrapProgram $out/bin/${cargoToml.package.name} \
                      --set LEPTOS_SITE_ROOT $out/bin/site
                  '';
                });
              };
            };


            rustDevShell = pkgs.mkShell {
              shellHook = ''
                # For rust-analyzer 'hover' tooltips to work.
                export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
              '';
              buildInputs = [
                pkgs.libiconv
              ];
              nativeBuildInputs = with pkgs; [
                rustToolchain
                cargo-watch
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
            packages = {
              default = rustPackages.default.package;
            };

            # Rust dev environment
            devShells.default = pkgs.mkShell {
              inputsFrom = [
                config.treefmt.build.devShell
                rustDevShell
              ];
              nativeBuildInputs = with pkgs; [
                just
                tailwindcss
                cargo-leptos
                binaryen # Provides wasm-opt
              ];
            };
          };
      });
  };
}
