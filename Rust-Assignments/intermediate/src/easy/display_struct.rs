/*
  Problem 36: Implement Display for a Struct

  Define a struct Point { x: f64, y: f64 } and implement std::fmt::Display so that
  it prints as (x, y) with two decimal places.
  For example, Point { x: 1.5, y: 2.0 } should display as (1.50, 2.00).

  Run the tests for this problem with:
    cargo test --test display_struct_test
*/

use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
