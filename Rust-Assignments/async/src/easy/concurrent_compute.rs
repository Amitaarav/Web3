/*
  Problem 85: Concurrent Computation

  Write an async function that takes two async tasks (represented by closures
  that return a Future<Item = i32>), runs them concurrently using tokio::join!,
  and returns the sum of their results.

  Run the tests for this problem with:
    cargo test --test concurrent_compute_test
*/

use std::future::Future;

pub async fn compute_concurrently<F1, F2>(f1: F1, f2: F2) -> i32
where
    F1: Future<Output = i32>,
    F2: Future<Output = i32>,
{
    todo!()
}
