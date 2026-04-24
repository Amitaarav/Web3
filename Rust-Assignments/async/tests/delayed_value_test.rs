#[path = "../src/easy/delayed_value.rs"]
mod delayed_value;
use delayed_value::delayed_value;

use std::time::Instant;

#[tokio::test]
async fn test_value() {
    assert_eq!(delayed_value(42).await, 42);
}

#[tokio::test]
async fn test_delay() {
    let start = Instant::now();
    delayed_value(1).await;
    assert!(start.elapsed().as_millis() >= 10);
}

#[tokio::test]
async fn test_zero() {
    assert_eq!(delayed_value(0).await, 0);
}

#[tokio::test]
async fn test_negative() {
    assert_eq!(delayed_value(-1).await, -1);
}

#[tokio::test]
async fn test_large() {
    assert_eq!(delayed_value(1000).await, 1000);
}

#[tokio::test]
async fn test_returns_exact_input() {
    assert_eq!(delayed_value(99).await, 99);
}

#[tokio::test]
async fn test_delay_is_at_least_10ms() {
    let start = Instant::now();
    delayed_value(0).await;
    assert!(start.elapsed().as_millis() >= 10);
}

#[tokio::test]
async fn test_multiple_calls_return_same() {
    let r1 = delayed_value(5).await;
    let r2 = delayed_value(5).await;
    assert_eq!(r1, r2);
}

#[tokio::test]
async fn test_negative_large() {
    assert_eq!(delayed_value(-100).await, -100);
}

#[tokio::test]
async fn test_value_unchanged() {
    let val = 42;
    assert_eq!(delayed_value(val).await, val);
}
