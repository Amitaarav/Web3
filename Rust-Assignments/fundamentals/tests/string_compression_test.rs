#[path = "../src/medium/string_compression.rs"]
mod string_compression;
use string_compression::compress;


#[test]
fn test_compression() {
    assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
}

#[test]
fn test_no_compression_needed() {
    assert_eq!(compress("abcdef"), "abcdef");
}

#[test]
fn test_empty() {
    assert_eq!(compress(""), "");
}

#[test]
fn test_single_char() {
    assert_eq!(compress("a"), "a");
}

#[test]
fn test_all_same() {
    assert_eq!(compress("aaaa"), "a4");
}

#[test]
fn test_tie_returns_original() {
    assert_eq!(compress("aa"), "aa");
}

#[test]
fn test_two_chars_returns_original() {
    assert_eq!(compress("ab"), "ab");
}

#[test]
fn test_multiple_groups() {
    assert_eq!(compress("aabb"), "a2b2");
}

#[test]
fn test_long_run() {
    assert_eq!(compress("zzzzzzzzzz"), "z10");
}

#[test]
fn test_alternating() {
    assert_eq!(compress("ababab"), "ababab");
}
