use crate::constants::{NBK, USERNAME_REPLACERS, USER_ID_LENGTH};
use anyhow::Result;
use fake::{faker::name::en::Name, Fake};
use log::debug;
use rust_socketio::client::Client;
use std::iter::repeat_with;

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
    debug!("sending {data}");

    socket.emit("set_username", data)?;

    Ok(())
}
