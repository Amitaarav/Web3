/*
  Problem 27: Trait — Area Calculation

  Define a trait Shape with a method fn area(&self) -> f64.
  Implement it for structs Circle { radius: f64 } and Square { side: f64 }.
  Write a function print_area that accepts any &dyn Shape and returns the area.

  Run the tests for this problem with:
    cargo test --test trait_area_test
*/

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }
}

pub fn print_area(shape: &dyn Shape) -> f64 {
    todo!()
}
