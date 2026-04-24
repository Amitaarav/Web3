#[path = "../src/medium/trait_area.rs"]
mod trait_area;
use trait_area::{print_area, Circle, Shape, Square};


#[test]
fn test_circle() {
    let c = Circle { radius: 1.0 };
    assert!((print_area(&c) - std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_square() {
    let s = Square { side: 4.0 };
    assert!((print_area(&s) - 16.0).abs() < f64::EPSILON);
}

#[test]
fn test_zero_radius() {
    let c = Circle { radius: 0.0 };
    assert!((print_area(&c) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_large_square() {
    let s = Square { side: 100.0 };
    assert!((print_area(&s) - 10000.0).abs() < f64::EPSILON);
}

#[test]
fn test_circle_radius_2() {
    let c = Circle { radius: 2.0 };
    assert!((print_area(&c) - 4.0 * std::f64::consts::PI).abs() < 1e-10);
}

#[test]
fn test_square_side_1() {
    let s = Square { side: 1.0 };
    assert!((print_area(&s) - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_zero_side_square() {
    let s = Square { side: 0.0 };
    assert!((print_area(&s) - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_circle_area_positive() {
    let c = Circle { radius: 5.0 };
    assert!(print_area(&c) > 0.0);
}

#[test]
fn test_square_area_positive() {
    let s = Square { side: 3.0 };
    assert!(print_area(&s) > 0.0);
}

#[test]
fn test_circle_radius_10() {
    let c = Circle { radius: 10.0 };
    assert!((print_area(&c) - 100.0 * std::f64::consts::PI).abs() < 1e-8);
}
