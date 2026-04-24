#[path = "../src/hard/manual_future.rs"]
mod manual_future;
use manual_future::ReadyFuture;


#[tokio::test]
async fn test_ready() {
    let f = ReadyFuture { value: Some(42) };
    assert_eq!(f.await, 42);
}

#[tokio::test]
async fn test_string() {
    let f = ReadyFuture { value: Some("ready".to_string()) };
    assert_eq!(f.await, "ready");
}

#[tokio::test]
async fn test_zero() {
    let f = ReadyFuture { value: Some(0) };
    assert_eq!(f.await, 0);
}

#[tokio::test]
async fn test_negative() {
    let f = ReadyFuture { value: Some(-1) };
    assert_eq!(f.await, -1);
}

#[tokio::test]
async fn test_large_value() {
    let f = ReadyFuture { value: Some(99999) };
    assert_eq!(f.await, 99999);
}

#[tokio::test]
async fn test_empty_string() {
    let f = ReadyFuture { value: Some("".to_string()) };
    assert_eq!(f.await, "");
}

#[tokio::test]
async fn test_bool_true() {
    let f = ReadyFuture { value: Some(true) };
    assert_eq!(f.await, true);
}

#[tokio::test]
async fn test_bool_false() {
    let f = ReadyFuture { value: Some(false) };
    assert_eq!(f.await, false);
}

#[tokio::test]
async fn test_vec() {
    let f = ReadyFuture { value: Some(vec![1, 2, 3]) };
    assert_eq!(f.await, vec![1, 2, 3]);
}

#[tokio::test]
async fn test_consistent() {
    let f1 = ReadyFuture { value: Some(7) };
    assert_eq!(f1.await, 7);
}
