{ inputs, lib, ... }:
{
  perSystem =
    {
commonArgs,      config,
      inputs',
      pkgs,
      ...
    }:
    let
      inherit (commonArgs) rustChannel rustVersion;
      inherit (pkgs.rust-bin.${rustChannel}.${rustVersion}) rust-src;

      craneLib = inputs.crane.mkLib pkgs;

      nativeBuildInputs = [
        pkgs.pkg-config
      ];

      buildInputs = [
        pkgs.openssl
      ];

      commonPkgs =
        buildInputs
        ++ nativeBuildInputs
        ++ [
          inputs'.nixpkgs-trunk.legacyPackages.just
          pkgs.reuse
        ];

      checksPkgs = config.pre-commit.settings.enabledPackages ++ [
        pkgs.biome
      ];

      formatterPkgs = (lib.attrValues config.treefmt.build.programs) ++ [
        config.formatter
        config.treefmt.build.wrapper
      ];

      ciPkgs = commonPkgs ++ checksPkgs;
      devPkgs =
        commonPkgs
        ++ checksPkgs
        ++ formatterPkgs
        ++ [
          pkgs.rust-analyzer-unwrapped
        ];

      shellHook = ''
        : "''${PRJ_BIN_HOME:=''${PRJ_PATH:-''${PRJ_ROOT}/.bin}}"

        export PRJ_BIN_HOME

        ${config.pre-commit.installationScript}
      '';
    in
    {
      devShells.default = craneLib.devShell {
        inherit shellHook;
        packages = devPkgs;
        RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
      };

      devShells.ci = pkgs.mkShellNoCC { nativeBuildInputs = ciPkgs; };
    };
}
