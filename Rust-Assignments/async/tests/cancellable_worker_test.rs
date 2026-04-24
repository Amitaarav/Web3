#[path = "../src/medium/cancellable_worker.rs"]
mod cancellable_worker;
use cancellable_worker::cancellable_worker;


#[test]
fn test_termination() {
    // If the worker doesn't terminate, this test will hang (and eventually time out)
    cancellable_worker();
}

#[test]
fn test_runs_at_least_once() {
    // This is more of a smoke test to ensure no panics
    cancellable_worker();
}

#[test]
fn test_no_panic_1() {
    cancellable_worker();
}

#[test]
fn test_no_panic_2() {
    cancellable_worker();
}

#[test]
fn test_terminates_quickly() {
    use std::time::Instant;
    let start = Instant::now();
    cancellable_worker();
    assert!(start.elapsed().as_secs() < 10);
}

#[test]
fn test_can_be_called_sequentially() {
    cancellable_worker();
    cancellable_worker();
}

#[test]
fn test_three_sequential_calls() {
    for _ in 0..3 {
        cancellable_worker();
    }
}

#[test]
fn test_does_not_hang() {
    cancellable_worker();
}

#[test]
fn test_returns_unit() {
    let result: () = cancellable_worker();
    let _ = result;
}

#[test]
fn test_five_calls() {
    for _ in 0..5 {
        cancellable_worker();
    }
}
