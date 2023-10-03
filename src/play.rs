use crate::{
    constants::{GAME_SPEED, GAME_URL, SURRENDER_TURN, WS_URL},
    upvote::get,
    utils::{join_private, random_user_id, set_custom_map, set_force_start, set_speed},
    GameUpdate,
};
use anyhow::Result;
use log::{debug, info};
use rust_socketio::{ClientBuilder, Payload};
use serde_json::json;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn play(map: &str, count: usize) -> Result<()> {
    get(map)?;

    let (tx, rx) = mpsc::channel();

    let map_clone = map.to_string();
    let map_clone_2 = map.to_string();

    let room = random_user_id();
    let room_clone = room.clone();
    let room_clone_2 = room.clone();
    let room_clone_3 = room.clone();
    let room_clone_4 = room.clone();
    let room_clone_5 = room.clone();
    let room_clone_6 = room.clone();

    info!("bots playing map `{map}` at {GAME_URL}/{room}");

    let host_should_vote = Arc::new(Mutex::new(true));
    let host_should_vote_clone = host_should_vote.clone();

    let host_bot = ClientBuilder::new(WS_URL)
        .on("open", move |_, socket| {
            info!("host_bot connected");
            join_private(&socket, &random_user_id(), &room_clone).ok();
        })
        .on("queue_update", move |_, socket| {
            {
                let mut should_vote = host_should_vote.lock().unwrap();

                if !*should_vote {
                    return;
                }

                *should_vote = false;
            }

            set_custom_map(&socket, &room_clone_2, &map_clone)
                .and_then(|_| set_speed(&socket, &room_clone_2, GAME_SPEED))
                .and_then(|_| set_force_start(&socket, &room_clone_2))
                .ok();
        })
        .on("game_over", move |_, socket| {
            {
                *host_should_vote_clone.lock().unwrap() = true;
            }

            tx.send(()).unwrap();
            join_private(&socket, &random_user_id(), &room_clone_3).ok();
        })
        .connect()?;

    let surrender_should_vote = Arc::new(Mutex::new(true));
    let surrender_should_vote_clone = surrender_should_vote.clone();

    let surrender_bot = ClientBuilder::new(WS_URL)
        .on("open", move |_, socket| {
            info!("surrender_bot connected");
            join_private(&socket, &random_user_id(), &room_clone_4).ok();
        })
        .on("queue_update", move |_, socket| {
            {
                let mut should_vote = surrender_should_vote.lock().unwrap();

                if !*should_vote {
                    return;
                }

                *should_vote = false;
            }

            set_custom_map(&socket, &room_clone_5, &map_clone_2)
                .and_then(|_| set_speed(&socket, &room_clone_5, GAME_SPEED))
                .and_then(|_| set_force_start(&socket, &room_clone_5))
                .ok();
        })
        .on("game_update", move |payload, socket| {
            if let Payload::String(s) = payload {
                let data: GameUpdate = serde_json::from_str(&s).unwrap();
                debug!("{data:?}");
                if data.turn > SURRENDER_TURN {
                    socket.emit("surrender", json!(())).ok();
                }
            }
        })
        .on("game_over", move |_, socket| {
            {
                *surrender_should_vote_clone.lock().unwrap() = true;
            }
            thread::sleep(Duration::from_millis(300));
            join_private(&socket, &random_user_id(), &room_clone_6).ok();
        })
        .connect()?;

    let mut cnt = 0;

    while rx.recv().is_ok() {
        cnt += 1;
        info!("map played ({cnt}/{count})");

        if cnt >= count {
            debug!("shutting down...");
            host_bot.disconnect().unwrap();
            surrender_bot.disconnect().unwrap();
            break;
        }
    }

    info!("successfully played map for {count} times. Fetching new data...");

    get(map)?;

    Ok(())
}
