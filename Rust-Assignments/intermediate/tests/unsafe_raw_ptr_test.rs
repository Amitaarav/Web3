#[path = "../src/medium/unsafe_raw_ptr.rs"]
mod unsafe_raw_ptr;
use unsafe_raw_ptr::read_via_raw_pointer;


#[test]
fn test_basic() {
    let x = 42;
    assert_eq!(read_via_raw_pointer(&x), 42);
}

#[test]
fn test_negative() {
    let x = -7;
    assert_eq!(read_via_raw_pointer(&x), -7);
}

#[test]
fn test_zero() {
    let x = 0;
    assert_eq!(read_via_raw_pointer(&x), 0);
}

#[test]
fn test_large_positive() {
    let x = 1_000_000;
    assert_eq!(read_via_raw_pointer(&x), 1_000_000);
}

#[test]
fn test_large_negative() {
    let x = -1_000_000;
    assert_eq!(read_via_raw_pointer(&x), -1_000_000);
}

#[test]
fn test_one() {
    let x = 1;
    assert_eq!(read_via_raw_pointer(&x), 1);
}

#[test]
fn test_minus_one() {
    let x = -1;
    assert_eq!(read_via_raw_pointer(&x), -1);
}

#[test]
fn test_hundred() {
    let x = 100;
    assert_eq!(read_via_raw_pointer(&x), 100);
}

#[test]
fn test_does_not_modify_value() {
    let x = 55;
    let _ = read_via_raw_pointer(&x);
    assert_eq!(x, 55);
}
