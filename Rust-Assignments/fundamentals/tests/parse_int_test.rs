#[path = "../src/easy/parse_int.rs"]
mod parse_int;
use parse_int::parse_int;


#[test]
fn test_valid() {
    assert_eq!(parse_int("42"), Ok(42));
}

#[test]
fn test_negative() {
    assert_eq!(parse_int("-7"), Ok(-7));
}

#[test]
fn test_invalid() {
    assert!(parse_int("abc").is_err());
}

#[test]
fn test_empty() {
    assert!(parse_int("").is_err());
}

#[test]
fn test_float_string() {
    assert!(parse_int("3.14").is_err());
}

#[test]
fn test_zero() {
    assert_eq!(parse_int("0"), Ok(0));
}

#[test]
fn test_large_positive() {
    assert_eq!(parse_int("1000000"), Ok(1000000));
}

#[test]
fn test_large_negative() {
    assert_eq!(parse_int("-999"), Ok(-999));
}

#[test]
fn test_whitespace_invalid() {
    assert!(parse_int(" 5").is_err());
}

#[test]
fn test_overflow_string() {
    assert!(parse_int("99999999999999999999").is_err());
}
