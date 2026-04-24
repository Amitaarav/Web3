/*
  Problem 83: Async Delay

  Write an async function delayed_value(val: i32) -> i32 that sleeps for
  10 milliseconds using tokio::time::sleep and then returns the value.

  Run the tests for this problem with:
    cargo test --test delayed_value_test
*/

use tokio::time::{sleep, Duration};

pub async fn delayed_value(val: i32) -> i32 {
    todo!()
}
