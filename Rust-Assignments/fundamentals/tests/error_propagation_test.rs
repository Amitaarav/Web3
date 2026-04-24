#[path = "../src/medium/error_propagation.rs"]
mod error_propagation;
use error_propagation::parse_and_add;


#[test]
fn test_valid() {
    assert_eq!(parse_and_add("10", "20"), Ok(30));
}

#[test]
fn test_first_invalid() {
    assert!(parse_and_add("abc", "5").is_err());
}

#[test]
fn test_second_invalid() {
    assert!(parse_and_add("5", "xyz").is_err());
}

#[test]
fn test_both_invalid() {
    assert!(parse_and_add("a", "b").is_err());
}

#[test]
fn test_negative() {
    assert_eq!(parse_and_add("-3", "7"), Ok(4));
}

#[test]
fn test_both_negative() {
    assert_eq!(parse_and_add("-5", "-5"), Ok(-10));
}

#[test]
fn test_zeros() {
    assert_eq!(parse_and_add("0", "0"), Ok(0));
}

#[test]
fn test_large_numbers() {
    assert_eq!(parse_and_add("1000", "2000"), Ok(3000));
}

#[test]
fn test_float_string_invalid() {
    assert!(parse_and_add("1.5", "2").is_err());
}

#[test]
fn test_empty_string_invalid() {
    assert!(parse_and_add("", "5").is_err());
}
