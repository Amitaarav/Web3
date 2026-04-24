#[path = "../src/easy/newtype_wrapper.rs"]
mod newtype_wrapper;
use newtype_wrapper::{Kilometers, Meters};


#[test]
fn test_conversion() {
    let km = Kilometers::from(Meters(1500.0));
    assert!((km.0 - 1.5).abs() < f64::EPSILON);
}

#[test]
fn test_add_meters() {
    assert_eq!(Meters(100.0) + Meters(200.0), Meters(300.0));
}

#[test]
fn test_zero_conversion() {
    let km = Kilometers::from(Meters(0.0));
    assert!((km.0 - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_into() {
    let km: Kilometers = Meters(5000.0).into();
    assert!((km.0 - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_1000_meters_is_1_km() {
    let km = Kilometers::from(Meters(1000.0));
    assert!((km.0 - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_add_zeros() {
    assert_eq!(Meters(0.0) + Meters(0.0), Meters(0.0));
}

#[test]
fn test_add_negative() {
    assert_eq!(Meters(100.0) + Meters(-50.0), Meters(50.0));
}

#[test]
fn test_500_meters() {
    let km = Kilometers::from(Meters(500.0));
    assert!((km.0 - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_add_large() {
    assert_eq!(Meters(1000.0) + Meters(2000.0), Meters(3000.0));
}

#[test]
fn test_km_value() {
    let km = Kilometers::from(Meters(2500.0));
    assert!((km.0 - 2.5).abs() < f64::EPSILON);
}
