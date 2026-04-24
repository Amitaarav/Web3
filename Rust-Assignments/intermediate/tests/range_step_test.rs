#[path = "../src/medium/range_step.rs"]
mod range_step;
use range_step::RangeStep;


#[test]
fn test_basic() {
    let v: Vec<i32> = RangeStep::new(0, 10, 3).collect();
    assert_eq!(v, vec![0, 3, 6, 9]);
}

#[test]
fn test_step_one() {
    let v: Vec<i32> = RangeStep::new(1, 5, 1).collect();
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_zero_step() {
    let v: Vec<i32> = RangeStep::new(0, 10, 0).collect();
    assert_eq!(v, vec![]);
}

#[test]
fn test_empty_range() {
    let v: Vec<i32> = RangeStep::new(10, 5, 1).collect();
    assert_eq!(v, vec![]);
}

#[test]
fn test_negative_step() {
    let v: Vec<i32> = RangeStep::new(10, 0, -3).collect();
    assert_eq!(v, vec![10, 7, 4, 1]);
}

#[test]
fn test_step_2() {
    let v: Vec<i32> = RangeStep::new(0, 8, 2).collect();
    assert_eq!(v, vec![0, 2, 4, 6]);
}

#[test]
fn test_single_element() {
    let v: Vec<i32> = RangeStep::new(5, 6, 1).collect();
    assert_eq!(v, vec![5]);
}

#[test]
fn test_large_step_skips_all() {
    let v: Vec<i32> = RangeStep::new(0, 5, 10).collect();
    assert_eq!(v, vec![0]);
}

#[test]
fn test_negative_step_count() {
    let v: Vec<i32> = RangeStep::new(5, 0, -1).collect();
    assert_eq!(v.len(), 5);
}

#[test]
fn test_start_equals_end() {
    let v: Vec<i32> = RangeStep::new(5, 5, 1).collect();
    assert_eq!(v, vec![]);
}
