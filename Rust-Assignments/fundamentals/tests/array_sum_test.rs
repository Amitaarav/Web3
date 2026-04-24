#[path = "../src/easy/array_sum.rs"]
mod array_sum;
use array_sum::array_sum;


#[test]
fn test_positive() {
    assert_eq!(array_sum(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_zeros() {
    assert_eq!(array_sum(&[0, 0, 0, 0, 0]), 0);
}

#[test]
fn test_negative() {
    assert_eq!(array_sum(&[-1, -2, -3, -4, -5]), -15);
}

#[test]
fn test_mixed() {
    assert_eq!(array_sum(&[10, -10, 20, -20, 0]), 0);
}

#[test]
fn test_all_ones() {
    assert_eq!(array_sum(&[1, 1, 1, 1, 1]), 5);
}

#[test]
fn test_large() {
    assert_eq!(array_sum(&[100, 200, 300, 400, 500]), 1500);
}

#[test]
fn test_single_nonzero() {
    assert_eq!(array_sum(&[0, 0, 0, 0, 42]), 42);
}

#[test]
fn test_descending() {
    assert_eq!(array_sum(&[5, 4, 3, 2, 1]), 15);
}

#[test]
fn test_all_same() {
    assert_eq!(array_sum(&[7, 7, 7, 7, 7]), 35);
}

#[test]
fn test_large_negative() {
    assert_eq!(array_sum(&[-100, -200, -300, -400, -500]), -1500);
}
