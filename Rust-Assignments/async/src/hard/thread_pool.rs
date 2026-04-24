/*
  Problem 78: Thread Pool — Simple Worker Thread

  Implement a simple ThreadPool that can execute closures. The pool should
  maintain a fixed number of worker threads and use a channel to send
  jobs to workers. Implement new(size: usize) and execute<F>(&self, f: F).

  Run the tests for this problem with:
    cargo test --test thread_pool_test
*/

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        todo!()
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
