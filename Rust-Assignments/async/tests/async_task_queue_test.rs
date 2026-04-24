#[path = "../src/hard/async_task_queue.rs"]
mod async_task_queue;
use async_task_queue::TaskQueue;

use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_queue_execution() {
    let queue = TaskQueue::new(2);
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        queue.push(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }).await;
    }

    sleep(Duration::from_millis(50)).await;
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[tokio::test]
async fn test_single_task() {
    let queue = TaskQueue::new(1);
    let counter = Arc::new(Mutex::new(0));
    let c = Arc::clone(&counter);
    queue.push(move || { *c.lock().unwrap() += 1; }).await;
    sleep(Duration::from_millis(50)).await;
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[tokio::test]
async fn test_ten_tasks() {
    let queue = TaskQueue::new(4);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        queue.push(move || { *counter.lock().unwrap() += 1; }).await;
    }
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 10);
}

#[tokio::test]
async fn test_counter_starts_zero() {
    let counter = Arc::new(Mutex::new(0));
    assert_eq!(*counter.lock().unwrap(), 0);
}

#[tokio::test]
async fn test_single_worker() {
    let queue = TaskQueue::new(1);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        queue.push(move || { *counter.lock().unwrap() += 1; }).await;
    }
    sleep(Duration::from_millis(50)).await;
    assert_eq!(*counter.lock().unwrap(), 3);
}

#[tokio::test]
async fn test_large_pool() {
    let queue = TaskQueue::new(8);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        queue.push(move || { *counter.lock().unwrap() += 1; }).await;
    }
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 8);
}

#[tokio::test]
async fn test_result_accumulation() {
    let queue = TaskQueue::new(2);
    let sum = Arc::new(Mutex::new(0));
    for i in 1..=5 {
        let sum = Arc::clone(&sum);
        queue.push(move || { *sum.lock().unwrap() += i; }).await;
    }
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*sum.lock().unwrap(), 15);
}

#[tokio::test]
async fn test_three_workers_five_tasks() {
    let queue = TaskQueue::new(3);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        queue.push(move || { *counter.lock().unwrap() += 1; }).await;
    }
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[tokio::test]
async fn test_consistency() {
    for _ in 0..3 {
        let queue = TaskQueue::new(2);
        let counter = Arc::new(Mutex::new(0));
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            queue.push(move || { *counter.lock().unwrap() += 1; }).await;
        }
        sleep(Duration::from_millis(50)).await;
        assert_eq!(*counter.lock().unwrap(), 5);
    }
}

#[tokio::test]
async fn test_two_tasks() {
    let queue = TaskQueue::new(2);
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        queue.push(move || { *counter.lock().unwrap() += 1; }).await;
    }
    sleep(Duration::from_millis(50)).await;
    assert_eq!(*counter.lock().unwrap(), 2);
}
