#[path = "../src/medium/producer_consumer.rs"]
mod producer_consumer;
use producer_consumer::producer_consumer;


#[test]
fn test_collection() {
    let result = producer_consumer();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_length() {
    let result = producer_consumer();
    assert_eq!(result.len(), 5);
}

#[test]
fn test_contains_1() {
    assert!(producer_consumer().contains(&1));
}

#[test]
fn test_contains_5() {
    assert!(producer_consumer().contains(&5));
}

#[test]
fn test_first_element() {
    assert_eq!(producer_consumer()[0], 1);
}

#[test]
fn test_last_element() {
    let r = producer_consumer();
    assert_eq!(r[r.len() - 1], 5);
}

#[test]
fn test_sum() {
    let sum: i32 = producer_consumer().iter().sum();
    assert_eq!(sum, 15);
}

#[test]
fn test_is_sorted() {
    let r = producer_consumer();
    assert!(r.windows(2).all(|w| w[0] <= w[1]));
}

#[test]
fn test_no_duplicates() {
    let r = producer_consumer();
    let unique: std::collections::HashSet<_> = r.iter().collect();
    assert_eq!(unique.len(), r.len());
}

#[test]
fn test_consistent() {
    assert_eq!(producer_consumer(), producer_consumer());
}
