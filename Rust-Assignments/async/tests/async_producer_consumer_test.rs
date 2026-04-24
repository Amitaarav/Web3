#[path = "../src/medium/async_producer_consumer.rs"]
mod async_producer_consumer;
use async_producer_consumer::async_producer_consumer;


#[tokio::test]
async fn test_count() {
    let result = async_producer_consumer().await;
    assert_eq!(result.len(), 10);
}

#[tokio::test]
async fn test_values() {
    let result = async_producer_consumer().await;
    assert!(result.contains(&1));
    assert!(result.contains(&10));
}

#[tokio::test]
async fn test_contains_5() {
    assert!(async_producer_consumer().await.contains(&5));
}

#[tokio::test]
async fn test_length_consistent() {
    assert_eq!(async_producer_consumer().await.len(), 10);
}

#[tokio::test]
async fn test_sum() {
    let sum: i32 = async_producer_consumer().await.iter().sum();
    assert_eq!(sum, 55); // 1+2+...+10
}

#[tokio::test]
async fn test_min_is_1() {
    let result = async_producer_consumer().await;
    assert_eq!(*result.iter().min().unwrap(), 1);
}

#[tokio::test]
async fn test_max_is_10() {
    let result = async_producer_consumer().await;
    assert_eq!(*result.iter().max().unwrap(), 10);
}

#[tokio::test]
async fn test_all_positive() {
    let result = async_producer_consumer().await;
    assert!(result.iter().all(|&x| x > 0));
}

#[tokio::test]
async fn test_no_duplicates() {
    let result = async_producer_consumer().await;
    let unique: std::collections::HashSet<_> = result.iter().collect();
    assert_eq!(unique.len(), result.len());
}

#[tokio::test]
async fn test_consistent() {
    let r1 = async_producer_consumer().await;
    let r2 = async_producer_consumer().await;
    let mut s1 = r1.clone(); s1.sort();
    let mut s2 = r2.clone(); s2.sort();
    assert_eq!(s1, s2);
}
