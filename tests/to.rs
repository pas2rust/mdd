#![cfg(all(
    feature = "getters",
    feature = "setters",
    feature = "builder",
    feature = "to"
))]

use std::borrow::Cow;

use mdd::{Builder, Getters, Setters, To};

#[derive(Builder, PartialEq, Default, Getters, Setters, Clone, To, Debug, Hash, Eq)]
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

#[test]
fn to_vec() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let vec = user.clone().to_vec();
    let expected_vec = vec![user];

    assert_eq!(vec, expected_vec);
}

#[test]
fn to_vec_of_tuples() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let vec = user.clone().to_vec_of_tuples();
    let expected_vec = vec![(user.get_id(), user)];

    assert_eq!(vec, expected_vec);
}

#[test]
fn to_tuple() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let tuple = user.to_tuple();
    let expected_tuple = (&user,);
    assert_eq!(tuple, expected_tuple);
}

#[test]
fn to_cow() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    let cow_owned = user.clone().to_cow_owned();
    let user_clone = user.clone();
    let cow_borrowed = user_clone.to_cow_borrowed();
    let expected_cow_borrowed = Cow::Borrowed(&user);
    let expected_cow_owned = Cow::Owned(user.clone());

    assert_eq!(cow_owned, expected_cow_owned);
    assert_eq!(cow_borrowed, expected_cow_borrowed);
}
