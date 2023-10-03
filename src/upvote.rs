use crate::{
    constants::{MAP_INFO_API, MAP_UPVOTE_API, WS_URL},
    utils::{random_user_id, random_username, set_pb, set_username},
    MapInfo,
};
use anyhow::{Context, Result};
use log::{debug, info, warn};
use rust_socketio::{ClientBuilder, Payload};
use serde_json::json;
use std::sync::mpsc;

pub fn get(map: &str) -> Result<MapInfo> {
    info!("searching for map `{map}`");

    let client = reqwest::blocking::Client::new();
    let params = [("name", map)];

    debug!("search params: {params:?}");

    let map = client
        .get(MAP_INFO_API)
        .query(&params)
        .send()
        .with_context(|| "failed to search.")?
        .json::<MapInfo>()
        .with_context(|| "map not found.")?;

    print!("{map}");

    Ok(map)
}

fn vote(map: &str, user_id: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let params = json!({"map": map, "user_id":user_id });

    debug!("vote params: {params}");

    client
        .post(MAP_UPVOTE_API)
        .json(&params)
        .send()
        .with_context(|| "failed to vote.")?;

    Ok(())
}

pub fn upvote(map: &str, count: u64) -> Result<()> {
    get(map)?;

    let pb = set_pb(count);

    for i in 1..=count {
        let (tx, rx) = mpsc::channel();

        let socket = ClientBuilder::new(WS_URL)
            .on("error_set_username", move |payload, _| {
                debug!("received payload: {payload:?}");

                if let Payload::String(s) = payload {
                    tx.send(s).unwrap();
                }
            })
            .connect()?;

        let mut username = random_username();
        let user_id = random_user_id();
        set_username(&socket, &username, &user_id)?;

        while let Ok(message) = rx.recv() {
            if &message == "\"\"" {
                socket.disconnect()?;
                break;
            }

            warn!("failed to set username. Retrying...");

            username = random_username();
            set_username(&socket, &user_id, &username)?;
        }

        debug!("username: {username}, user_id: {user_id}");

        vote(map, &user_id)?;

        pb.set_position(i);
    }

    info!("successfully voted map for {count} times. Fetching new data...");

    get(map)?;

    Ok(())
}
