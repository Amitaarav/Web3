/*
  Problem 42: Newtype Pattern — Wrapper

  Create newtypes Meters(pub f64) and Kilometers(pub f64).
  Implement From<Meters> for Kilometers (1 km = 1000 m).
  Also implement Add for Meters so two Meters values can be added together.

  Run the tests for this problem with:
    cargo test --test newtype_wrapper_test
*/

use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Meters(pub f64);

#[derive(Debug, PartialEq)]
pub struct Kilometers(pub f64);

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        todo!()
    }
}

impl Add for Meters {
    type Output = Meters;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
