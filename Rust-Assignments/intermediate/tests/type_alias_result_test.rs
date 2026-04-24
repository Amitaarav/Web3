#[path = "../src/easy/type_alias_result.rs"]
mod type_alias_result;
use type_alias_result::{find_user, parse_age, AppError};


#[test]
fn test_find_user_ok() {
    assert_eq!(find_user(1), Ok("User_1".to_string()));
}

#[test]
fn test_find_user_not_found() {
    assert_eq!(find_user(0), Err(AppError::NotFound("User 0 not found".to_string())));
}

#[test]
fn test_parse_age_ok() {
    assert_eq!(parse_age("25"), Ok(25));
}

#[test]
fn test_parse_age_fail() {
    assert!(matches!(parse_age("abc"), Err(AppError::ParseFailed(_))));
}

#[test]
fn test_find_user_2() {
    assert_eq!(find_user(2), Ok("User_2".to_string()));
}

#[test]
fn test_find_user_large_id() {
    assert_eq!(find_user(100), Ok("User_100".to_string()));
}

#[test]
fn test_parse_age_zero() {
    assert_eq!(parse_age("0"), Ok(0));
}

#[test]
fn test_parse_age_negative_string() {
    assert!(matches!(parse_age("-1"), Err(AppError::ParseFailed(_))));
}

#[test]
fn test_parse_age_empty_string() {
    assert!(matches!(parse_age(""), Err(AppError::ParseFailed(_))));
}

#[test]
fn test_find_user_ok_is_ok() {
    assert!(find_user(5).is_ok());
}

#[test]
fn test_find_user_zero_is_err() {
    assert!(find_user(0).is_err());
}
