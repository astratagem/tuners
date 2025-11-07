{
  perSystem =
    { ... }:
    {
      treefmt = {
        projectRootFile = ".git/config";
        programs = {
          biome.enable = true;
          nixfmt.enable = true;
          rustfmt.enable = true;
        };
      };
    };
}
