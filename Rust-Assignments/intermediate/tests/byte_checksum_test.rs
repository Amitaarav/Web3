#[path = "../src/easy/byte_checksum.rs"]
mod byte_checksum;
use byte_checksum::compute_checksum;


#[test]
fn test_basic() {
    assert_eq!(compute_checksum(&[1, 2, 3]), 0); // 1 ^ 2 ^ 3 = 0
}

#[test]
fn test_empty() {
    assert_eq!(compute_checksum(&[]), 0);
}

#[test]
fn test_single() {
    assert_eq!(compute_checksum(&[42]), 42);
}

#[test]
fn test_all_same() {
    assert_eq!(compute_checksum(&[255, 255]), 0);
}

#[test]
fn test_complex() {
    assert_eq!(compute_checksum(&[0xaa, 0xbb, 0xcc]), 0xdd); // 0xAA^0xBB^0xCC = 0xDD
}

#[test]
fn test_xor_with_itself() {
    assert_eq!(compute_checksum(&[0x5A, 0x5A]), 0);
}

#[test]
fn test_all_zeros() {
    assert_eq!(compute_checksum(&[0, 0, 0, 0]), 0);
}

#[test]
fn test_single_byte_0xff() {
    assert_eq!(compute_checksum(&[0xFF]), 0xFF);
}

#[test]
fn test_four_bytes() {
    assert_eq!(compute_checksum(&[1, 2, 4, 8]), 15); // 1^2^4^8 = 15
}

#[test]
fn test_order_independent() {
    assert_eq!(compute_checksum(&[1, 2, 3]), compute_checksum(&[3, 2, 1]));
}
