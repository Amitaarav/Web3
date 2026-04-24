/*
  Problem 3: Tuple Swap

  Write a function that takes a tuple of two i32 values and returns a new tuple with the elements swapped.

  Run the tests for this problem with:
    cargo test --test tuple_swap_test
*/

pub fn swap_tuple(t: (i32, i32)) -> (i32, i32) {
    (t.1, t.0)
}
