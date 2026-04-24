#[path = "../src/medium/pipeline.rs"]
mod pipeline;
use pipeline::data_pipeline;


#[test]
fn test_sum() {
    // 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
    assert_eq!(data_pipeline(), 55);
}

#[test]
fn test_consistency() {
    for _ in 0..5 {
        assert_eq!(data_pipeline(), 55);
    }
}

#[test]
fn test_result_is_positive() {
    assert!(data_pipeline() > 0);
}

#[test]
fn test_result_gt_50() {
    assert!(data_pipeline() > 50);
}

#[test]
fn test_result_lt_100() {
    assert!(data_pipeline() < 100);
}

#[test]
fn test_deterministic() {
    let r1 = data_pipeline();
    let r2 = data_pipeline();
    assert_eq!(r1, r2);
}

#[test]
fn test_divisible_by_5() {
    assert_eq!(data_pipeline() % 5, 0);
}

#[test]
fn test_not_zero() {
    assert_ne!(data_pipeline(), 0);
}

#[test]
fn test_value_type() {
    let result: i32 = data_pipeline();
    assert_eq!(result, 55);
}

#[test]
fn test_10_consecutive_same() {
    let first = data_pipeline();
    for _ in 0..9 {
        assert_eq!(data_pipeline(), first);
    }
}
