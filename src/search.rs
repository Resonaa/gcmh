use crate::{constants::MAP_SEARCH_API, MapInfo};
use anyhow::{Context, Result};
use log::{debug, info};
use reqwest::blocking::Client;

pub fn search(map: &str, count: u64) -> Result<()> {
    info!("searching for {count} map(s) in keyword `{map}`");

    let client = Client::new();
    let params = [("q", map)];

    debug!("search params: {:?}", params);

    let maps = client
        .get(MAP_SEARCH_API)
        .query(&params)
        .send()
        .and_then(|data| data.json::<Vec<MapInfo>>())
        .map(|mut maps| {
            maps.truncate(count as usize);
            maps
        })
        .with_context(|| "failed to search.")?;

    info!("found {} map(s)", maps.len());

    for map in maps {
        print!("{map}");
    }

    Ok(())
}
