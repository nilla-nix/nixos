use clap::{
    CommandFactory, Parser,
    builder::styling::{AnsiColor, Color::Ansi, Style},
};
use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, debug, error, trace};
use nixos_cli_def::{Cli, Commands, commands::completions};
use tokio;

const B: Style = Style::new().bold();
const D: Style = Style::new().dimmed();
const R: Style = Style::new().fg_color(Some(Ansi(AnsiColor::Red)));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::White)
        .debug(Color::Magenta)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    let cli = Cli::parse();
    let mut filter_level = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    if cli.quiet {
        filter_level = LevelFilter::Error;
    }

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "🍦 Nilla  {B}{}{B:#}  {}",
                colors.color(record.level()),
                match record.level() {
                    log::Level::Trace => format!("{D}{message}{D:#}"),
                    log::Level::Error => format!("{R}{message}{R:#}"),
                    _ => message.to_string(),
                }
            ));
        })
        .level(filter_level)
        .chain(
            fern::Dispatch::new()
                .filter(|f| f.level() == LevelFilter::Error)
                .chain(std::io::stderr()),
        )
        .chain(
            fern::Dispatch::new()
                .filter(|f| f.level() != LevelFilter::Error)
                .chain(std::io::stderr()),
        )
        .apply()?;

    trace!("Running {:?}", cli.command);

    match &cli.command {
        Some(command) => match command {
            Commands::Test(args) => nilla_nixos::commands::test::test_cmd(&cli, args).await,
            Commands::Switch(args) => nilla_nixos::commands::switch::switch_cmd(&cli, args).await,
            Commands::Build(args) => nilla_nixos::commands::build::build_cmd(&cli, args).await,
            Commands::Completions(args) => completions::completions_cmd(args, &mut Cli::command()),
            Commands::External(items) => debug!("got external subcommand: {items:?}"),
        },
        None => {
            error!("No subcommand found");
            println!("{}", Cli::command().render_long_help().to_string());
        }
    };
    Ok(())
}
