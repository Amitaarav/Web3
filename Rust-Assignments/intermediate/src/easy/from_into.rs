/*
  Problem 37: From/Into Conversion

  Define a struct Celsius(pub f64) and a struct Fahrenheit(pub f64).
  Implement From<Celsius> for Fahrenheit using the formula F = C * 9/5 + 32.
  This automatically gives you Into<Fahrenheit> for Celsius.

  Run the tests for this problem with:
    cargo test --test from_into_test
*/

#[derive(Debug, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, PartialEq)]
pub struct Fahrenheit(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}
