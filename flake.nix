{
  description = "Nixos support for Nilla.";

  outputs = inputs:
    let
      project = import ./nilla.nix;
    in
    {
      packages = {
        x86_64-linux = rec {
          nilla = project.packages.nilla-nixos.result.x86_64-linux;
          default = nilla;
        };
      };
    };
}
