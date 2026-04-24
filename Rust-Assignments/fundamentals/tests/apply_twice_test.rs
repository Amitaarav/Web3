#[path = "../src/easy/apply_twice.rs"]
mod apply_twice;
use apply_twice::apply_twice;


#[test]
fn test_double() {
    assert_eq!(apply_twice(3, |x| x * 2), 12);
}

#[test]
fn test_increment() {
    assert_eq!(apply_twice(0, |x| x + 1), 2);
}

#[test]
fn test_square() {
    assert_eq!(apply_twice(2, |x| x * x), 16);
}

#[test]
fn test_identity() {
    assert_eq!(apply_twice(5, |x| x), 5);
}

#[test]
fn test_subtract() {
    assert_eq!(apply_twice(10, |x| x - 1), 8);
}

#[test]
fn test_negate() {
    assert_eq!(apply_twice(5, |x| -x), 5);
}

#[test]
fn test_add_ten() {
    assert_eq!(apply_twice(0, |x| x + 10), 20);
}

#[test]
fn test_triple() {
    assert_eq!(apply_twice(1, |x| x * 3), 9);
}

#[test]
fn test_negative_input() {
    assert_eq!(apply_twice(-3, |x| x + 1), -1);
}

#[test]
fn test_halve() {
    assert_eq!(apply_twice(100, |x| x / 2), 25);
}
