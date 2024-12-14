#![cfg(all(
    feature = "getters",
    feature = "setters",
    feature = "builder",
    feature = "repository",
    feature = "json"
))]
use mdd::{Builder, Getters, Json, Repository, Setters};
use uuid::Uuid;

#[derive(Builder, PartialEq, Json, Default, Getters, Setters, Clone, Repository)]
#[disk(path = "public/users", pretty = true, unique = email,id)]
pub struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: usize,
}

#[tokio::test]
pub async fn test_disk_insert_json_random_user() {
    let user = User::new()
        .id(Uuid::new_v4().to_string())
        .name("John Doe")
        .password("password123")
        .email(Uuid::new_v4().to_string())
        .age::<usize>(19)
        .build()
        .unwrap();
    let insert = user.disk_insert_json().await;
    use colorful::Colorful;
    match insert {
        Ok(_) => assert!(true),
        Err(err) => assert!(false, "Err({})", err.red()),
    };
}

#[tokio::test]
pub async fn test_disk_insert_json_unique_id() {
    let user = User::new()
        .id("3f8c9a1c-7d6a-4b2d-99f7-8dbd5402b6f1")
        .name("John Doe")
        .password("password123")
        .email(Uuid::new_v4().to_string())
        .age::<usize>(19)
        .build()
        .unwrap();
    let insert = user.disk_insert_json().await;

    match insert {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    };
}

#[tokio::test]
pub async fn test_disk_insert_json_unique_email() {
    let user = User::new()
        .id(Uuid::new_v4().to_string())
        .name("John Doe")
        .password("password123")
        .email("JohnDoe@gmail.com")
        .age::<usize>(19)
        .build()
        .unwrap();
    let insert = user.disk_insert_json().await;

    match insert {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    };
}
