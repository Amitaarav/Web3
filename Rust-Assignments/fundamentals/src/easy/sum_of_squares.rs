/*
  Problem 17: Iterator — Sum of Squares

  Write a function that takes a slice of i32 values and returns the sum of squares
  using iterator adapters (.iter(), .map(), .sum()).

  Run the tests for this problem with:
    cargo test --test sum_of_squares_test
*/

pub fn sum_of_squares(values: &[i32]) -> i32 {
    // if values.is_empty(){
    //   None
    // } else {
    //   Some(values.iter().map(|x| x * x).sum())
    // }
    values.iter().map(|x| x * x).sum()
}
