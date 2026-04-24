#[path = "../src/easy/slice_average.rs"]
mod slice_average;
use slice_average::average;


#[test]
fn test_basic() {
    assert_eq!(average(&[1.0, 2.0, 3.0]), Some(2.0));
}

#[test]
fn test_single() {
    assert_eq!(average(&[42.0]), Some(42.0));
}

#[test]
fn test_empty() {
    assert_eq!(average(&[]), None);
}

#[test]
fn test_negative() {
    let result = average(&[-2.0, 2.0]).unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_two_elements() {
    let result = average(&[3.0, 7.0]).unwrap();
    assert!((result - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_all_same() {
    let result = average(&[4.0, 4.0, 4.0]).unwrap();
    assert!((result - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_all_zeros() {
    let result = average(&[0.0, 0.0, 0.0]).unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_large_values() {
    let result = average(&[100.0, 200.0, 300.0]).unwrap();
    assert!((result - 200.0).abs() < f64::EPSILON);
}

#[test]
fn test_returns_some_for_nonempty() {
    assert!(average(&[1.0]).is_some());
}

#[test]
fn test_returns_none_for_empty() {
    assert!(average(&[]).is_none());
}
