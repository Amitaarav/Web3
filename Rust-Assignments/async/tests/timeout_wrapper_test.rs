#[path = "../src/medium/timeout_wrapper.rs"]
mod timeout_wrapper;
use timeout_wrapper::timeout_wrapper;

use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_success() {
    let f = async { sleep(Duration::from_millis(10)).await; 42 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(50)).await, Ok(42));
}

#[tokio::test]
async fn test_timeout() {
    let f = async { sleep(Duration::from_millis(100)).await; 42 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(10)).await, Err("Timeout".to_string()));
}

#[tokio::test]
async fn test_instant_computation() {
    let f = async { 0 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(100)).await, Ok(0));
}

#[tokio::test]
async fn test_returns_ok_for_fast_future() {
    let f = async { sleep(Duration::from_millis(1)).await; 10 };
    assert!(timeout_wrapper(f, Duration::from_millis(100)).await.is_ok());
}

#[tokio::test]
async fn test_returns_err_on_slow_future() {
    let f = async { sleep(Duration::from_millis(200)).await; 1 };
    assert!(timeout_wrapper(f, Duration::from_millis(10)).await.is_err());
}

#[tokio::test]
async fn test_err_message() {
    let f = async { sleep(Duration::from_millis(200)).await; 1 };
    let result = timeout_wrapper(f, Duration::from_millis(10)).await;
    assert_eq!(result, Err("Timeout".to_string()));
}

#[tokio::test]
async fn test_value_preserved() {
    let f = async { 99 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(100)).await, Ok(99));
}

#[tokio::test]
async fn test_large_timeout() {
    let f = async { sleep(Duration::from_millis(10)).await; 7 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(1000)).await, Ok(7));
}

#[tokio::test]
async fn test_consistent_ok() {
    for _ in 0..3 {
        let f = async { 42 };
        assert_eq!(timeout_wrapper(f, Duration::from_millis(100)).await, Ok(42));
    }
}

#[tokio::test]
async fn test_negative_value_ok() {
    let f = async { -5 };
    assert_eq!(timeout_wrapper(f, Duration::from_millis(100)).await, Ok(-5));
}
