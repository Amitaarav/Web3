#[path = "../src/easy/first_and_last.rs"]
mod first_and_last;
use first_and_last::first_and_last;


#[test]
fn test_normal() {
    assert_eq!(first_and_last(&[1, 2, 3, 4, 5]), Some((1, 5)));
}

#[test]
fn test_single() {
    assert_eq!(first_and_last(&[42]), Some((42, 42)));
}

#[test]
fn test_empty() {
    assert_eq!(first_and_last(&[]), None);
}

#[test]
fn test_two_elements() {
    assert_eq!(first_and_last(&[10, 20]), Some((10, 20)));
}

#[test]
fn test_negatives() {
    assert_eq!(first_and_last(&[-5, 0, 5]), Some((-5, 5)));
}

#[test]
fn test_all_same() {
    assert_eq!(first_and_last(&[3, 3, 3]), Some((3, 3)));
}

#[test]
fn test_large_slice() {
    let v: Vec<i32> = (1..=100).collect();
    assert_eq!(first_and_last(&v), Some((1, 100)));
}

#[test]
fn test_descending() {
    assert_eq!(first_and_last(&[10, 9, 8, 7]), Some((10, 7)));
}

#[test]
fn test_returns_some_for_nonempty() {
    assert!(first_and_last(&[1]).is_some());
}

#[test]
fn test_returns_none_for_empty() {
    assert!(first_and_last(&[]).is_none());
}
