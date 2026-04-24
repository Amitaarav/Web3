#[path = "../src/easy/hello_async.rs"]
mod hello_async;
use hello_async::hello_async;


#[tokio::test]
async fn test_hello() {
    assert_eq!(hello_async().await, "hello from tokio");
}

#[tokio::test]
async fn test_contains_tokio() {
    assert!(hello_async().await.contains("tokio"));
}

#[tokio::test]
async fn test_contains_hello() {
    assert!(hello_async().await.contains("hello"));
}

#[tokio::test]
async fn test_is_not_empty() {
    assert!(!hello_async().await.is_empty());
}

#[tokio::test]
async fn test_length() {
    assert_eq!(hello_async().await.len(), "hello from tokio".len());
}

#[tokio::test]
async fn test_starts_with_hello() {
    assert!(hello_async().await.starts_with("hello"));
}

#[tokio::test]
async fn test_ends_with_tokio() {
    assert!(hello_async().await.ends_with("tokio"));
}

#[tokio::test]
async fn test_consistent_across_calls() {
    assert_eq!(hello_async().await, hello_async().await);
}

#[tokio::test]
async fn test_returns_string() {
    let result: String = hello_async().await;
    assert_eq!(result, "hello from tokio");
}

#[tokio::test]
async fn test_lowercase() {
    let result = hello_async().await;
    assert_eq!(result, result.to_lowercase());
}
