use anyhow::Result;
use clap::Parser;
use gcmh::{play::play, search::search, upvote::upvote, Cli, Commands};

fn main() -> Result<()> {
    let Cli {
        commands,
        map,
        count,
        verbose,
    } = Cli::parse();

    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .format_timestamp(None)
        .init();

    match commands {
        Commands::Search => search(&map, count),
        Commands::Upvote => upvote(&map, count),
        Commands::Play => play(&map, count),
    }
}
