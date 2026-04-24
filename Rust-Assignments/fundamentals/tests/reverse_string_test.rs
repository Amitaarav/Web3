#[path = "../src/easy/reverse_string.rs"]
mod reverse_string;
use reverse_string::reverse_string;


#[test]
fn test_hello() {
    assert_eq!(reverse_string("hello"), "olleh");
}

#[test]
fn test_empty() {
    assert_eq!(reverse_string(""), "");
}

#[test]
fn test_single() {
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn test_palindrome() {
    assert_eq!(reverse_string("racecar"), "racecar");
}

#[test]
fn test_unicode() {
    assert_eq!(reverse_string("rust 🦀"), "🦀 tsur");
}

#[test]
fn test_two_chars() {
    assert_eq!(reverse_string("ab"), "ba");
}

#[test]
fn test_spaces() {
    assert_eq!(reverse_string("  "), "  ");
}

#[test]
fn test_numbers() {
    assert_eq!(reverse_string("123"), "321");
}

#[test]
fn test_mixed() {
    assert_eq!(reverse_string("abc123"), "321cba");
}

#[test]
fn test_double_reverse_identity() {
    let s = "hello world";
    assert_eq!(reverse_string(reverse_string(s).as_str()), s);
}
