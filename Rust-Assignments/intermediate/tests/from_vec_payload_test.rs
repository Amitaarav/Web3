#[path = "../src/medium/from_vec_payload.rs"]
mod from_vec_payload;
use from_vec_payload::Payload;


#[test]
fn test_conversion() {
    let input = vec![0x1234, 0x5678];
    let payload = Payload::from(input);
    assert_eq!(payload.data, vec![0x12, 0x34, 0x56, 0x78]);
}

#[test]
fn test_empty() {
    let payload = Payload::from(vec![]);
    assert!(payload.data.is_empty());
}

#[test]
fn test_single() {
    let payload = Payload::from(vec![0xFFFF]);
    assert_eq!(payload.data, vec![0xFF, 0xFF]);
}

#[test]
fn test_zero_value() {
    let payload = Payload::from(vec![0x0000]);
    assert_eq!(payload.data, vec![0x00, 0x00]);
}

#[test]
fn test_three_values() {
    let payload = Payload::from(vec![0xAABB, 0xCCDD, 0xEEFF]);
    assert_eq!(payload.data.len(), 6);
}

#[test]
fn test_high_byte() {
    let payload = Payload::from(vec![0xFF00]);
    assert_eq!(payload.data, vec![0xFF, 0x00]);
}

#[test]
fn test_low_byte() {
    let payload = Payload::from(vec![0x00FF]);
    assert_eq!(payload.data, vec![0x00, 0xFF]);
}

#[test]
fn test_data_length_is_twice_input() {
    let input = vec![1u16, 2, 3, 4];
    let payload = Payload::from(input);
    assert_eq!(payload.data.len(), 8);
}

#[test]
fn test_0x0102() {
    let payload = Payload::from(vec![0x0102]);
    assert_eq!(payload.data, vec![0x01, 0x02]);
}
