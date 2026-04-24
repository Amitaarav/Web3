#[path = "../src/medium/running_sum.rs"]
mod running_sum;
use running_sum::running_sum;


#[test]
fn test_basic() {
    assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
}

#[test]
fn test_single() {
    assert_eq!(running_sum(vec![5]), vec![5]);
}

#[test]
fn test_empty() {
    assert_eq!(running_sum(vec![]), vec![]);
}

#[test]
fn test_negative() {
    assert_eq!(running_sum(vec![1, -1, 1, -1]), vec![1, 0, 1, 0]);
}

#[test]
fn test_all_zeros() {
    assert_eq!(running_sum(vec![0, 0, 0]), vec![0, 0, 0]);
}

#[test]
fn test_all_same() {
    assert_eq!(running_sum(vec![3, 3, 3]), vec![3, 6, 9]);
}

#[test]
fn test_descending() {
    assert_eq!(running_sum(vec![5, 4, 3, 2, 1]), vec![5, 9, 12, 14, 15]);
}

#[test]
fn test_last_element_is_total() {
    let v = vec![1, 2, 3, 4, 5];
    let result = running_sum(v);
    assert_eq!(*result.last().unwrap(), 15);
}

#[test]
fn test_length_preserved() {
    let v = vec![10, 20, 30];
    assert_eq!(running_sum(v).len(), 3);
}

#[test]
fn test_large_values() {
    assert_eq!(running_sum(vec![100, 200, 300]), vec![100, 300, 600]);
}
