/*
  Problem 67: Enum Dispatch — Multi-Type Handler

  Define an enum Shape with variants Circle(Circle) and Square(Square).
  Implement fn area(&self) -> f64 for the enum by matching and calling
  the area() method of the inner structs. This is an alternative to trait objects.

  Run the tests for this problem with:
    cargo test --test enum_dispatch_test
*/

pub struct Circle { pub radius: f64 }
pub struct Square { pub side: f64 }

impl Circle {
    pub fn area(&self) -> f64 { todo!() }
}

impl Square {
    pub fn area(&self) -> f64 { todo!() }
}

pub enum Shape {
    Circle(Circle),
    Square(Square),
}

impl Shape {
    pub fn area(&self) -> f64 {
        todo!()
    }
}
