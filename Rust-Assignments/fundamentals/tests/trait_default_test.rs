#[path = "../src/medium/trait_default.rs"]
mod trait_default;
use trait_default::{Greet, Person};


#[test]
fn test_name() {
    let p = Person { name: "Alice".to_string() };
    assert_eq!(p.name(), "Alice");
}

#[test]
fn test_greeting() {
    let p = Person { name: "Bob".to_string() };
    assert_eq!(p.greeting(), "Hello, Bob!");
}

#[test]
fn test_empty_name() {
    let p = Person { name: "".to_string() };
    assert_eq!(p.greeting(), "Hello, !");
}

#[test]
fn test_name_charlie() {
    let p = Person { name: "Charlie".to_string() };
    assert_eq!(p.name(), "Charlie");
}

#[test]
fn test_greeting_charlie() {
    let p = Person { name: "Charlie".to_string() };
    assert_eq!(p.greeting(), "Hello, Charlie!");
}

#[test]
fn test_greeting_with_spaces() {
    let p = Person { name: "Jane Doe".to_string() };
    assert_eq!(p.greeting(), "Hello, Jane Doe!");
}

#[test]
fn test_name_single_char() {
    let p = Person { name: "X".to_string() };
    assert_eq!(p.name(), "X");
}

#[test]
fn test_greeting_single_char() {
    let p = Person { name: "X".to_string() };
    assert_eq!(p.greeting(), "Hello, X!");
}

#[test]
fn test_greeting_contains_name() {
    let p = Person { name: "Zara".to_string() };
    assert!(p.greeting().contains("Zara"));
}

#[test]
fn test_greeting_starts_with_hello() {
    let p = Person { name: "Alice".to_string() };
    assert!(p.greeting().starts_with("Hello,"));
}
