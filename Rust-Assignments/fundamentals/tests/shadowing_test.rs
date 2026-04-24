#[path = "../src/easy/shadowing.rs"]
mod shadowing;
use shadowing::shadow_transform;


#[test]
fn test_basic() {
    assert_eq!(shadow_transform(5), "10");
}

#[test]
fn test_zero() {
    assert_eq!(shadow_transform(0), "0");
}

#[test]
fn test_large() {
    assert_eq!(shadow_transform(1000), "2000");
}

#[test]
fn test_one() {
    assert_eq!(shadow_transform(1), "2");
}

#[test]
fn test_two() {
    assert_eq!(shadow_transform(2), "4");
}

#[test]
fn test_ten() {
    assert_eq!(shadow_transform(10), "20");
}

#[test]
fn test_fifty() {
    assert_eq!(shadow_transform(50), "100");
}

#[test]
fn test_hundred() {
    assert_eq!(shadow_transform(100), "200");
}

#[test]
fn test_seven() {
    assert_eq!(shadow_transform(7), "14");
}

#[test]
fn test_returns_string() {
    let result = shadow_transform(3);
    assert_eq!(result, "6");
}
