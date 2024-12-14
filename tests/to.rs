#![cfg(all(
    feature = "getters",
    feature = "setters",
    feature = "builder",
    feature = "json"
))]
use mdd::{Builder, Getters, Json, Setters, To};

#[derive(Builder, PartialEq, Json, Default, Getters, Setters, Clone, To, Debug)]
#[parser(key = id)]
pub struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: u8,
}

#[test]
fn to_hashmap() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();
    let hashmap = user.clone().to_hashmap();
    let expected_hashmap = std::collections::HashMap::from([(user.get_id(), user)]);
    println!("hash: {:#?}", hashmap);
    assert_eq!(hashmap, expected_hashmap);
}

#[test]
fn to_arc() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let arc = user.clone().to_arc();
    let expected_arc = std::sync::Arc::new(user.clone());

    assert_eq!(arc, expected_arc);
}

#[test]
fn to_box() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let boxed = user.clone().to_box();
    let expected_box = Box::new(user.clone());

    assert_eq!(boxed, expected_box);
}

#[test]
fn to_btree() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let btree = user.clone().to_btreemap();
    let expected_btree = std::collections::BTreeMap::from([(user.get_id(), user.clone())]);

    assert_eq!(btree, expected_btree);
}
