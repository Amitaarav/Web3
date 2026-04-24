#[path = "../src/medium/serialization.rs"]
mod serialization;
use serialization::Record;


#[test]
fn test_roundtrip() {
    let r = Record { id: 42, value: 1000 };
    let bytes = r.to_bytes();
    assert_eq!(Record::from_bytes(&bytes), Ok(Record { id: 42, value: 1000 }));
}

#[test]
fn test_byte_length() {
    let r = Record { id: 1, value: 2 };
    assert_eq!(r.to_bytes().len(), 6);
}

#[test]
fn test_from_bytes_too_short() {
    assert!(Record::from_bytes(&[0, 1, 2]).is_err());
}

#[test]
fn test_zero_values() {
    let r = Record { id: 0, value: 0 };
    let bytes = r.to_bytes();
    assert_eq!(bytes, vec![0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_max_values() {
    let r = Record { id: u32::MAX, value: u16::MAX };
    let bytes = r.to_bytes();
    assert_eq!(Record::from_bytes(&bytes), Ok(r));
}

#[test]
fn test_empty_bytes_err() {
    assert!(Record::from_bytes(&[]).is_err());
}

#[test]
fn test_exact_6_bytes_ok() {
    let r = Record { id: 1, value: 1 };
    let bytes = r.to_bytes();
    assert_eq!(bytes.len(), 6);
    assert!(Record::from_bytes(&bytes).is_ok());
}

#[test]
fn test_id_preserved() {
    let r = Record { id: 99999, value: 1 };
    let bytes = r.to_bytes();
    let r2 = Record::from_bytes(&bytes).unwrap();
    assert_eq!(r2.id, 99999);
}

#[test]
fn test_value_preserved() {
    let r = Record { id: 1, value: 12345 };
    let bytes = r.to_bytes();
    let r2 = Record::from_bytes(&bytes).unwrap();
    assert_eq!(r2.value, 12345);
}

#[test]
fn test_from_bytes_7_bytes_err() {
    assert!(Record::from_bytes(&[0u8; 7]).is_err());
}
