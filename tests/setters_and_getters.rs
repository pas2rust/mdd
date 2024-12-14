#![cfg(all(feature = "getters", feature = "setters", feature = "builder"))]

use mdd::{Builder, Getters, Setters};

#[derive(Builder, PartialEq, Default, Getters, Setters, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub email: String,
    pub age: u8,
    pub friends: Vec<User>,
}

#[test]
pub fn setters() {
    let friends = vec![User::default(); 10];
    let mut user_new = User::new()
        .id("id")
        .name("name")
        .password("password")
        .email("email")
        .age(18)
        .friends(friends.clone())
        .build()
        .unwrap();

    user_new.set_age(12);
    user_new.set_email("newemail");
    user_new.set_password("newpassword");
    user_new.set_name("newname");
    user_new.set_id("newid");
    assert_eq!(*user_new.get_ref_age(), 12);
    assert_eq!(*user_new.get_ref_email(), "newemail");
    assert_eq!(*user_new.get_ref_password(), "newpassword");
    assert_eq!(*user_new.get_ref_id(), "newid");
    assert_eq!(*user_new.get_ref_name(), "newname");
}

#[test]
pub fn getters_mut() {
    let friends = vec![User::default(); 10];
    let mut user_new = User::new()
        .id("id")
        .name("name")
        .password("password")
        .email("email")
        .age(18)
        .friends(friends.clone())
        .build()
        .unwrap();

    *user_new.get_mut_age() = 24;
    *user_new.get_mut_email() = "mutemail".to_string();
    *user_new.get_mut_password() = "mutpassword".to_string();
    *user_new.get_mut_name() = "mutname".to_string();
    *user_new.get_mut_id() = "mutid".to_string();
    assert_eq!(*user_new.get_ref_age(), 24);
    assert_eq!(*user_new.get_ref_email(), "mutemail");
    assert_eq!(*user_new.get_ref_password(), "mutpassword");
    assert_eq!(*user_new.get_ref_id(), "mutid");
    assert_eq!(*user_new.get_ref_name(), "mutname");
}
#[test]
pub fn getters_mut_setters() {
    let friends = vec![User::default(); 10];
    let mut user_new = User::new()
        .id("id")
        .name("name")
        .password("password")
        .email("email")
        .age(18)
        .friends(friends.clone())
        .build()
        .unwrap();
    let mut user_default = User::default();

    *user_new.get_mut_age() = 24;
    *user_new.get_mut_email() = "mut email".to_string();
    *user_new.get_mut_password() = "mut password".to_string();
    *user_new.get_mut_name() = "mut name".to_string();
    *user_new.get_mut_id() = "mut id".to_string();

    assert_eq!(*user_new.get_ref_age(), 24);
    assert_eq!(*user_new.get_ref_email(), "mut email");
    assert_eq!(*user_new.get_ref_password(), "mut password");
    assert_eq!(*user_new.get_ref_id(), "mut id");
    assert_eq!(*user_new.get_ref_name(), "mut name");

    *user_default.get_mut_age() = 24;
    *user_default.get_mut_email() = "mut email".to_string();
    *user_default.get_mut_password() = "mut password".to_string();
    *user_default.get_mut_name() = "mut name".to_string();
    *user_default.get_mut_id() = "mut id".to_string();

    assert_eq!(*user_default.get_ref_age(), 24);
    assert_eq!(*user_default.get_ref_email(), "mut email");
    assert_eq!(*user_default.get_ref_password(), "mut password");
    assert_eq!(*user_default.get_ref_id(), "mut id");
    assert_eq!(*user_default.get_ref_name(), "mut name");
}
