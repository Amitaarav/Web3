/*
  Problem 77: Message Passing — Data Pipeline

  Create a three-stage pipeline using mpsc channels:
  1. Producer: Sends numbers 1..=5.
  2. Processor: Receives numbers, squares them, and sends to next stage.
  3. Consumer: Receives squared numbers and sums them.
  Implement this using three threads and return the final sum.

  Run the tests for this problem with:
    cargo test --test pipeline_test
*/

use std::sync::mpsc;
use std::thread;

pub fn data_pipeline() -> i32 {
    todo!()
}
