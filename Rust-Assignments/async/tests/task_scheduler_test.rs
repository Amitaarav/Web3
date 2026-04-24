#[path = "../src/hard/task_scheduler.rs"]
mod task_scheduler;
use task_scheduler::TaskScheduler;

use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_scheduling() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));

    scheduler.start();

    {
        let counter = Arc::clone(&counter);
        scheduler.schedule(Duration::from_millis(50), move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }

    // Should not have run yet
    sleep(Duration::from_millis(10)).await;
    assert_eq!(*counter.lock().unwrap(), 0);

    // Should have run now
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[tokio::test]
async fn test_multiple_tasks() {
    let scheduler = TaskScheduler::new();
    let result = Arc::new(Mutex::new(Vec::new()));

    scheduler.start();

    {
        let result = Arc::clone(&result);
        scheduler.schedule(Duration::from_millis(100), move || {
            result.lock().unwrap().push(2);
        });
    }
    {
        let result = Arc::clone(&result);
        scheduler.schedule(Duration::from_millis(20), move || {
            result.lock().unwrap().push(1);
        });
    }

    sleep(Duration::from_millis(150)).await;
    let final_res = result.lock().unwrap();
    assert_eq!(*final_res, vec![1, 2]); // Order based on delay
}

#[tokio::test]
async fn test_counter_starts_zero() {
    let counter = Arc::new(Mutex::new(0));
    assert_eq!(*counter.lock().unwrap(), 0);
}

#[tokio::test]
async fn test_two_tasks_run() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));
    scheduler.start();
    for _ in 0..2 {
        let counter = Arc::clone(&counter);
        scheduler.schedule(Duration::from_millis(20), move || {
            *counter.lock().unwrap() += 1;
        });
    }
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 2);
}

#[tokio::test]
async fn test_task_runs_after_delay() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));
    scheduler.start();
    let c = Arc::clone(&counter);
    scheduler.schedule(Duration::from_millis(50), move || {
        *c.lock().unwrap() += 10;
    });
    sleep(Duration::from_millis(5)).await;
    assert_eq!(*counter.lock().unwrap(), 0);
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 10);
}

#[tokio::test]
async fn test_task_not_run_immediately() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));
    scheduler.start();
    let c = Arc::clone(&counter);
    scheduler.schedule(Duration::from_millis(200), move || {
        *c.lock().unwrap() += 1;
    });
    assert_eq!(*counter.lock().unwrap(), 0);
}

#[tokio::test]
async fn test_five_tasks_run() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));
    scheduler.start();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        scheduler.schedule(Duration::from_millis(20), move || {
            *counter.lock().unwrap() += 1;
        });
    }
    sleep(Duration::from_millis(150)).await;
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[tokio::test]
async fn test_ordering_by_delay() {
    let scheduler = TaskScheduler::new();
    let result = Arc::new(Mutex::new(Vec::new()));
    scheduler.start();
    let r1 = Arc::clone(&result);
    scheduler.schedule(Duration::from_millis(80), move || { r1.lock().unwrap().push(2); });
    let r2 = Arc::clone(&result);
    scheduler.schedule(Duration::from_millis(30), move || { r2.lock().unwrap().push(1); });
    sleep(Duration::from_millis(150)).await;
    assert_eq!(*result.lock().unwrap(), vec![1, 2]);
}

#[tokio::test]
async fn test_accumulation() {
    let scheduler = TaskScheduler::new();
    let sum = Arc::new(Mutex::new(0));
    scheduler.start();
    for i in 1..=3u32 {
        let sum = Arc::clone(&sum);
        scheduler.schedule(Duration::from_millis((20 * i).into()), move || {
            *sum.lock().unwrap() += i;
        });
    }
    sleep(Duration::from_millis(200)).await;
    assert_eq!(*sum.lock().unwrap(), 6);
}

#[tokio::test]
async fn test_set_value() {
    let scheduler = TaskScheduler::new();
    let counter = Arc::new(Mutex::new(0));
    scheduler.start();
    let c = Arc::clone(&counter);
    scheduler.schedule(Duration::from_millis(30), move || {
        *c.lock().unwrap() = 42;
    });
    sleep(Duration::from_millis(100)).await;
    assert_eq!(*counter.lock().unwrap(), 42);
}
