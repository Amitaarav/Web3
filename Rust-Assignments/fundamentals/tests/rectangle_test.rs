#[path = "../src/easy/rectangle.rs"]
mod rectangle;
use rectangle::Rectangle;


#[test]
fn test_area() {
    let r = Rectangle { width: 5.0, height: 3.0 };
    assert!((r.area() - 15.0).abs() < f64::EPSILON);
}

#[test]
fn test_square() {
    let r = Rectangle { width: 4.0, height: 4.0 };
    assert!(r.is_square());
}

#[test]
fn test_not_square() {
    let r = Rectangle { width: 4.0, height: 5.0 };
    assert!(!r.is_square());
}

#[test]
fn test_zero_area() {
    let r = Rectangle { width: 0.0, height: 10.0 };
    assert!((r.area() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_area_large() {
    let r = Rectangle { width: 100.0, height: 100.0 };
    assert!((r.area() - 10000.0).abs() < f64::EPSILON);
}

#[test]
fn test_is_square_unit() {
    let r = Rectangle { width: 1.0, height: 1.0 };
    assert!(r.is_square());
}

#[test]
fn test_not_square_height_larger() {
    let r = Rectangle { width: 3.0, height: 6.0 };
    assert!(!r.is_square());
}

#[test]
fn test_area_fractional() {
    let r = Rectangle { width: 2.5, height: 4.0 };
    assert!((r.area() - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_area_commutative() {
    let r1 = Rectangle { width: 3.0, height: 5.0 };
    let r2 = Rectangle { width: 5.0, height: 3.0 };
    assert!((r1.area() - r2.area()).abs() < f64::EPSILON);
}

#[test]
fn test_zero_height_area() {
    let r = Rectangle { width: 10.0, height: 0.0 };
    assert!((r.area() - 0.0).abs() < f64::EPSILON);
}
