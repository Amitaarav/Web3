#[path = "../src/medium/collect_ticks.rs"]
mod collect_ticks;
use collect_ticks::collect_ticks;


#[tokio::test]
async fn test_count() {
    let result = collect_ticks().await;
    assert_eq!(result.len(), 5);
}

#[tokio::test]
async fn test_timing() {
    let result = collect_ticks().await;
    assert!(result[4] > result[0]);
}

#[tokio::test]
async fn test_strictly_increasing() {
    let result = collect_ticks().await;
    assert!(result.windows(2).all(|w| w[1] > w[0]));
}

#[tokio::test]
async fn test_first_tick_before_last() {
    let result = collect_ticks().await;
    assert!(result[0] < result[4]);
}

#[tokio::test]
async fn test_last_tick_largest() {
    let result = collect_ticks().await;
    assert_eq!(*result.iter().max().unwrap(), result[4]);
}

#[tokio::test]
async fn test_five_elements() {
    let result = collect_ticks().await;
    assert_eq!(result.len(), 5);
}

#[tokio::test]
async fn test_consistent_count() {
    for _ in 0..3 {
        assert_eq!(collect_ticks().await.len(), 5);
    }
}

#[tokio::test]
async fn test_second_gt_first() {
    let result = collect_ticks().await;
    assert!(result[1] > result[0]);
}

#[tokio::test]
async fn test_third_gt_second() {
    let result = collect_ticks().await;
    assert!(result[2] > result[1]);
}

#[tokio::test]
async fn test_no_duplicates() {
    let result = collect_ticks().await;
    let unique: std::collections::HashSet<_> = result.iter().collect();
    assert_eq!(unique.len(), result.len());
}
