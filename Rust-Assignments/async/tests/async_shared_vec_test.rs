#[path = "../src/medium/async_shared_vec.rs"]
mod async_shared_vec;
use async_shared_vec::async_shared_vec;


#[tokio::test]
async fn test_length() {
    assert_eq!(async_shared_vec().await, 50);
}

#[tokio::test]
async fn test_consistency() {
    for _ in 0..5 {
        assert_eq!(async_shared_vec().await, 50);
    }
}

#[tokio::test]
async fn test_positive() {
    assert!(async_shared_vec().await > 0);
}

#[tokio::test]
async fn test_equals_50() {
    assert_eq!(async_shared_vec().await, 50);
}

#[tokio::test]
async fn test_not_zero() {
    assert_ne!(async_shared_vec().await, 0);
}

#[tokio::test]
async fn test_gt_40() {
    assert!(async_shared_vec().await > 40);
}

#[tokio::test]
async fn test_lt_100() {
    assert!(async_shared_vec().await < 100);
}

#[tokio::test]
async fn test_three_calls_same() {
    let r1 = async_shared_vec().await;
    let r2 = async_shared_vec().await;
    let r3 = async_shared_vec().await;
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
}

#[tokio::test]
async fn test_divisible_by_5() {
    assert_eq!(async_shared_vec().await % 5, 0);
}

#[tokio::test]
async fn test_result_type() {
    let result: usize = async_shared_vec().await;
    assert_eq!(result, 50);
}
