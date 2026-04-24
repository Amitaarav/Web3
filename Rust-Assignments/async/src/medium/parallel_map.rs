/*
  Problem 73: Parallel Map

  Write a function parallel_map(v: Vec<i32>, f: fn(i32) -> i32) -> Vec<i32>
  that applies the function to each element in a separate thread and
  collects the results in the original order.
  This exercises thread spawning and joining in a loop.

  Run the tests for this problem with:
    cargo test --test parallel_map_test
*/

use std::thread;

pub fn parallel_map(v: Vec<i32>, f: fn(i32) -> i32) -> Vec<i32> {
    todo!()
}
