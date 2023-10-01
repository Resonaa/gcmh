use anyhow::Result;
use clap::Parser;
use gcmh::{
    search::{print_search_results, search},
    Cli, Commands,
};
use log::debug;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .format_timestamp(None)
        .init();

    debug!("cli parsed: {:?}", cli);

    match cli.commands {
        Commands::Search => {
            let maps = search(cli.map, cli.count).await?;
            print_search_results(&maps);
        }
        _ => unimplemented!(),
    }

    Ok(())
}
