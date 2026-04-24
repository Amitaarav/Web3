#[path = "../src/easy/concurrent_compute.rs"]
mod concurrent_compute;
use concurrent_compute::compute_concurrently;


async fn task1() -> i32 { 10 }
async fn task2() -> i32 { 20 }

#[tokio::test]
async fn test_sum() {
    assert_eq!(compute_concurrently(task1(), task2()).await, 30);
}

#[tokio::test]
async fn test_with_delays() {
    use tokio::time::{sleep, Duration};
    let f1 = async { sleep(Duration::from_millis(10)).await; 5 };
    let f2 = async { sleep(Duration::from_millis(5)).await; 5 };
    assert_eq!(compute_concurrently(f1, f2).await, 10);
}

#[tokio::test]
async fn test_zeros() {
    assert_eq!(compute_concurrently(async { 0 }, async { 0 }).await, 0);
}

#[tokio::test]
async fn test_negatives() {
    assert_eq!(compute_concurrently(async { -5 }, async { -5 }).await, -10);
}

#[tokio::test]
async fn test_one_zero() {
    assert_eq!(compute_concurrently(async { 0 }, async { 42 }).await, 42);
}

#[tokio::test]
async fn test_large_values() {
    assert_eq!(compute_concurrently(async { 1000 }, async { 2000 }).await, 3000);
}

#[tokio::test]
async fn test_commutative() {
    let r1 = compute_concurrently(async { 3 }, async { 7 }).await;
    let r2 = compute_concurrently(async { 7 }, async { 3 }).await;
    assert_eq!(r1, r2);
}

#[tokio::test]
async fn test_single_contributions() {
    assert_eq!(compute_concurrently(async { 1 }, async { 1 }).await, 2);
}

#[tokio::test]
async fn test_mixed_signs() {
    assert_eq!(compute_concurrently(async { 10 }, async { -10 }).await, 0);
}

#[tokio::test]
async fn test_both_positive() {
    assert!(compute_concurrently(async { 5 }, async { 5 }).await > 0);
}
