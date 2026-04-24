/*
  Problem 62: Smart Pointers — Drop Trait

  Implement a struct Logger { name: String } that prints "Dropping <name>"
  when it goes out of scope using the Drop trait.
  Since we cannot easily test stdout in a unit test, have it increment a
  static atomic counter (or a shared counter like Arc<Mutex<i32>>) instead.
  For this exercise, use an Arc<Mutex<usize>> to track drops.

  Run the tests for this problem with:
    cargo test --test smart_pointer_drop_test
*/

use std::sync::{Arc, Mutex};

pub struct Logger {
    pub name: String,
    pub drop_count: Arc<Mutex<usize>>,
}

impl Drop for Logger {
    fn drop(&mut self) {
        todo!()
    }
}
