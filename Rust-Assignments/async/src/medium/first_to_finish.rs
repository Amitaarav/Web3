/*
  Problem 86: Async Select — First to Finish

  Write an async function that takes two async tasks (Future<Item = i32>) and
  uses tokio::select! to return the result of the task that finishes first.

  Run the tests for this problem with:
    cargo test --test first_to_finish_test
*/

use std::future::Future;

pub async fn first_to_finish<F1, F2>(f1: F1, f2: F2) -> i32
where
    F1: Future<Output = i32> + Unpin,
    F2: Future<Output = i32> + Unpin,
{
    todo!()
}
