pub mod constants;
pub mod search;
pub mod utils;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    /// Map name
    #[arg(short, long, default_value_t = String::from("1*1 Ultimate"))]
    pub map: String,

    /// Operations count
    #[arg(short, long, default_value_t = 10)]
    pub count: usize,

    /// Interval (ms) between two operations
    #[arg(short, long, default_value_t = 1606)]
    pub interval: u32,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for maps
    Search,

    /// Upvote a map
    Upvote,

    /// Play a map
    Play,
}
