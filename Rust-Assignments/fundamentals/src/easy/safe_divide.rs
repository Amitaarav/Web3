/*
  Problem 14: Safe Division with Option

  Write a function that divides two f64 numbers.
  Return None if the divisor is zero, otherwise return Some(result).

  Run the tests for this problem with:
    cargo test --test safe_divide_test
*/

pub fn safe_divide(a: f64, b: f64) -> Option<f64> {
    // if b == 0.0 {
    //   None
    // }else{
    //   Some(a/b)
    // }
    if b.abs() < f64::EPSILON {
      None
    } else {
      Some(a / b)
    }
}
