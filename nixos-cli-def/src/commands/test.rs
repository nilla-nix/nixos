use clap::Args;

#[derive(Debug, Args)]
#[command(about = "Start a development shell from a Nilla project")]
pub struct TestArgs {
    #[arg(help = "System name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
