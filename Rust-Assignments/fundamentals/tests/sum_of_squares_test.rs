#[path = "../src/easy/sum_of_squares.rs"]
mod sum_of_squares;
use sum_of_squares::sum_of_squares;


#[test]
fn test_basic() {
    assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
}

#[test]
fn test_single() {
    assert_eq!(sum_of_squares(&[5]), 25);
}

#[test]
fn test_empty() {
    assert_eq!(sum_of_squares(&[]), 0);
}

#[test]
fn test_negative() {
    assert_eq!(sum_of_squares(&[-3, 4]), 25);
}

#[test]
fn test_zeros() {
    assert_eq!(sum_of_squares(&[0, 0, 0]), 0);
}

#[test]
fn test_all_ones() {
    assert_eq!(sum_of_squares(&[1, 1, 1, 1]), 4);
}

#[test]
fn test_ten() {
    assert_eq!(sum_of_squares(&[10]), 100);
}

#[test]
fn test_two_elements() {
    assert_eq!(sum_of_squares(&[3, 4]), 25);
}

#[test]
fn test_all_same() {
    assert_eq!(sum_of_squares(&[2, 2, 2]), 12);
}

#[test]
fn test_large() {
    assert_eq!(sum_of_squares(&[10, 20]), 500);
}
