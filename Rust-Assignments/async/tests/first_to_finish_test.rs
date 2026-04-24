#[path = "../src/medium/first_to_finish.rs"]
mod first_to_finish;
use first_to_finish::first_to_finish;

use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_first_wins() {
    let f1 = async { sleep(Duration::from_millis(50)).await; 1 };
    let f2 = async { sleep(Duration::from_millis(10)).await; 2 };
    // Box and pin them to satisfy Unpin requirement for select!
    let f1 = Box::pin(f1);
    let f2 = Box::pin(f2);
    assert_eq!(first_to_finish(f1, f2).await, 2);
}

#[tokio::test]
async fn test_other_wins() {
    let f1 = async { sleep(Duration::from_millis(5)).await; 10 };
    let f2 = async { sleep(Duration::from_millis(100)).await; 20 };
    let f1 = Box::pin(f1);
    let f2 = Box::pin(f2);
    assert_eq!(first_to_finish(f1, f2).await, 10);
}

#[tokio::test]
async fn test_returns_i32() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(10)).await; 0 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(50)).await; 1 });
    let result: i32 = first_to_finish(f1, f2).await;
    assert_eq!(result, 0);
}

#[tokio::test]
async fn test_zero_value() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(1)).await; 0 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(200)).await; 99 });
    assert_eq!(first_to_finish(f1, f2).await, 0);
}

#[tokio::test]
async fn test_negative_value() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(1)).await; -5 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(200)).await; 5 });
    assert_eq!(first_to_finish(f1, f2).await, -5);
}

#[tokio::test]
async fn test_faster_is_second() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(200)).await; 111 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(5)).await; 222 });
    assert_eq!(first_to_finish(f1, f2).await, 222);
}

#[tokio::test]
async fn test_large_value() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(1)).await; 9999 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(200)).await; 1 });
    assert_eq!(first_to_finish(f1, f2).await, 9999);
}

#[tokio::test]
async fn test_result_not_the_slower_one() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(1)).await; 42 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(100)).await; 99 });
    assert_ne!(first_to_finish(f1, f2).await, 99);
}

#[tokio::test]
async fn test_both_same_value() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(5)).await; 7 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(50)).await; 7 });
    assert_eq!(first_to_finish(f1, f2).await, 7);
}

#[tokio::test]
async fn test_consistent() {
    let f1 = Box::pin(async { sleep(Duration::from_millis(1)).await; 100 });
    let f2 = Box::pin(async { sleep(Duration::from_millis(200)).await; 200 });
    assert_eq!(first_to_finish(f1, f2).await, 100);
}
