# SPDX-FileCopyrightText: (C) 2025 chris montgomery <chmont@protonmail.com>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{
  perSystem =
    { commonArgs, craneLib, ... }:
    {
      packages.default = craneLib.buildPackage (
        commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        }
      );
    };
}
