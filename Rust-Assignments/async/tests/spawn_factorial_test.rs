#[path = "../src/easy/spawn_factorial.rs"]
mod spawn_factorial;
use spawn_factorial::spawn_factorial;


#[tokio::test]
async fn test_factorial() {
    assert_eq!(spawn_factorial().await, 120);
}

#[tokio::test]
async fn test_result_is_positive() {
    assert!(spawn_factorial().await > 0);
}

#[tokio::test]
async fn test_consistent() {
    assert_eq!(spawn_factorial().await, spawn_factorial().await);
}

#[tokio::test]
async fn test_divisible_by_24() {
    assert_eq!(spawn_factorial().await % 24, 0);
}

#[tokio::test]
async fn test_divisible_by_5() {
    assert_eq!(spawn_factorial().await % 5, 0);
}

#[tokio::test]
async fn test_gt_100() {
    assert!(spawn_factorial().await > 100);
}

#[tokio::test]
async fn test_lt_200() {
    assert!(spawn_factorial().await < 200);
}

#[tokio::test]
async fn test_is_u64() {
    let result: u64 = spawn_factorial().await;
    assert_eq!(result, 120);
}

#[tokio::test]
async fn test_multiple_calls_same_result() {
    let r1 = spawn_factorial().await;
    let r2 = spawn_factorial().await;
    assert_eq!(r1, r2);
}

#[tokio::test]
async fn test_divisible_by_120() {
    assert_eq!(spawn_factorial().await % 120, 0);
}
