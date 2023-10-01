use crate::constants::MAP_SEARCH_API;
use anyhow::{Context, Result};
use futures_util::TryFutureExt;
use log::debug;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    table,
};
use serde::Deserialize;

/// Map info returned by generals.io API.
#[derive(Debug, Deserialize)]
pub struct MapInfo {
    pub score: f64,
    pub upvotes: usize,
    pub title: String,
    pub username: String,
}

pub async fn search(map: String, count: usize) -> Result<Vec<MapInfo>> {
    let client = reqwest::Client::new();
    let params = [("q", map)];

    debug!("search params: {:?}", params);

    client
        .get(MAP_SEARCH_API)
        .query(&params)
        .send()
        .and_then(|data| data.json::<Vec<MapInfo>>())
        .map_ok(|mut maps| {
            maps.truncate(count);
            maps
        })
        .await
        .with_context(|| "failed to search.")
}

pub fn print_search_results(maps: &Vec<MapInfo>) {
    if maps.is_empty() {
        println!("No results found.");
        return;
    }

    let format = FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[LinePosition::Top, LinePosition::Bottom],
            LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();

    for map in maps {
        let mut table = table!(
            ["Name", map.title],
            ["Author", map.username],
            ["Upvotes", map.upvotes],
            ["Score", map.score]
        );

        table.set_format(format);

        table.printstd();
    }
}
