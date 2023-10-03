use crate::{
    constants::{CONCURRENT_PLAYS, GAME_SPEED, GAME_URL, SURRENDER_TURN, WS_URL},
    upvote::get,
    utils::{join_private, random_user_id, set_custom_map, set_force_start, set_pb, set_speed},
    GameUpdate,
};
use anyhow::Result;
use log::{debug, info};
use rust_socketio::{ClientBuilder, Payload};
use serde_json::json;
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    },
    thread,
};

fn bot(room: &str, map: &str, should_report: bool, tx: Sender<()>) -> Result<()> {
    let room = room.to_owned();
    let room_clone = room.to_owned();
    let room_clone_2 = room.to_owned();

    let map = map.to_owned();

    let should_vote = Arc::new(Mutex::new(true));
    let should_vote_clone = should_vote.clone();
    let should_vote_clone_2 = should_vote.clone();

    ClientBuilder::new(WS_URL)
        .on("open", move |_, socket| {
            debug!("bot connected");
            join_private(&socket, &random_user_id(), &room).ok();
        })
        .on("close", move |_, _| {
            debug!("bot disconnected");
            *should_vote.lock().unwrap() = true;
        })
        .on("queue_update", move |_, socket| {
            {
                let mut should_vote = should_vote_clone.lock().unwrap();

                if !*should_vote {
                    return;
                }

                *should_vote = false;
            }

            set_custom_map(&socket, &room_clone, &map)
                .and_then(|_| set_speed(&socket, &room_clone, GAME_SPEED))
                .and_then(|_| set_force_start(&socket, &room_clone))
                .ok();
        })
        .on("game_update", move |payload, socket| {
            if let Payload::String(s) = payload {
                let data: GameUpdate = serde_json::from_str(&s).unwrap();
                if data.turn > SURRENDER_TURN {
                    socket.emit("surrender", json!(())).ok();
                }
            }
        })
        .on("game_over", move |_, socket| {
            {
                *should_vote_clone_2.lock().unwrap() = true;
            }

            if should_report && tx.send(()).is_err() {
                socket.disconnect().unwrap();
            }

            join_private(&socket, &random_user_id(), &room_clone_2).ok();
        })
        .connect()?;

    Ok(())
}

pub fn play(map: &str, count: u64) -> Result<()> {
    get(map)?;

    let (tx, rx) = mpsc::channel();

    info!("playing map `{map}` with speed {GAME_SPEED} and turns {SURRENDER_TURN}");

    let mut threads = Vec::new();

    for _ in 0..CONCURRENT_PLAYS {
        let map = map.to_owned();
        let tx = tx.clone();

        threads.push(thread::spawn(move || {
            let room = random_user_id();
            info!("starting a game at {GAME_URL}/{room}");
            bot(&room, &map, true, tx.clone()).unwrap();
            bot(&room, &map, false, tx).unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let pb = set_pb(count);

    let mut cnt = 0;

    while rx.recv().is_ok() {
        cnt += 1;
        pb.set_position(cnt);

        if cnt >= count {
            debug!("shutting down...");
            break;
        }
    }

    info!("successfully played map for {count} times. Fetching new data...");

    get(map)?;

    Ok(())
}
