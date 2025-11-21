# SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{
  perSystem =
    { config, pkgs, ... }:
    {
      pre-commit.settings = {
        hooks = {
          biome.enable = true;
          markdownlint.enable = true;
          markdownlint.excludes = [
            # Auto-generated
            "CHANGELOG.md"
          ];
          reuse = {
            enable = true;
            stages = [ "pre-push" ];
          };
          treefmt.enable = true;
          yamllint.enable = true;
        };
        default_stages = [
          "pre-commit"
          "pre-push"
        ];
        excludes = [ ];
      };
    };
}
