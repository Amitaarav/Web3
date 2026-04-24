#[path = "../src/medium/lifetime_longest.rs"]
mod lifetime_longest;
use lifetime_longest::longest;


#[test]
fn test_first_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn test_second_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn test_equal_length() {
    assert_eq!(longest("abc", "def"), "abc");
}

#[test]
fn test_empty_and_nonempty() {
    assert_eq!(longest("", "x"), "x");
}

#[test]
fn test_both_empty() {
    assert_eq!(longest("", ""), "");
}

#[test]
fn test_long_vs_short() {
    assert_eq!(longest("superlongstring", "ab"), "superlongstring");
}

#[test]
fn test_single_chars() {
    assert_eq!(longest("a", "b"), "a");
}

#[test]
fn test_with_spaces() {
    assert_eq!(longest("hello world", "hi"), "hello world");
}

#[test]
fn test_returns_reference() {
    let s1 = "rustlang".to_string();
    let s2 = "go";
    let result = longest(&s1, s2);
    assert_eq!(result, "rustlang");
}

#[test]
fn test_nonempty_and_empty_first() {
    assert_eq!(longest("hello", ""), "hello");
}
