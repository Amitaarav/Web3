#[path = "../src/hard/parallel_word_count.rs"]
mod parallel_word_count;
use parallel_word_count::parallel_word_count;


#[test]
fn test_basic() {
    let lines = vec![
        "hello world".to_string(),
        "hello rust".to_string(),
        "parallel rust".to_string(),
    ];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
    assert_eq!(counts.get("parallel"), Some(&1));
}

#[test]
fn test_empty() {
    let counts = parallel_word_count(vec![]);
    assert!(counts.is_empty());
}

#[test]
fn test_single_line() {
    let lines = vec!["one two three".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.len(), 3);
}

#[test]
fn test_missing_word() {
    let lines = vec!["hello world".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("rust"), None);
}

#[test]
fn test_single_word_line() {
    let lines = vec!["rust".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("rust"), Some(&1));
}

#[test]
fn test_same_word_across_lines() {
    let lines = vec!["go".to_string(), "go".to_string(), "go".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("go"), Some(&3));
}

#[test]
fn test_two_lines_unique_words() {
    let lines = vec!["alpha".to_string(), "beta".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.len(), 2);
}

#[test]
fn test_count_is_accurate() {
    let lines = vec!["a a a".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("a"), Some(&3));
}

#[test]
fn test_empty_line_ignored() {
    let lines = vec!["".to_string(), "hello".to_string()];
    let counts = parallel_word_count(lines);
    assert_eq!(counts.get("hello"), Some(&1));
}
