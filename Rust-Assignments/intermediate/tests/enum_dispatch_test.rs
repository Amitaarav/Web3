#[path = "../src/medium/enum_dispatch.rs"]
mod enum_dispatch;
use enum_dispatch::{Circle, Shape, Square};


#[test]
fn test_circle_area() {
    let s = Shape::Circle(Circle { radius: 1.0 });
    assert!((s.area() - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_square_area() {
    let s = Shape::Square(Square { side: 4.0 });
    assert!((s.area() - 16.0).abs() < f64::EPSILON);
}

#[test]
fn test_large_radius() {
    let s = Shape::Circle(Circle { radius: 10.0 });
    assert!((s.area() - (100.0 * std::f64::consts::PI)).abs() < 1e-8);
}

#[test]
fn test_zero_radius_circle() {
    let s = Shape::Circle(Circle { radius: 0.0 });
    assert!((s.area() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_zero_side_square() {
    let s = Shape::Square(Square { side: 0.0 });
    assert!((s.area() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_unit_square() {
    let s = Shape::Square(Square { side: 1.0 });
    assert!((s.area() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_circle_area_positive() {
    let s = Shape::Circle(Circle { radius: 5.0 });
    assert!(s.area() > 0.0);
}

#[test]
fn test_square_side_3() {
    let s = Shape::Square(Square { side: 3.0 });
    assert!((s.area() - 9.0).abs() < f64::EPSILON);
}

#[test]
fn test_circle_radius_2() {
    let s = Shape::Circle(Circle { radius: 2.0 });
    assert!((s.area() - 4.0 * std::f64::consts::PI).abs() < 1e-10);
}
