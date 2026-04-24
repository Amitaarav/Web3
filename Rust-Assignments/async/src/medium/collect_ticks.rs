/*
  Problem 95: Async Streams — Collect Ticks

  Write an async function that creates a tokio::time::interval (1ms).
  Collect the first 5 "ticks" (Instant values) from the interval stream
  into a Vec and return it.

  Run the tests for this problem with:
    cargo test --test collect_ticks_test
*/

use tokio::time::{interval, Duration, Instant};

pub async fn collect_ticks() -> Vec<Instant> {
    todo!()
}
