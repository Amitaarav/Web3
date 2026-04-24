#[path = "../src/easy/color_enum.rs"]
mod color_enum;
use color_enum::{color_to_rgb, Color};


#[test]
fn test_red() {
    assert_eq!(color_to_rgb(Color::Red), (255, 0, 0));
}

#[test]
fn test_green() {
    assert_eq!(color_to_rgb(Color::Green), (0, 255, 0));
}

#[test]
fn test_blue() {
    assert_eq!(color_to_rgb(Color::Blue), (0, 0, 255));
}

#[test]
fn test_custom() {
    assert_eq!(color_to_rgb(Color::Custom(128, 64, 32)), (128, 64, 32));
}

#[test]
fn test_custom_zeros() {
    assert_eq!(color_to_rgb(Color::Custom(0, 0, 0)), (0, 0, 0));
}

#[test]
fn test_custom_max() {
    assert_eq!(color_to_rgb(Color::Custom(255, 255, 255)), (255, 255, 255));
}

#[test]
fn test_custom_yellow() {
    assert_eq!(color_to_rgb(Color::Custom(255, 255, 0)), (255, 255, 0));
}

#[test]
fn test_red_no_green() {
    let (_, g, _) = color_to_rgb(Color::Red);
    assert_eq!(g, 0);
}

#[test]
fn test_green_no_blue() {
    let (_, _, b) = color_to_rgb(Color::Green);
    assert_eq!(b, 0);
}

#[test]
fn test_blue_no_red() {
    let (r, _, _) = color_to_rgb(Color::Blue);
    assert_eq!(r, 0);
}
