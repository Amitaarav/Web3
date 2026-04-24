/*
  Problem 16: Closure — Apply Twice

  Write a function that takes an i32 and a closure F: Fn(i32) -> i32,
  applies the closure twice to the value, and returns the result.

  Run the tests for this problem with:
    cargo test --test apply_twice_test
*/

pub fn apply_twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    todo!()
}
