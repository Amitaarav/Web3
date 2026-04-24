/*
  Problem 94: Async Rate Limiter

  Implement a simple async RateLimiter that allows a maximum number of requests
  per second. Use a Semaphore (tokio::sync::Semaphore) to limit concurrency.
  Implement a method fn acquire(&self) -> Future<Output = ()>.
  When acquiring, permanently consume the permit by calling .forget() on the
  SemaphorePermit so that available_permits() decreases with each acquire call. 

  Run the tests for this problem with:
    cargo test --test rate_limiter_test
*/

use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimiter {
    pub semaphore: Arc<Semaphore>,
}

impl RateLimiter {
    pub fn new(permits: usize) -> Self {
        todo!()
    }

    pub async fn acquire(&self) {
        todo!()
    }
}
