/*
  Problem 100: Async Task Scheduler

  Implement a TaskScheduler that can schedule async closures to run after
  a specific delay. It should use a PriorityQueue (or sorted Vec) to keep track
  of tasks and a background tokio task to execute them when their time comes.

  Run the tests for this problem with:
    cargo test --test task_scheduler_test
*/

use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};

pub struct TaskScheduler {
    pub tasks: Arc<Mutex<Vec<(Instant, Box<dyn FnOnce() + Send + 'static>)>>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        todo!()
    }

    pub fn schedule<F>(&self, delay: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }

    pub fn start(&self) {
        todo!() // Spawn a background task to process the queue
    }
}
