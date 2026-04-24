#[path = "../src/easy/safe_divide.rs"]
mod safe_divide;
use safe_divide::safe_divide;


#[test]
fn test_normal_division() {
    assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
}

#[test]
fn test_divide_by_zero() {
    assert_eq!(safe_divide(10.0, 0.0), None);
}

#[test]
fn test_zero_dividend() {
    assert_eq!(safe_divide(0.0, 5.0), Some(0.0));
}

#[test]
fn test_negative() {
    assert_eq!(safe_divide(-10.0, 2.0), Some(-5.0));
}

#[test]
fn test_negative_divisor() {
    assert_eq!(safe_divide(10.0, -2.0), Some(-5.0));
}

#[test]
fn test_both_negative() {
    assert_eq!(safe_divide(-10.0, -2.0), Some(5.0));
}

#[test]
fn test_fractional_result() {
    let result = safe_divide(1.0, 3.0).unwrap();
    assert!((result - 1.0 / 3.0).abs() < 1e-10);
}

#[test]
fn test_large_values() {
    assert_eq!(safe_divide(1000.0, 100.0), Some(10.0));
}

#[test]
fn test_negative_zero_divisor() {
    assert_eq!(safe_divide(5.0, -0.0), None);
}

#[test]
fn test_returns_some_for_valid() {
    assert!(safe_divide(9.0, 3.0).is_some());
}
