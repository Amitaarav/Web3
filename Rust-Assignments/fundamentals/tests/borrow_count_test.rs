#[path = "../src/easy/borrow_count.rs"]
mod borrow_count;
use borrow_count::count_above;


#[test]
fn test_basic() {
    assert_eq!(count_above(&vec![1, 5, 10, 15, 20], 10), 2);
}

#[test]
fn test_none_above() {
    assert_eq!(count_above(&vec![1, 2, 3], 10), 0);
}

#[test]
fn test_all_above() {
    assert_eq!(count_above(&vec![100, 200, 300], 0), 3);
}

#[test]
fn test_empty() {
    assert_eq!(count_above(&vec![], 5), 0);
}

#[test]
fn test_equal_to_threshold() {
    assert_eq!(count_above(&vec![10], 10), 0);
}

#[test]
fn test_single_above() {
    assert_eq!(count_above(&vec![11], 10), 1);
}

#[test]
fn test_negative_threshold() {
    assert_eq!(count_above(&vec![-5, 0, 5], -1), 2);
}

#[test]
fn test_large_threshold() {
    assert_eq!(count_above(&vec![1, 2, 3], 1000), 0);
}

#[test]
fn test_mixed_positives_negatives() {
    assert_eq!(count_above(&vec![-3, -1, 0, 1, 3], 0), 2);
}

#[test]
fn test_all_equal() {
    assert_eq!(count_above(&vec![5, 5, 5], 5), 0);
}
