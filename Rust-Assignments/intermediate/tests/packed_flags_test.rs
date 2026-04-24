#[path = "../src/hard/packed_flags.rs"]
mod packed_flags;
use packed_flags::PackedFlags;


#[test]
fn test_initially_all_false() {
    let flags = PackedFlags::new();
    for i in 0..8 {
        assert!(!flags.get(i));
    }
}

#[test]
fn test_set_and_get() {
    let mut flags = PackedFlags::new();
    flags.set(3, true);
    assert!(flags.get(3));
    assert!(!flags.get(2));
}

#[test]
fn test_clear_flag() {
    let mut flags = PackedFlags::new();
    flags.set(5, true);
    flags.set(5, false);
    assert!(!flags.get(5));
}

#[test]
fn test_as_byte() {
    let mut flags = PackedFlags::new();
    flags.set(0, true);
    flags.set(7, true);
    assert_eq!(flags.as_byte(), 0b10000001);
}

#[test]
fn test_all_set() {
    let mut flags = PackedFlags::new();
    for i in 0..8 {
        flags.set(i, true);
    }
    assert_eq!(flags.as_byte(), 0xFF);
}

#[test]
fn test_set_bit_0() {
    let mut flags = PackedFlags::new();
    flags.set(0, true);
    assert!(flags.get(0));
}

#[test]
fn test_set_bit_7() {
    let mut flags = PackedFlags::new();
    flags.set(7, true);
    assert!(flags.get(7));
}

#[test]
fn test_clear_all() {
    let mut flags = PackedFlags::new();
    for i in 0..8 {
        flags.set(i, true);
    }
    for i in 0..8 {
        flags.set(i, false);
    }
    assert_eq!(flags.as_byte(), 0x00);
}

#[test]
fn test_set_does_not_affect_others() {
    let mut flags = PackedFlags::new();
    flags.set(4, true);
    assert!(!flags.get(3));
    assert!(!flags.get(5));
}

#[test]
fn test_as_byte_zero_initially() {
    let flags = PackedFlags::new();
    assert_eq!(flags.as_byte(), 0x00);
}
