#[path = "../src/easy/extract_nibbles.rs"]
mod extract_nibbles;
use extract_nibbles::extract_nibbles;


#[test]
fn test_ab() {
    assert_eq!(extract_nibbles(0xAB), (0x0A, 0x0B));
}

#[test]
fn test_zero() {
    assert_eq!(extract_nibbles(0x00), (0x00, 0x00));
}

#[test]
fn test_ff() {
    assert_eq!(extract_nibbles(0xFF), (0x0F, 0x0F));
}

#[test]
fn test_f0() {
    assert_eq!(extract_nibbles(0xF0), (0x0F, 0x00));
}

#[test]
fn test_0f() {
    assert_eq!(extract_nibbles(0x0F), (0x00, 0x0F));
}

#[test]
fn test_12() {
    assert_eq!(extract_nibbles(0x12), (0x01, 0x02));
}

#[test]
fn test_high_nibble() {
    let (high, _) = extract_nibbles(0xAB);
    assert_eq!(high, 0x0A);
}

#[test]
fn test_low_nibble() {
    let (_, low) = extract_nibbles(0xAB);
    assert_eq!(low, 0x0B);
}

#[test]
fn test_both_nibbles_4_bits() {
    let (high, low) = extract_nibbles(0xFF);
    assert!(high <= 0x0F && low <= 0x0F);
}

#[test]
fn test_cd() {
    assert_eq!(extract_nibbles(0xCD), (0x0C, 0x0D));
}
