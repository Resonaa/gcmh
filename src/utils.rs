use crate::constants::{NBK, USERNAME_REPLACERS, USER_ID_LENGTH};
use anyhow::Result;
use fake::{faker::name::en::Name, Fake};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::debug;
use rust_socketio::{client::Client, RawClient};
use std::{fmt::Write, iter::repeat_with};

/// Generates a random username.
///
/// # Examples
///
/// ```
/// let username = gcmh::utils::random_username();
///
/// println!("{}", username);
/// ```
pub fn random_username() -> String {
    Name()
        .fake::<String>()
        .replace(' ', fastrand::choice(&USERNAME_REPLACERS).unwrap())
}

/// Generates a random user_id.
///
/// # Examples
///
/// ```
/// let user_id = gcmh::utils::random_user_id();
///
/// println!("{}", user_id);
/// ```
pub fn random_user_id() -> String {
    repeat_with(fastrand::alphanumeric)
        .take(USER_ID_LENGTH)
        .collect()
}

macro_rules! strings {
    ($first: expr, $( $x: expr ),*) => {
        {
            let mut ans = String::from($first);
            $(
                ans.push_str(&format!("\",\"{}", $x));
            )*
            ans
        }
    };
}

pub fn set_username(socket: &Client, user_id: &str, username: &str) -> Result<()> {
    debug!("setting username {username}, user_id {user_id}");

    let data = strings!(username, user_id, NBK);

    socket.emit("set_username", data)?;

    Ok(())
}

pub fn join_private(socket: &RawClient, user_id: &str, room: &str) -> Result<()> {
    debug!("user_id {user_id}, joining {room}");

    let data = strings!(room, user_id, NBK);

    socket.emit("join_private", data)?;

    Ok(())
}

pub fn set_force_start(socket: &RawClient, room: &str) -> Result<()> {
    debug!("setting force start in {room}");

    let data = strings!(room, "true");

    socket.emit("set_force_start", data)?;

    Ok(())
}

pub fn set_custom_map(socket: &RawClient, room: &str, map: &str) -> Result<()> {
    debug!("setting custom map {map} in {room}");

    let data = format!("{room}\",{{\"map\":\"{map}\"}},\"");

    socket.emit("set_custom_options", data)?;

    Ok(())
}

pub fn set_speed(socket: &RawClient, room: &str, speed: f64) -> Result<()> {
    debug!("setting speed {speed} in {room}");

    let data = format!("{room}\",{{\"game_speed\":{speed}}},\"");

    socket.emit("set_custom_options", data)?;

    Ok(())
}

pub fn set_pb(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);

    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    pb.set_position(0);

    pb
}
