{ lib, config }:
let
  inherit (config) inputs;
in
{
  options.systems = {
    nixos = lib.options.create {
      description = "NixOS systems to create.";
      default.value = { };
      type = lib.types.attrs.of (lib.types.submodule ({ config, name }: {
        options = {
          args = lib.options.create {
            description = "Additional arguments to pass to system modules.";
            type = lib.types.attrs.any;
            default.value = { };
          };

          pkgs = lib.options.create {
            description = "The Nixpkgs instance to use.";
            type = lib.types.raw;
            default.value =
              if
                inputs ? nixpkgs
                && inputs.nixpkgs.result ? x86_64-linux
              then
                inputs.nixpkgs.result.x86_64-linux
              else
                null;
          };

          modules = lib.options.create {
            description = "A list of modules to use for the system.";
            type = lib.types.list.of lib.types.raw;
            default.value = [ ];
          };

          result = lib.options.create {
            description = "The created NixOS system.";
            type = lib.types.raw;
            writable = false;
            default.value =
              if builtins.isNull config.pkgs then
                builtins.throw "A Nixpkgs instance is required for the NixOS system \"${name}\", but none was provided and \"inputs.nixpkgs\" does not exist."
              else import "${config.pkgs.path}/nixos/lib/eval-config.nix" {
                pkgs = config.pkgs;
                lib = config.pkgs.lib;
                specialArgs = config.args;
                modules = config.modules;
                modulesLocation = null;
              };
          };
        };
      }));
    };
  };
}
