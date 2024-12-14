#![cfg(feature = "builder")]
use mdd::Builder;

#[derive(Builder, PartialEq, Default)]
pub struct User {
    #[pattern(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-5][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$")]
    pub id: String,
    #[pattern(r"^[a-zA-Z]{3,20}\s[a-zA-Z]{3,20}$")]
    pub name: String,
    #[pattern(r"^[a-zA-Z]{6,20}")]
    pub password: String,
    #[pattern(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")]
    //#[pattern_notify("email must be valid")]
    pub email: String,
    //#[min(18)]
    //#[min_notify("must be over 18 years old")]
    //#[max(30)]
    //#[min_notify("must be at most 30 years old")]
    //#[range("range(min(1, min), max(18, max))")]
    pub age: u8,
}

#[test]
fn builder() {
    let user = User::new()
        .id("123e4567-e89b-12d3-a456-426614174000")
        .name("John Doe")
        .password("password123")
        .email("johndoe@example.com")
        .age(18)
        .build()
        .unwrap();

    assert_eq!(user.id, "123e4567-e89b-12d3-a456-426614174000");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.password, "password123");
    assert_eq!(user.email, "johndoe@example.com");
    assert_eq!(user.age, 18);
}
