#[path = "../src/medium/refcell_counter.rs"]
mod refcell_counter;
use refcell_counter::Counter;


#[test]
fn test_increment() {
    let c = Counter::new(0);
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_initial_value() {
    let c = Counter::new(10);
    assert_eq!(c.get(), 10);
}

#[test]
fn test_many_increments() {
    let c = Counter::new(0);
    for _ in 0..100 {
        c.increment();
    }
    assert_eq!(c.get(), 100);
}

#[test]
fn test_zero_initial() {
    let c = Counter::new(0);
    assert_eq!(c.get(), 0);
}

#[test]
fn test_single_increment() {
    let c = Counter::new(5);
    c.increment();
    assert_eq!(c.get(), 6);
}

#[test]
fn test_negative_start() {
    let c = Counter::new(-5);
    c.increment();
    assert_eq!(c.get(), -4);
}

#[test]
fn test_increment_twice_from_zero() {
    let c = Counter::new(0);
    c.increment();
    c.increment();
    assert_eq!(c.get(), 2);
}

#[test]
fn test_immutable_borrow_allows_increment() {
    let c = Counter::new(0);
    let c_ref = &c;
    c_ref.increment();
    assert_eq!(c.get(), 1);
}

#[test]
fn test_large_start_value() {
    let c = Counter::new(1000);
    assert_eq!(c.get(), 1000);
}

#[test]
fn test_increments_by_one() {
    let c = Counter::new(42);
    c.increment();
    assert_eq!(c.get(), 43);
}
