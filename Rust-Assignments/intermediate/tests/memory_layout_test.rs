#[path = "../src/easy/memory_layout.rs"]
mod memory_layout;
use memory_layout::type_info;


#[test]
fn test_u8() {
    assert_eq!(type_info::<u8>(), (1, 1));
}

#[test]
fn test_u32() {
    assert_eq!(type_info::<u32>(), (4, 4));
}

#[test]
fn test_u64() {
    assert_eq!(type_info::<u64>(), (8, 8));
}

#[test]
fn test_bool() {
    assert_eq!(type_info::<bool>(), (1, 1));
}

#[test]
fn test_i32() {
    assert_eq!(type_info::<i32>(), (4, 4));
}

#[test]
fn test_i64() {
    assert_eq!(type_info::<i64>(), (8, 8));
}

#[test]
fn test_u16() {
    assert_eq!(type_info::<u16>(), (2, 2));
}

#[test]
fn test_i8() {
    assert_eq!(type_info::<i8>(), (1, 1));
}

#[test]
fn test_f32() {
    assert_eq!(type_info::<f32>(), (4, 4));
}

#[test]
fn test_f64() {
    assert_eq!(type_info::<f64>(), (8, 8));
}
