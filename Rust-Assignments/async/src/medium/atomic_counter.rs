/*
  Problem 79: Atomic Counter

  Rewrite the multithreaded counter problem (Problem 74) using AtomicI32
  instead of Mutex<i32>. Compare the performance and complexity.
  Show use of fetch_add and Ordering.

  Run the tests for this problem with:
    cargo test --test atomic_counter_test
*/

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_counter() -> i32 {
    todo!()
}
