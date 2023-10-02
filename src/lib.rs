pub mod constants;
pub mod search;
pub mod upvote;
pub mod utils;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    table,
};
use serde::Deserialize;
use std::fmt::Display;

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

/// Map info returned by generals.io API.
#[derive(Debug, Deserialize)]
pub struct MapInfo {
    pub score: f64,
    pub upvotes: usize,
    pub title: String,

    #[serde(default)]
    pub username: String,
}

impl Display for MapInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = FormatBuilder::new()
            .borders('|')
            .separators(
                &[LinePosition::Top, LinePosition::Bottom],
                LineSeparator::new('-', '+', '+', '+'),
            )
            .padding(1, 1)
            .build();

        let mut table = table!(
            ["Name", self.title],
            ["Author", self.username],
            ["Upvotes", self.upvotes],
            ["Score", self.score]
        );

        table.set_format(format);

        table.fmt(f)
    }
}
