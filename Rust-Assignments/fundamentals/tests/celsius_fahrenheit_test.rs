#[path = "../src/easy/celsius_fahrenheit.rs"]
mod celsius_fahrenheit;
use celsius_fahrenheit::celsius_to_fahrenheit;


#[test]
fn test_freezing() {
    assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < f64::EPSILON);
}

#[test]
fn test_boiling() {
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative() {
    assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < f64::EPSILON);
}

#[test]
fn test_body_temp() {
    assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.01);
}

#[test]
fn test_room_temp() {
    assert!((celsius_to_fahrenheit(20.0) - 68.0).abs() < f64::EPSILON);
}

#[test]
fn test_minus_10() {
    assert!((celsius_to_fahrenheit(-10.0) - 14.0).abs() < f64::EPSILON);
}

#[test]
fn test_result_gt_celsius_for_positive() {
    assert!(celsius_to_fahrenheit(50.0) > 50.0);
}

#[test]
fn test_result_type_f64() {
    let result: f64 = celsius_to_fahrenheit(25.0);
    assert!((result - 77.0).abs() < f64::EPSILON);
}

#[test]
fn test_200_celsius() {
    assert!((celsius_to_fahrenheit(200.0) - 392.0).abs() < f64::EPSILON);
}

#[test]
fn test_minus_273_approx() {
    let result = celsius_to_fahrenheit(-273.15);
    assert!((result - (-459.67)).abs() < 0.01);
}
