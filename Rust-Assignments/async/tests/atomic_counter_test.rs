#[path = "../src/medium/atomic_counter.rs"]
mod atomic_counter;
use atomic_counter::atomic_counter;


#[test]
fn test_final_value() {
    assert_eq!(atomic_counter(), 1000);
}

#[test]
fn test_consistency() {
    for _ in 0..10 {
        assert_eq!(atomic_counter(), 1000);
    }
}

#[test]
fn test_is_positive() {
    assert!(atomic_counter() > 0);
}

#[test]
fn test_gt_500() {
    assert!(atomic_counter() > 500);
}

#[test]
fn test_divisible_by_10() {
    assert_eq!(atomic_counter() % 10, 0);
}

#[test]
fn test_no_race_1() {
    assert_eq!(atomic_counter(), 1000);
}

#[test]
fn test_no_race_2() {
    assert_eq!(atomic_counter(), 1000);
}

#[test]
fn test_three_consecutive() {
    for _ in 0..3 {
        assert_eq!(atomic_counter(), 1000);
    }
}

#[test]
fn test_less_than_2000() {
    assert!(atomic_counter() < 2000);
}

#[test]
fn test_result_eq_1000() {
    let result = atomic_counter();
    assert_eq!(result, 1000);
}
