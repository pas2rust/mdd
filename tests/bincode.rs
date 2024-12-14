#![cfg(all(
    feature = "getters",
    feature = "setters",
    feature = "builder",
    feature = "bytes"
))]

use mdd::{Builder, Bytes, Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Builder, PartialEq, Bytes, Serialize, Deserialize, Default, Getters, Setters, Clone,
)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub age: u8,
    pub friends: Vec<User>,
}

#[test]
fn to_bytes_and_from_bytes() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .friends(vec![])
        .build()
        .unwrap();
    let bytes = user.to_bytes().unwrap();
    let from_user = User::from_bytes(&bytes).unwrap();

    assert_eq!(user, from_user)
}
