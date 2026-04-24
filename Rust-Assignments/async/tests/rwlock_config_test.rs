#[path = "../src/medium/rwlock_config.rs"]
mod rwlock_config;
use rwlock_config::update_and_read_config;


#[test]
fn test_config_value() {
    let final_val = update_and_read_config();
    assert!(!final_val.is_empty());
}

#[test]
fn test_thread_safety() {
    // Run multiple times to ensure no deadlocks or panics
    for _ in 0..5 {
        update_and_read_config();
    }
}

#[test]
fn test_returns_string() {
    let val: String = update_and_read_config();
    assert!(!val.is_empty());
}

#[test]
fn test_no_panic() {
    let _ = update_and_read_config();
}

#[test]
fn test_10_iterations_no_deadlock() {
    for _ in 0..10 {
        update_and_read_config();
    }
}

#[test]
fn test_result_is_deterministic() {
    let r1 = update_and_read_config();
    let r2 = update_and_read_config();
    assert_eq!(r1, r2);
}

#[test]
fn test_result_nonempty() {
    assert!(update_and_read_config().len() > 0);
}

#[test]
fn test_is_valid_string() {
    let result = update_and_read_config();
    assert!(result.is_ascii() || !result.is_empty());
}

#[test]
fn test_20_iterations() {
    for _ in 0..20 {
        let _ = update_and_read_config();
    }
}

#[test]
fn test_consistent_across_calls() {
    let results: Vec<String> = (0..3).map(|_| update_and_read_config()).collect();
    assert!(results.windows(2).all(|w| w[0] == w[1]));
}
