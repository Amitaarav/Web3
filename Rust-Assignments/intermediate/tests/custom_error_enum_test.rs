#[path = "../src/medium/custom_error_enum.rs"]
mod custom_error_enum;
use custom_error_enum::{validate_packet, DataError};


#[test]
fn test_valid_packet() {
    let mut packet = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    packet[9] = packet[..9].iter().fold(0u8, |acc, &b| acc ^ b);
    assert!(validate_packet(&packet).is_ok());
}

#[test]
fn test_invalid_length() {
    let result = validate_packet(&[1, 2, 3]);
    assert!(matches!(result, Err(DataError::InvalidLength { .. })));
}

#[test]
fn test_checksum_mismatch() {
    let packet = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 0]; // bad checksum
    assert!(matches!(validate_packet(&packet), Err(DataError::ChecksumMismatch)));
}

#[test]
fn test_display() {
    let err = DataError::InvalidLength { expected: 10, actual: 5 };
    let msg = format!("{}", err);
    assert!(msg.contains("10") && msg.contains("5"));
}

#[test]
fn test_display_checksum() {
    let err = DataError::ChecksumMismatch;
    let msg = format!("{}", err);
    assert!(!msg.is_empty());
}

#[test]
fn test_too_long() {
    let result = validate_packet(&[0u8; 12]);
    assert!(matches!(result, Err(DataError::InvalidLength { .. })));
}

#[test]
fn test_empty_packet_invalid_length() {
    assert!(validate_packet(&[]).is_err());
}

#[test]
fn test_valid_returns_ok() {
    let mut packet = [0u8; 10];
    packet[9] = packet[..9].iter().fold(0u8, |acc, &b| acc ^ b);
    assert!(validate_packet(&packet).is_ok());
}

#[test]
fn test_invalid_length_has_expected() {
    let result = validate_packet(&[1, 2, 3]);
    if let Err(DataError::InvalidLength { expected, .. }) = result {
        assert_eq!(expected, 10);
    }
}

#[test]
fn test_invalid_length_has_actual() {
    let result = validate_packet(&[1, 2, 3]);
    if let Err(DataError::InvalidLength { actual, .. }) = result {
        assert_eq!(actual, 3);
    }
}
