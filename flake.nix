# SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{
  description = "Tuners";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{
      flake-parts,
      nixpkgs,
      ...
    }:
    let
      systems = import inputs.systems;

      # For details on these options, See
      # https://github.com/oxalica/rust-overlay?tab=readme-ov-file#cheat-sheet-common-usage-of-rust-bin
      #
      # Channel of the Rust toolchain (stable or beta).
      rustChannel = "stable";
      # Version (latest or specific date/semantic version)
      rustVersion = "latest";
      # Profile (default or minimal)
      rustProfile = "default";
    in
    flake-parts.lib.mkFlake { inherit inputs; } {
      inherit systems;

      imports = [
        inputs.flake-parts.flakeModules.modules
        inputs.flake-parts.flakeModules.partitions
      ];

      perSystem =
        { system, pkgs, ... }:
        let
          commonArgs = {
            inherit
              src
              rustChannel
              rustProfile
              rustVersion
              ;
          };

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
            pkgs: pkgs.rust-bin.${rustChannel}.${rustVersion}.${rustProfile}
          );

          # Depending on your code base, you may have to customize the
          # source filtering to include non-standard files during the build.
          # https://crane.dev/source-filtering.html?highlight=source#source-filtering
          src = craneLib.cleanCargoSource (craneLib.path ./.);
        in
        {
          _module.args = {
            inherit commonArgs craneLib;
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ inputs.rust-overlay.overlays.default ];
            };
          };
          formatter = pkgs.nixfmt-rfc-style;
        };

      partitions.dev = {
        extraInputsFlake = ./.config;
        module =
          { inputs, ... }:
          {
            imports = [
              inputs.git-hooks.flakeModule
              inputs.treefmt-nix.flakeModule
              ./.config/devshells
              ./.config/git-hooks.nix
              ./.config/treefmt.nix
            ];
          };
      };

      partitionedAttrs = {
        checks = "dev";
        devShells = "dev";
      };
    };
}
