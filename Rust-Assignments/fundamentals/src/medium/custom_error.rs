/*
  Problem 26: Custom Error Type

  Define a custom error enum ValidationError with variants TooShort, TooLong,
  and InvalidChar(char). Write a function that validates a username:
  must be 3–20 characters and only contain alphanumeric chars or underscores.
  Return Ok(()) or the appropriate error.

  Run the tests for this problem with:
    cargo test --test custom_error_test
*/

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort,
    TooLong,
    InvalidChar(char),
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    todo!()
}
