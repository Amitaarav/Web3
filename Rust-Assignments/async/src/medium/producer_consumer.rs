/*
  Problem 76: Message Passing — Producer-Consumer

  Write a function that creates a channel (std::sync::mpsc). Spawn a producer
  thread that sends the numbers 1 to 5 into the channel. The main thread (consumer)
  should collect these numbers into a Vec<i32> and return them.

  Run the tests for this problem with:
    cargo test --test producer_consumer_test
*/

use std::sync::mpsc;
use std::thread;

pub fn producer_consumer() -> Vec<i32> {
    todo!()
}
