/*
  Problem 97: Async Timeout Wrapper

  Write an async function timeout_wrapper<F, T>(f: F, duration: Duration)
  -> Result<T, String> where F is a future. Use tokio::time::timeout to
  execute the future. Return Err("Timeout") if the operation takes too long.

  Run the tests for this problem with:
    cargo test --test timeout_wrapper_test
*/

use tokio::time::{timeout, Duration};
use std::future::Future;

pub async fn timeout_wrapper<F, T>(f: F, duration: Duration) -> Result<T, String>
where
    F: Future<Output = T>,
{
    todo!()
}
