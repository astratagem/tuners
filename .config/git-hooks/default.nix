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
