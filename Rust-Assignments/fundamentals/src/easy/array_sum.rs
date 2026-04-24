/*
  Problem 4: Array Sum

  Write a function that takes a reference to an array of exactly 5 i32 values and returns their sum.

  Run the tests for this problem with:
    cargo test --test array_sum_test
*/

pub fn array_sum(arr: &[i32; 5]) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
      sum += arr[i];
    }
    sum
}
