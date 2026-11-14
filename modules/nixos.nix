{ lib, config }:
let
  inherit (config) inputs;

  nixpkgsDeferred = lib.types.create {
    name = "NixpkgsDeferred";
    description = "Nixpkgs deferred module";
    check = with builtins; v: isAttrs v || isFunction v || isPath v;
    merge = loc: defs: {
      imports = map (def: {
        _file = "${def.__file__}; via ${lib.options.getIdentifier loc}";
        imports = [ def.value ];
      }) defs;
    };
  };
in
{
  options = {
    modules.nixos = lib.options.create {
      description = "NixOS modules which are available from this Nilla project.";
      type = lib.types.attrs.of nixpkgsDeferred;
      default.value = { };
    };

    systems.nixos = lib.options.create {
      description = "NixOS systems to create.";
      default.value = { };
      type = lib.types.attrs.of (lib.types.submodule ({ config }: {
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
            default.value = import (config.pkgs.path + "/nixos/lib/eval-config.nix") {
              system = null;
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

  config = {
    assertions = lib.attrs.mapToList
      (name: value: {
        assertion = !(builtins.isNull value.pkgs);
        message = "A Nixpkgs instance is required for the NixOS system \"${name}\", but none was provided and \"inputs.nixpkgs\" does not exist.";
      })
      config.systems.nixos;

    explain.modules.nixos = {
      name = "NixOS Modules";
      description = "NixOS modules which are available from this Nilla project.";

      data = {
        columns = [ "Name" ];
        rows = lib.attrs.mapToList (name: _: [ name ]) config.modules.nixos;
      };
    };
  };
}
