/*
  Problem 55: Custom Error Enum (thiserror-style)

  Define an error enum DataError with variants InvalidLength { expected, actual },
  ChecksumMismatch, and Utf8Error(std::string::FromUtf8Error).
  Implement std::fmt::Display and std::error::Error manually.
  Write a function validate_packet that checks a &[u8] has exactly 10 bytes
  with a valid checksum (last byte = XOR of all previous bytes).

  Run the tests for this problem with:
    cargo test --test custom_error_enum_test
*/

use std::fmt;

#[derive(Debug)]
pub enum DataError {
    InvalidLength { expected: usize, actual: usize },
    ChecksumMismatch,
    Utf8Error(std::string::FromUtf8Error),
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for DataError {}

pub fn validate_packet(data: &[u8]) -> Result<(), DataError> {
    todo!()
}
