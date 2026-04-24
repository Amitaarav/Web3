#[path = "../src/medium/set_clear_toggle.rs"]
mod set_clear_toggle;
use set_clear_toggle::{clear_bit, set_bit, toggle_bit};


#[test]
fn test_set_bit() {
    assert_eq!(set_bit(0b0000, 2), 0b0100);
}

#[test]
fn test_set_already_set() {
    assert_eq!(set_bit(0b0100, 2), 0b0100);
}

#[test]
fn test_clear_bit() {
    assert_eq!(clear_bit(0b1111, 1), 0b1101);
}

#[test]
fn test_toggle_on() {
    assert_eq!(toggle_bit(0b0000, 3), 0b1000);
}

#[test]
fn test_toggle_off() {
    assert_eq!(toggle_bit(0b1000, 3), 0b0000);
}

#[test]
fn test_clear_already_clear() {
    assert_eq!(clear_bit(0b0000, 2), 0b0000);
}

#[test]
fn test_set_bit_0() {
    assert_eq!(set_bit(0b0000, 0), 0b0001);
}

#[test]
fn test_clear_bit_0() {
    assert_eq!(clear_bit(0b1111, 0), 0b1110);
}

#[test]
fn test_toggle_twice_identity() {
    let v = toggle_bit(toggle_bit(0b1010, 2), 2);
    assert_eq!(v, 0b1010);
}

#[test]
fn test_set_multiple_bits() {
    let v = set_bit(set_bit(0b0000, 0), 1);
    assert_eq!(v, 0b0011);
}
