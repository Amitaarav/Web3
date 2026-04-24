#[path = "../src/medium/shared_counter.rs"]
mod shared_counter;
use shared_counter::multithreaded_counter;


#[test]
fn test_final_value() {
    assert_eq!(multithreaded_counter(), 1000);
}

#[test]
fn test_consistency() {
    // Run multiple times to check for race conditions
    for _ in 0..10 {
        assert_eq!(multithreaded_counter(), 1000);
    }
}

#[test]
fn test_result_is_positive() {
    assert!(multithreaded_counter() > 0);
}

#[test]
fn test_result_gt_500() {
    assert!(multithreaded_counter() > 500);
}

#[test]
fn test_result_is_exact() {
    assert_eq!(multithreaded_counter(), 1000);
}

#[test]
fn test_no_race_condition_1() {
    assert_eq!(multithreaded_counter(), 1000);
}

#[test]
fn test_no_race_condition_2() {
    assert_eq!(multithreaded_counter(), 1000);
}

#[test]
fn test_divisible_by_10() {
    assert_eq!(multithreaded_counter() % 10, 0);
}

#[test]
fn test_consistent_three_runs() {
    let r1 = multithreaded_counter();
    let r2 = multithreaded_counter();
    let r3 = multithreaded_counter();
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
}

#[test]
fn test_result_lt_2000() {
    assert!(multithreaded_counter() < 2000);
}
