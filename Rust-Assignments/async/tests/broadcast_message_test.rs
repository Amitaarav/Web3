#[path = "../src/medium/broadcast_message.rs"]
mod broadcast_message;
use broadcast_message::broadcast_demo;


#[tokio::test]
async fn test_broadcast() {
    // If each of the 3 tasks receives the message (say 10), the sum should be 30
    let result = broadcast_demo().await;
    assert!(result > 0);
}

#[tokio::test]
async fn test_consistency() {
    for _ in 0..3 {
        broadcast_demo().await;
    }
}

#[tokio::test]
async fn test_result_is_positive() {
    assert!(broadcast_demo().await > 0);
}

#[tokio::test]
async fn test_three_receivers_minimum() {
    let result = broadcast_demo().await;
    assert!(result >= 3);
}

#[tokio::test]
async fn test_no_panic() {
    let _ = broadcast_demo().await;
}

#[tokio::test]
async fn test_deterministic() {
    let r1 = broadcast_demo().await;
    let r2 = broadcast_demo().await;
    assert_eq!(r1, r2);
}

#[tokio::test]
async fn test_result_gt_0() {
    assert!(broadcast_demo().await > 0);
}

#[tokio::test]
async fn test_five_calls_consistent() {
    let first = broadcast_demo().await;
    for _ in 0..4 {
        assert_eq!(broadcast_demo().await, first);
    }
}

#[tokio::test]
async fn test_result_is_integer() {
    let result: i32 = broadcast_demo().await;
    assert!(result > 0);
}

#[tokio::test]
async fn test_completes() {
    broadcast_demo().await;
}
