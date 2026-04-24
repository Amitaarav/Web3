#[path = "../src/easy/oneshot_channel.rs"]
mod oneshot_channel;
use oneshot_channel::oneshot_demo;


#[tokio::test]
async fn test_oneshot() {
    assert_eq!(oneshot_demo().await, "done");
}

#[tokio::test]
async fn test_returns_string() {
    let result: String = oneshot_demo().await;
    assert_eq!(result, "done");
}

#[tokio::test]
async fn test_is_not_empty() {
    assert!(!oneshot_demo().await.is_empty());
}

#[tokio::test]
async fn test_consistent() {
    assert_eq!(oneshot_demo().await, oneshot_demo().await);
}

#[tokio::test]
async fn test_length() {
    assert_eq!(oneshot_demo().await.len(), 4);
}

#[tokio::test]
async fn test_starts_with_d() {
    assert!(oneshot_demo().await.starts_with('d'));
}

#[tokio::test]
async fn test_contains_done() {
    assert!(oneshot_demo().await.contains("done"));
}

#[tokio::test]
async fn test_multiple_calls() {
    for _ in 0..3 {
        assert_eq!(oneshot_demo().await, "done");
    }
}

#[tokio::test]
async fn test_ends_with_e() {
    assert!(oneshot_demo().await.ends_with('e'));
}

#[tokio::test]
async fn test_is_lowercase() {
    let r = oneshot_demo().await;
    assert_eq!(r, r.to_lowercase());
}
