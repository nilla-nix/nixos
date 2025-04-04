use clap::Args;

#[derive(Debug, Args)]
#[command(about = "Build, install, and switch into a system")]
pub struct SwitchArgs {
    #[arg(help = "System name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
