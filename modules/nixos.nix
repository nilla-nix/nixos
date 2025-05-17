{ config, lib }@global:
let
  inherit (global.config) inputs;
in
{
  options.systems = {
    nixos = lib.options.create {
      description = "NixOS systems to create.";
      default.value = { };
      type = lib.types.attrs.of (lib.types.submodule ({ config, name, ... }@submodule: {
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

          home-manager = lib.options.create {
            description = "The home-manager input to use.";
            type = lib.types.raw;
            default.value =
              if inputs ? home-manager
              then inputs.home-manager.result
              else null;
          };

          homes = lib.options.create {
            description = "A list of homes to activate for the system.";
            type = lib.types.list.of lib.types.string;
            default.value = [ ];
          };

          result = lib.options.create {
            description = "The created NixOS system.";
            type = lib.types.raw;
            writable = false;
            default.value = import "${submodule.config.pkgs.path}/nixos/lib/eval-config.nix" {
              pkgs = submodule.config.pkgs;
              lib = submodule.config.pkgs.lib;
              specialArgs = submodule.config.args;
              modules = submodule.config.modules;
              modulesLocation = null;
            };
          };
        };

        config = let
          homeModules = map (homeName: let
            system = submodule.config.pkgs.system;
            home = global.config.homes.${homeName};
            homeConfig = home.result.${system}.config;
            username = homeConfig.home.username;
            warn = builtins.warn or builtins.trace; # builtins.warn doesn't exist on some versions of nix/lix
            warnIf = condition: message: value: if condition then warn message value else value;
            homeManager = submodule.config.home-manager;
          in (
            ({ config, lib, ... }:
              warnIf (home.home-manager != homeManager) "The home \"${homeName}\" isn't using the same home-manager input as the NixOS system \"${name}\". This may work, but is not officially supported by the Nilla Home or Nilla NixOS maintainers. Please fix this before reporting any bugs you may find."
            {
              users.users.${username} = {
                isNormalUser = lib.modules.mkDefault true;
                home = homeConfig.home.homeDirectory;
              };

              home-manager.users.${username} = homeConfig;
            })
          )) submodule.config.homes;
        in {
          modules = (if submodule.config.homes != [] then [ submodule.config.home-manager.nixosModules.default ] else []) ++ homeModules;
        };
      }));
    };
  };

  config = {
    assertions = lib.lists.flatten (lib.attrs.mapToList
      (name: value: let
        hasNixpkgs = !(builtins.isNull value.pkgs);
        requestedHomes = value.homes != [];
        hasHomeManager = !(builtins.isNull value.home-manager);
      in [
        {
          assertion = hasNixpkgs;
          message = "A Nixpkgs instance is required for the NixOS system \"${name}\", but none was provided and \"inputs.nixpkgs\" does not exist.";
        }
        {
          assertion = !requestedHomes || hasHomeManager;
          message = "A home-manager instance is required to enable homes for the NixOS system \"${name}\", but none was provided and \"inputs.home-manager\" does not exist.";
        }
        (map (home: let
          hasHome = global.config ? homes.${home};
          homeHasHomeManager = !(builtins.isNull global.config.homes.${home}.home-manager);
          homeIsValidForSystem = global.config.homes.${home} ? result.${value.pkgs.system};
        in [
          {
            assertion = hasHome;
            message = "You've asked for the home \"${home}\" to be activated in the NixOS system \"${name}\", but it doesn't exist. Please set it up using [the Nilla Home plugin](https://github.com/nilla-nix/home).";
          }
          {
            assertion = !hasHome || homeHasHomeManager;
            message = "You've asked for the home \"${home}\" to be activated in the NixOS system \"${name}\", but it needs a home-manager instance, none was provided and \"inputs.home-manager\" does not exist.";
          }
          {
            assertion = !hasHome || !homeHasHomeManager || !hasNixpkgs || homeIsValidForSystem;
            message = "You've asked for the home \"${home}\" to be activated in the NixOS system \"${name}\", but it isn't valid for \"${value.pkgs.system}\" systems.";
          }
        ]) value.homes)
        (let
          usernames = map (home: let
            hasHome = global.config ? homes.${home};
            homeHasHomeManager = !(builtins.isNull global.config.homes.${home}.home-manager);
            homeIsValidForSystem = global.config.homes.${home} ? result.${value.pkgs.system};
          in 
            if hasHome && homeHasHomeManager && hasNixpkgs && homeIsValidForSystem
            then global.config.homes.${home}.result.${value.pkgs.system}.config.home.username
            else null) value.homes;
          existingUsernames = builtins.filter (username: username != null) usernames;
          uniqueUsernames = lib.lists.unique existingUsernames;
        in {
          assertion = !hasNixpkgs || (existingUsernames == uniqueUsernames);
          message = "There are multiple homes for a single user in the NixOS system \"${name}\". Please make sure you've only enabled a single home per user.";
        })
      ])
      global.config.systems.nixos);
  };
}
