/*
  Problem 10: Mutable Reference — Double All

  Write a function that takes a mutable reference to a Vec<i32> and doubles every element in-place.
  The function should return nothing (unit type).

  Run the tests for this problem with:
    cargo test --test double_all_test
*/

pub fn double_all(values: &mut Vec<i32>) {
    for val in values {
      *val *= 2
    }
}
