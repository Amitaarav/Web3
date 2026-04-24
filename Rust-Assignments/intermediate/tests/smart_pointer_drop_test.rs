#[path = "../src/medium/smart_pointer_drop.rs"]
mod smart_pointer_drop;
use smart_pointer_drop::Logger;

use std::sync::{Arc, Mutex};

#[test]
fn test_drop() {
    let counter = Arc::new(Mutex::new(0));
    {
        let _l = Logger {
            name: "test".to_string(),
            drop_count: Arc::clone(&counter),
        };
        assert_eq!(*counter.lock().unwrap(), 0);
    }
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_multiple_drops() {
    let counter = Arc::new(Mutex::new(0));
    {
        let _l1 = Logger { name: "1".to_string(), drop_count: Arc::clone(&counter) };
        let _l2 = Logger { name: "2".to_string(), drop_count: Arc::clone(&counter) };
    }
    assert_eq!(*counter.lock().unwrap(), 2);
}

#[test]
fn test_not_dropped_while_in_scope() {
    let counter = Arc::new(Mutex::new(0));
    let _l = Logger { name: "live".to_string(), drop_count: Arc::clone(&counter) };
    assert_eq!(*counter.lock().unwrap(), 0);
}

#[test]
fn test_three_drops() {
    let counter = Arc::new(Mutex::new(0));
    {
        let _l1 = Logger { name: "a".to_string(), drop_count: Arc::clone(&counter) };
        let _l2 = Logger { name: "b".to_string(), drop_count: Arc::clone(&counter) };
        let _l3 = Logger { name: "c".to_string(), drop_count: Arc::clone(&counter) };
    }
    assert_eq!(*counter.lock().unwrap(), 3);
}

#[test]
fn test_counter_starts_at_zero() {
    let counter = Arc::new(Mutex::new(0));
    assert_eq!(*counter.lock().unwrap(), 0);
    let _l = Logger { name: "x".to_string(), drop_count: Arc::clone(&counter) };
}

#[test]
fn test_drop_in_nested_scope() {
    let counter = Arc::new(Mutex::new(0));
    {
        {
            let _l = Logger { name: "inner".to_string(), drop_count: Arc::clone(&counter) };
        }
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}

#[test]
fn test_name_field_stored() {
    let counter = Arc::new(Mutex::new(0));
    let l = Logger { name: "named".to_string(), drop_count: Arc::clone(&counter) };
    assert_eq!(l.name, "named");
}

#[test]
fn test_explicit_drop() {
    let counter = Arc::new(Mutex::new(0));
    let l = Logger { name: "explicit".to_string(), drop_count: Arc::clone(&counter) };
    drop(l);
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_counter_increments_sequentially() {
    let counter = Arc::new(Mutex::new(0));
    let l1 = Logger { name: "a".to_string(), drop_count: Arc::clone(&counter) };
    drop(l1);
    assert_eq!(*counter.lock().unwrap(), 1);
    let l2 = Logger { name: "b".to_string(), drop_count: Arc::clone(&counter) };
    drop(l2);
    assert_eq!(*counter.lock().unwrap(), 2);
}
