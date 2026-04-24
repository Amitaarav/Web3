/*
  Problem 90: Manual Future Implementation

  Implement a struct ReadyFuture that implements the std::future::Future trait.
  When polled, it should always return Poll::Ready(value).
  This is a minimal demonstration of how futures work under the hood.

  Run the tests for this problem with:
    cargo test --test manual_future_test
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ReadyFuture<T> {
    pub value: Option<T>,
}

impl<T: Unpin> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
