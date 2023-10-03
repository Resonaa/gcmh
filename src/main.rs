use anyhow::Result;
use clap::Parser;
use gcmh::{play::play, search::search, upvote::upvote, Cli, Commands};
use log::debug;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .format_timestamp(None)
        .init();

    debug!("cli parsed: {:?}", cli);

    match cli.commands {
        Commands::Search => {
            search(&cli.map, cli.count)?;
        }
        Commands::Upvote => {
            upvote(&cli.map, cli.count)?;
        }
        Commands::Play => {
            play(&cli.map, cli.count)?;
        }
    }

    Ok(())
}
