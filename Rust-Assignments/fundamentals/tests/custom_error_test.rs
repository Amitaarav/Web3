#[path = "../src/medium/custom_error.rs"]
mod custom_error;
use custom_error::{validate_username, ValidationError};


#[test]
fn test_valid() {
    assert_eq!(validate_username("rust_dev"), Ok(()));
}

#[test]
fn test_too_short() {
    assert_eq!(validate_username("ab"), Err(ValidationError::TooShort));
}

#[test]
fn test_too_long() {
    assert_eq!(validate_username(&"a".repeat(21)), Err(ValidationError::TooLong));
}

#[test]
fn test_invalid_char() {
    assert_eq!(validate_username("hello!"), Err(ValidationError::InvalidChar('!')));
}

#[test]
fn test_boundary_3() {
    assert_eq!(validate_username("abc"), Ok(()));
}

#[test]
fn test_boundary_20() {
    assert_eq!(validate_username(&"a".repeat(20)), Ok(()));
}

#[test]
fn test_with_underscore() {
    assert_eq!(validate_username("hello_world"), Ok(()));
}

#[test]
fn test_space_invalid() {
    assert!(validate_username("hello world").is_err());
}

#[test]
fn test_too_long_by_one() {
    assert_eq!(validate_username(&"a".repeat(21)), Err(ValidationError::TooLong));
}

#[test]
fn test_exactly_3_chars() {
    assert_eq!(validate_username("xyz"), Ok(()));
}
