#[path = "../src/medium/word_counter.rs"]
mod word_counter;
use word_counter::word_count;


#[test]
fn test_basic() {
    let result = word_count("hello world hello");
    assert_eq!(result.get("hello"), Some(&2));
    assert_eq!(result.get("world"), Some(&1));
}

#[test]
fn test_case_insensitive() {
    let result = word_count("Hello HELLO hello");
    assert_eq!(result.get("hello"), Some(&3));
}

#[test]
fn test_empty() {
    let result = word_count("");
    assert!(result.is_empty());
}

#[test]
fn test_single_word() {
    let result = word_count("rust");
    assert_eq!(result.get("rust"), Some(&1));
    assert_eq!(result.len(), 1);
}

#[test]
fn test_all_unique() {
    let result = word_count("one two three");
    assert_eq!(result.len(), 3);
}

#[test]
fn test_missing_key() {
    let result = word_count("hello world");
    assert_eq!(result.get("rust"), None);
}

#[test]
fn test_uppercase_lowercased() {
    let result = word_count("RUST");
    assert_eq!(result.get("rust"), Some(&1));
}

#[test]
fn test_three_repeats() {
    let result = word_count("go go go");
    assert_eq!(result.get("go"), Some(&3));
}

#[test]
fn test_two_words_each_twice() {
    let result = word_count("a b a b");
    assert_eq!(result.get("a"), Some(&2));
    assert_eq!(result.get("b"), Some(&2));
}

#[test]
fn test_result_is_lowercase() {
    let result = word_count("Rust RUST rust");
    assert!(result.contains_key("rust"));
    assert!(!result.contains_key("Rust"));
}
