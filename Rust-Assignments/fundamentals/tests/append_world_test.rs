#[path = "../src/easy/append_world.rs"]
mod append_world;
use append_world::append_world;


#[test]
fn test_hello() {
    assert_eq!(append_world(String::from("hello")), "hello world");
}

#[test]
fn test_empty() {
    assert_eq!(append_world(String::from("")), " world");
}

#[test]
fn test_existing() {
    assert_eq!(append_world(String::from("brave new")), "brave new world");
}

#[test]
fn test_single_word() {
    assert_eq!(append_world(String::from("rust")), "rust world");
}

#[test]
fn test_already_contains_world() {
    assert_eq!(append_world(String::from("hello world")), "hello world world");
}

#[test]
fn test_numbers() {
    assert_eq!(append_world(String::from("42")), "42 world");
}

#[test]
fn test_special_chars() {
    assert_eq!(append_world(String::from("foo!")), "foo! world");
}

#[test]
fn test_unicode() {
    assert_eq!(append_world(String::from("héllo")), "héllo world");
}

#[test]
fn test_multiple_spaces() {
    assert_eq!(append_world(String::from("foo  bar")), "foo  bar world");
}

#[test]
fn test_ends_with_space() {
    assert_eq!(append_world(String::from("hello ")), "hello  world");
}
