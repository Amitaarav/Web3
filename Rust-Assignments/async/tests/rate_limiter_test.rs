#[path = "../src/hard/rate_limiter.rs"]
mod rate_limiter;
use rate_limiter::RateLimiter;

use std::time::Instant;

#[tokio::test]
async fn test_limiter_permits() {
    let limiter = RateLimiter::new(2);
    limiter.acquire().await;
    limiter.acquire().await;
    // Third acquire should block if we don't drop the permits, but for this
    // mock we just check that we can acquire.
    assert_eq!(limiter.semaphore.available_permits(), 0);
}

#[tokio::test]
async fn test_new() {
    let limiter = RateLimiter::new(5);
    assert_eq!(limiter.semaphore.available_permits(), 5);
}

#[tokio::test]
async fn test_single_permit() {
    let limiter = RateLimiter::new(1);
    limiter.acquire().await;
    assert_eq!(limiter.semaphore.available_permits(), 0);
}

#[tokio::test]
async fn test_two_permits_after_one_acquire() {
    let limiter = RateLimiter::new(3);
    limiter.acquire().await;
    assert_eq!(limiter.semaphore.available_permits(), 2);
}

#[tokio::test]
async fn test_ten_permits() {
    let limiter = RateLimiter::new(10);
    assert_eq!(limiter.semaphore.available_permits(), 10);
}

#[tokio::test]
async fn test_three_all_acquired() {
    let limiter = RateLimiter::new(3);
    limiter.acquire().await;
    limiter.acquire().await;
    limiter.acquire().await;
    assert_eq!(limiter.semaphore.available_permits(), 0);
}

#[tokio::test]
async fn test_acquire_reduces_permits() {
    let limiter = RateLimiter::new(5);
    limiter.acquire().await;
    assert_eq!(limiter.semaphore.available_permits(), 4);
}

#[tokio::test]
async fn test_zero_initial_permits() {
    let limiter = RateLimiter::new(0);
    assert_eq!(limiter.semaphore.available_permits(), 0);
}

#[tokio::test]
async fn test_large_permit_count() {
    let limiter = RateLimiter::new(100);
    assert_eq!(limiter.semaphore.available_permits(), 100);
}

#[tokio::test]
async fn test_seven_permits() {
    let limiter = RateLimiter::new(7);
    assert_eq!(limiter.semaphore.available_permits(), 7);
}
