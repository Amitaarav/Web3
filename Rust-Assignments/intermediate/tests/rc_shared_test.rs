#[path = "../src/medium/rc_shared.rs"]
mod rc_shared;
use rc_shared::shared_ownership;


#[test]
fn test_ref_count() {
    let (count, val) = shared_ownership("hello".to_string());
    assert_eq!(count, 3); // original + 2 clones
    assert_eq!(val, "hello");
}

#[test]
fn test_empty_string() {
    let (count, val) = shared_ownership("".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "");
}

#[test]
fn test_long_string() {
    let s = "a".repeat(1000);
    let (count, val) = shared_ownership(s.clone());
    assert_eq!(count, 3);
    assert_eq!(val, s);
}

#[test]
fn test_count_is_always_3() {
    let (count, _) = shared_ownership("any value".to_string());
    assert_eq!(count, 3);
}

#[test]
fn test_numeric_string() {
    let (count, val) = shared_ownership("12345".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "12345");
}

#[test]
fn test_unicode_string() {
    let (count, val) = shared_ownership("🦀 Rust".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "🦀 Rust");
}

#[test]
fn test_single_char() {
    let (count, val) = shared_ownership("x".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "x");
}

#[test]
fn test_value_preserved_exactly() {
    let s = "hello world test".to_string();
    let (_, val) = shared_ownership(s.clone());
    assert_eq!(val, s);
}

#[test]
fn test_multiple_calls_independent() {
    let (c1, _) = shared_ownership("a".to_string());
    let (c2, _) = shared_ownership("b".to_string());
    assert_eq!(c1, 3);
    assert_eq!(c2, 3);
}

#[test]
fn test_spaces_string() {
    let (count, val) = shared_ownership("   ".to_string());
    assert_eq!(count, 3);
    assert_eq!(val, "   ");
}
