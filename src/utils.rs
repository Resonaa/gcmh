use crate::constants::{USERNAME_REPLACERS, USER_ID_LENGTH};
use fake::{faker::name::en::Name, Fake};
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
