#[path = "../src/easy/threaded_sum.rs"]
mod threaded_sum;
use threaded_sum::threaded_sum;


#[test]
fn test_even_length() {
    assert_eq!(threaded_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_odd_length() {
    assert_eq!(threaded_sum(vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_empty() {
    assert_eq!(threaded_sum(vec![]), 0);
}

#[test]
fn test_large() {
    let v = vec![1; 1000];
    assert_eq!(threaded_sum(v), 1000);
}

#[test]
fn test_single_element() {
    assert_eq!(threaded_sum(vec![42]), 42);
}

#[test]
fn test_all_zeros() {
    assert_eq!(threaded_sum(vec![0, 0, 0, 0]), 0);
}

#[test]
fn test_all_same() {
    assert_eq!(threaded_sum(vec![5; 10]), 50);
}

#[test]
fn test_two_elements() {
    assert_eq!(threaded_sum(vec![10, 20]), 30);
}

#[test]
fn test_negative_values() {
    assert_eq!(threaded_sum(vec![-1, -2, -3]), -6);
}

#[test]
fn test_mixed_values() {
    assert_eq!(threaded_sum(vec![1, -1, 2, -2, 3]), 3);
}
