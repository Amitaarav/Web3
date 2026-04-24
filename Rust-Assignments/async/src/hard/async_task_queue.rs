/*
  Problem 96: Async Task Queue

  Implement an async TaskQueue where you can push async closures and have a
  fixed number of workers process them. Use a mpsc channel for the queue.
  Workers should be spawned as tokio tasks.

  Run the tests for this problem with:
    cargo test --test async_task_queue_test
*/

use tokio::sync::mpsc;
use std::sync::Arc;

pub struct TaskQueue {
    pub sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl TaskQueue {
    pub fn new(worker_count: usize) -> Self {
        todo!()
    }

    pub async fn push<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}
