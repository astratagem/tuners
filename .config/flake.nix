# SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{
  description = "Private inputs for development purposes. These are used by the top level flake in the `dev` partition, but do not appear in consumers' lock files.";
  inputs = {
    nixpkgs-trunk.url = "github:NixOS/nixpkgs/master";
    git-hooks.url = "github:cachix/git-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    # See https://github.com/ursi/get-flake/issues/4
    git-hooks.inputs.nixpkgs.follows = "";
    treefmt-nix.inputs.nixpkgs.follows = "";
  };

  # This flake is only used for its inputs.
  outputs = { ... }: { };
}
