#[path = "../src/hard/fan_out_fan_in.rs"]
mod fan_out_fan_in;
use fan_out_fan_in::fan_out_fan_in;


#[tokio::test]
async fn test_square_sum() {
    assert_eq!(fan_out_fan_in(vec![1, 2, 3]).await, 14);
}

#[tokio::test]
async fn test_empty() {
    assert_eq!(fan_out_fan_in(vec![]).await, 0);
}

#[tokio::test]
async fn test_large() {
    let v = vec![1; 100];
    assert_eq!(fan_out_fan_in(v).await, 100);
}

#[tokio::test]
async fn test_single() {
    assert_eq!(fan_out_fan_in(vec![5]).await, 25);
}

#[tokio::test]
async fn test_two_elements() {
    assert_eq!(fan_out_fan_in(vec![3, 4]).await, 25);
}

#[tokio::test]
async fn test_zeros() {
    assert_eq!(fan_out_fan_in(vec![0, 0, 0]).await, 0);
}

#[tokio::test]
async fn test_negatives() {
    assert_eq!(fan_out_fan_in(vec![-2, -3]).await, 13);
}

#[tokio::test]
async fn test_result_non_negative() {
    assert!(fan_out_fan_in(vec![1, -1, 2]).await >= 0);
}

#[tokio::test]
async fn test_consistent() {
    let r1 = fan_out_fan_in(vec![1, 2, 3]).await;
    let r2 = fan_out_fan_in(vec![1, 2, 3]).await;
    assert_eq!(r1, r2);
}

#[tokio::test]
async fn test_single_two() {
    assert_eq!(fan_out_fan_in(vec![2]).await, 4);
}
