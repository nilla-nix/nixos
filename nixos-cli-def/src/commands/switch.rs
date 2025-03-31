use clap::Args;

#[derive(Debug, Args)]
#[command(about = "Run a package's main program")]
pub struct SwitchArgs {
    #[arg(help = "System name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
