#[path = "../src/hard/thread_pool.rs"]
mod thread_pool;
use thread_pool::ThreadPool;

use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn test_pool_execution() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    // Give some time for workers to complete
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(*counter.lock().unwrap(), 8);
}

#[test]
fn test_pool_size() {
    let pool = ThreadPool::new(3);
    assert_eq!(pool.workers.len(), 3);
}

#[test]
fn test_single_worker() {
    let pool = ThreadPool::new(1);
    assert_eq!(pool.workers.len(), 1);
}

#[test]
fn test_ten_workers_size() {
    let pool = ThreadPool::new(10);
    assert_eq!(pool.workers.len(), 10);
}

#[test]
fn test_single_task() {
    let pool = ThreadPool::new(2);
    let counter = Arc::new(Mutex::new(0));
    let c = Arc::clone(&counter);
    pool.execute(move || { *c.lock().unwrap() += 1; });
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_many_tasks() {
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..20 {
        let c = Arc::clone(&counter);
        pool.execute(move || { *c.lock().unwrap() += 1; });
    }
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(*counter.lock().unwrap(), 20);
}

#[test]
fn test_counter_accumulation() {
    let pool = ThreadPool::new(2);
    let sum = Arc::new(Mutex::new(0));
    for i in 1..=5 {
        let sum = Arc::clone(&sum);
        pool.execute(move || { *sum.lock().unwrap() += i; });
    }
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(*sum.lock().unwrap(), 15);
}

#[test]
fn test_pool_size_two() {
    let pool = ThreadPool::new(2);
    assert_eq!(pool.workers.len(), 2);
}

#[test]
fn test_pool_size_five() {
    let pool = ThreadPool::new(5);
    assert_eq!(pool.workers.len(), 5);
}

#[test]
fn test_consistency() {
    for _ in 0..3 {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(Mutex::new(0));
        for _ in 0..4 {
            let c = Arc::clone(&counter);
            pool.execute(move || { *c.lock().unwrap() += 1; });
        }
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(*counter.lock().unwrap(), 4);
    }
}
