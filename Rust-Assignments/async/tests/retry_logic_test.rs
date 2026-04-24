#[path = "../src/medium/retry_logic.rs"]
mod retry_logic;
use retry_logic::retry_operation;

use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_success_first_try() {
    let result: Result<i32, &str> = retry_operation(|| async { Ok(42) }, 3).await;
    assert_eq!(result, Ok(42));
}

#[tokio::test]
async fn test_retry_success() {
    let count = Arc::new(Mutex::new(0));
    let result: Result<i32, &str> = retry_operation(|| {
        let count = Arc::clone(&count);
        async move {
            let mut c = count.lock().unwrap();
            *c += 1;
            if *c < 3 { Err("fail") } else { Ok(100) }
        }
    }, 5).await;
    assert_eq!(result, Ok(100));
    assert_eq!(*count.lock().unwrap(), 3);
}

#[tokio::test]
async fn test_max_retries_exceeded() {
    let result: Result<i32, &str> = retry_operation(|| async { Err("always fail") }, 2).await;
    assert_eq!(result, Err("always fail"));
}

#[tokio::test]
async fn test_zero_retries() {
    let result: Result<i32, &str> = retry_operation(|| async { Err("fail") }, 0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_success_on_second_try() {
    let count = Arc::new(Mutex::new(0));
    let result: Result<i32, &str> = retry_operation(|| {
        let count = Arc::clone(&count);
        async move {
            let mut c = count.lock().unwrap();
            *c += 1;
            if *c < 2 { Err("fail") } else { Ok(99) }
        }
    }, 3).await;
    assert_eq!(result, Ok(99));
}

#[tokio::test]
async fn test_returns_ok_value() {
    let result: Result<i32, &str> = retry_operation(|| async { Ok(7) }, 3).await;
    assert_eq!(result.unwrap(), 7);
}

#[tokio::test]
async fn test_err_is_last_error() {
    let result: Result<i32, &str> = retry_operation(|| async { Err("last error") }, 2).await;
    assert_eq!(result, Err("last error"));
}

#[tokio::test]
async fn test_large_retry_count() {
    let result: Result<i32, &str> = retry_operation(|| async { Ok(1) }, 100).await;
    assert_eq!(result, Ok(1));
}

#[tokio::test]
async fn test_consistent_ok() {
    for _ in 0..3 {
        let result: Result<i32, &str> = retry_operation(|| async { Ok(5) }, 1).await;
        assert_eq!(result, Ok(5));
    }
}

#[tokio::test]
async fn test_one_retry_success_second() {
    let count = Arc::new(Mutex::new(0));
    let result: Result<i32, &str> = retry_operation(|| {
        let count = Arc::clone(&count);
        async move {
            let mut c = count.lock().unwrap();
            *c += 1;
            if *c == 1 { Err("first") } else { Ok(42) }
        }
    }, 2).await;
    assert_eq!(result, Ok(42));
}
