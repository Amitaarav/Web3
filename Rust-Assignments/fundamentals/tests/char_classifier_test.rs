#[path = "../src/easy/char_classifier.rs"]
mod char_classifier;
use char_classifier::classify_char;


#[test]
fn test_alpha() {
    assert_eq!(classify_char('a'), "alphabetic");
}

#[test]
fn test_digit() {
    assert_eq!(classify_char('5'), "numeric");
}

#[test]
fn test_space() {
    assert_eq!(classify_char(' '), "whitespace");
}

#[test]
fn test_special() {
    assert_eq!(classify_char('@'), "other");
}

#[test]
fn test_uppercase() {
    assert_eq!(classify_char('Z'), "alphabetic");
}

#[test]
fn test_tab_whitespace() {
    assert_eq!(classify_char('\t'), "whitespace");
}

#[test]
fn test_newline_whitespace() {
    assert_eq!(classify_char('\n'), "whitespace");
}

#[test]
fn test_zero_digit() {
    assert_eq!(classify_char('0'), "numeric");
}

#[test]
fn test_nine_digit() {
    assert_eq!(classify_char('9'), "numeric");
}

#[test]
fn test_exclamation_other() {
    assert_eq!(classify_char('!'), "other");
}
