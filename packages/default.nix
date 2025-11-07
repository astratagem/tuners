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
