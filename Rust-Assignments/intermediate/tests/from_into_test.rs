#[path = "../src/easy/from_into.rs"]
mod from_into;
use from_into::{Celsius, Fahrenheit};


#[test]
fn test_freezing() {
    let f: Fahrenheit = Celsius(0.0).into();
    assert!((f.0 - 32.0).abs() < f64::EPSILON);
}

#[test]
fn test_boiling() {
    let f = Fahrenheit::from(Celsius(100.0));
    assert!((f.0 - 212.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative() {
    let f: Fahrenheit = Celsius(-40.0).into();
    assert!((f.0 - (-40.0)).abs() < f64::EPSILON);
}

#[test]
fn test_body_temp() {
    let f = Fahrenheit::from(Celsius(37.0));
    assert!((f.0 - 98.6).abs() < 0.01);
}

#[test]
fn test_room_temp() {
    let f = Fahrenheit::from(Celsius(20.0));
    assert!((f.0 - 68.0).abs() < f64::EPSILON);
}

#[test]
fn test_minus_10() {
    let f: Fahrenheit = Celsius(-10.0).into();
    assert!((f.0 - 14.0).abs() < f64::EPSILON);
}

#[test]
fn test_result_is_f64() {
    let f = Fahrenheit::from(Celsius(25.0));
    assert!((f.0 - 77.0).abs() < f64::EPSILON);
}

#[test]
fn test_from_and_into_equivalent() {
    let c = Celsius(50.0);
    let f1 = Fahrenheit::from(Celsius(50.0));
    let f2: Fahrenheit = c.into();
    assert!((f1.0 - f2.0).abs() < f64::EPSILON);
}

#[test]
fn test_200_celsius() {
    let f = Fahrenheit::from(Celsius(200.0));
    assert!((f.0 - 392.0).abs() < f64::EPSILON);
}

#[test]
fn test_above_freezing() {
    let f = Fahrenheit::from(Celsius(1.0));
    assert!(f.0 > 32.0);
}
