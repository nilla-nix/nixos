# 🍦 Nilla NixOS

> Work with NixOS systems in [Nilla](https://github.com/nilla-nix/nilla) projects with ease.

## Integration with Nilla CLI

Nilla NixOS integrates with the [Nilla CLI](https://github.com/nilla-nix/cli) through its external subcommand mechanism. When you run `nilla os`, the Nilla CLI looks for a binary named `nilla-os` in your PATH and executes it with the remaining arguments.

### How It Works

The Nilla CLI supports external subcommands via the `external_subcommand` mechanism. When you run `nilla <subcommand>`, it searches for a binary named `nilla-<subcommand>` in your PATH. Once `nilla-nixos` is installed (using one of the methods below), it will be available as `nilla-os` and can be used via the Nilla CLI.

### Commands

Once installed, you can use it via the Nilla CLI:

```bash
# Build a NixOS system
nilla nixos build <system_name>

# Build and switch to a configuration
nilla nixos switch <system_name>

# Test a configuration (activate without making it boot default)
nilla nixos test <system_name>

# Pass additional nixos-rebuild options
nilla nixos switch <system_name> -- --target-host user@remote
nilla nixos build <system_name> -- --build-host user@builder
```

## Install with Nilla

You can add Nilla NixOS to your Nilla project and access using the following code:

```nix
# In any module of your project.
{ config }:
let
    nilla-nixos-package = config.inputs.nilla-nixos.packages.nilla-nixos.x86_64-linux;
in
{
    config = {
        inputs.nilla-nixos.src = builtins.fetchTarball {
            url = "https://github.com/nilla-nix/nixos/archive/main.tar.gz";
            sha256 = "0000000000000000000000000000000000000000000000000000";
        };

        # Do something with the package.
    };
}
```

## Install without Flakes

You can install Nilla NixOS in your NixOS, home-manager, or nix-darwin configuration.

```nix
# configuration.nix
{ pkgs, ... }:
let
  nilla-nixos = import (builtins.fetchTarball {
    url = "https://github.com/nilla-nix/nixos/archive/main.tar.gz";
    sha256 = "0000000000000000000000000000000000000000000000000000";
  });
  nilla-nixos-package = nilla-nixos.packages.nilla-nixos.result.${pkgs.system};
in
{
  environment.systemPackages = [
    nilla-nixos-package
  ];
}
```

## Install with Flakes

You can add Nilla NixOS as a Flake input.

```nix
# flake.nix
{
  inputs = {
    nilla-nixos.url = "github:nilla-nix/nixos";
  };

  outputs = { nilla-nixos, ... }:
    let
      nilla-nixos-package = nilla-nixos.packages.x86_64-linux.nilla-nixos;
    in
      # Do something with the package.
      {};
}
```

## Run with Flakes

You can run Nilla NixOS directly via Flakes.

```bash
# Place any arguments you want to provide to Nilla NixOS after the --
nix run github:nilla-nix/nixos -- --help
```
