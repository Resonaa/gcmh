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
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

fn bot(room: &str, map: &str, should_report: bool, tx: Sender<()>) -> Result<()> {
    let room = room.to_owned();
    let room_clone = room.to_owned();
    let room_clone_2 = room.to_owned();

    let map = map.to_owned();

    let should_vote = Arc::new(AtomicBool::new(true));
    let should_vote_clone = should_vote.clone();
    let should_vote_clone_2 = should_vote.clone();

    ClientBuilder::new(WS_URL)
        .on("open", move |_, socket| {
            debug!("bot connected");
            join_private(&socket, &random_user_id(), &room).ok();
        })
        .on("close", move |_, _| {
            debug!("bot disconnected");
            should_vote.store(true, Ordering::Relaxed);
        })
        .on("queue_update", move |_, socket| {
            if should_vote_clone
                .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                set_custom_map(&socket, &room_clone, &map)
                    .and_then(|_| set_speed(&socket, &room_clone, GAME_SPEED))
                    .and_then(|_| set_force_start(&socket, &room_clone))
                    .ok();
            }
        })
        .on("game_update", move |payload, socket| {
            if let Payload::String(s) = payload {
                let GameUpdate { turn } = serde_json::from_str(&s).unwrap();
                if turn >= SURRENDER_TURN {
                    socket.emit("surrender", json!(())).ok();
                }
            }
        })
        .on("game_over", move |_, socket| {
            should_vote_clone_2.store(true, Ordering::Relaxed);

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

    info!("playing map `{map}` with speed {GAME_SPEED} and turns {SURRENDER_TURN}...");

    for _ in 0..CONCURRENT_PLAYS {
        let map = map.to_owned();
        let tx = tx.clone();

        thread::spawn(move || {
            let room = random_user_id();
            debug!("starting a game at {GAME_URL}/{room}");
            bot(&room, &map, true, tx.clone()).unwrap();
            bot(&room, &map, false, tx).unwrap();
        });
    }

    let pb = set_pb(count);

    let mut cnt = 0;

    loop {
        if rx.try_recv().is_ok() {
            cnt += 1;
        }

        pb.set_position(cnt);

        if cnt >= count {
            debug!("shutting down...");
            break;
        }

        thread::sleep(Duration::from_millis(161));
    }

    info!("successfully played map for {count} times. Fetching new data...");

    get(map)?;

    Ok(())
}
