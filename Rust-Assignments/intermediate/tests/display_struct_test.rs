#[path = "../src/easy/display_struct.rs"]
mod display_struct;
use display_struct::Point;


#[test]
fn test_display_basic() {
    let p = Point { x: 1.5, y: 2.0 };
    assert_eq!(format!("{}", p), "(1.50, 2.00)");
}

#[test]
fn test_display_origin() {
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(format!("{}", p), "(0.00, 0.00)");
}

#[test]
fn test_display_negative() {
    let p = Point { x: -3.14, y: 2.72 };
    assert_eq!(format!("{}", p), "(-3.14, 2.72)");
}

#[test]
fn test_debug_still_works() {
    let p = Point { x: 1.0, y: 2.0 };
    let debug_str = format!("{:?}", p);
    assert!(debug_str.contains("Point"));
}

#[test]
fn test_display_large_values() {
    let p = Point { x: 100.0, y: 200.0 };
    assert_eq!(format!("{}", p), "(100.00, 200.00)");
}

#[test]
fn test_display_both_negative() {
    let p = Point { x: -1.0, y: -2.0 };
    assert_eq!(format!("{}", p), "(-1.00, -2.00)");
}

#[test]
fn test_display_format_has_parens() {
    let p = Point { x: 0.0, y: 0.0 };
    let s = format!("{}", p);
    assert!(s.starts_with('(') && s.ends_with(')'));
}

#[test]
fn test_display_two_decimal_places() {
    let p = Point { x: 1.0, y: 1.0 };
    let s = format!("{}", p);
    assert!(s.contains("1.00"));
}

#[test]
fn test_display_fractional() {
    let p = Point { x: 3.14, y: 2.72 };
    assert_eq!(format!("{}", p), "(3.14, 2.72)");
}

#[test]
fn test_display_contains_comma() {
    let p = Point { x: 1.0, y: 2.0 };
    let s = format!("{}", p);
    assert!(s.contains(','));
}
