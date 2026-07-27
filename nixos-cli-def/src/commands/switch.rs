use clap::Args;

#[derive(Debug, Args)]
#[command(
    about = "Build, install, and switch into a system",
    long_about = "Build, install, and switch into a system. Additional nixos-rebuild options can be passed after --, e.g.: nilla nixos switch -- --build-host user@remote --target-host user@target"
)]
pub struct SwitchArgs {
    #[arg(help = "System name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, help = "Additional arguments to pass to nixos-rebuild")]
    pub extra_nixos_rebuild_args: Vec<String>,
}
