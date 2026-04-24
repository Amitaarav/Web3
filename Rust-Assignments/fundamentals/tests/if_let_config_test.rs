#[path = "../src/easy/if_let_config.rs"]
mod if_let_config;
use if_let_config::get_config;


#[test]
fn test_some() {
    assert_eq!(get_config(Some("custom".to_string())), "custom");
}

#[test]
fn test_none() {
    assert_eq!(get_config(None), "default");
}

#[test]
fn test_empty_string() {
    assert_eq!(get_config(Some("".to_string())), "");
}

#[test]
fn test_some_single_char() {
    assert_eq!(get_config(Some("x".to_string())), "x");
}

#[test]
fn test_some_whitespace() {
    assert_eq!(get_config(Some(" ".to_string())), " ");
}

#[test]
fn test_some_long_value() {
    assert_eq!(get_config(Some("hello world foo bar".to_string())), "hello world foo bar");
}

#[test]
fn test_some_numbers() {
    assert_eq!(get_config(Some("42".to_string())), "42");
}

#[test]
fn test_some_unicode() {
    assert_eq!(get_config(Some("café".to_string())), "café");
}

#[test]
fn test_some_special_chars() {
    assert_eq!(get_config(Some("!@#$".to_string())), "!@#$");
}

#[test]
fn test_none_default_len() {
    assert_eq!(get_config(None).len(), "default".len());
}
