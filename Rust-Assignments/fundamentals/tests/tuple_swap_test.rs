#[path = "../src/easy/tuple_swap.rs"]
mod tuple_swap;
use tuple_swap::swap_tuple;


#[test]
fn test_swap() {
    assert_eq!(swap_tuple((1, 2)), (2, 1));
}

#[test]
fn test_same_values() {
    assert_eq!(swap_tuple((5, 5)), (5, 5));
}

#[test]
fn test_negative() {
    assert_eq!(swap_tuple((-3, 7)), (7, -3));
}

#[test]
fn test_zero() {
    assert_eq!(swap_tuple((0, 42)), (42, 0));
}

#[test]
fn test_both_negative() {
    assert_eq!(swap_tuple((-5, -10)), (-10, -5));
}

#[test]
fn test_both_zero() {
    assert_eq!(swap_tuple((0, 0)), (0, 0));
}

#[test]
fn test_large_values() {
    assert_eq!(swap_tuple((1000, 9999)), (9999, 1000));
}

#[test]
fn test_first_larger() {
    assert_eq!(swap_tuple((100, 1)), (1, 100));
}

#[test]
fn test_double_swap_identity() {
    let t = (3, 7);
    assert_eq!(swap_tuple(swap_tuple(t)), t);
}

#[test]
fn test_min_max() {
    assert_eq!(swap_tuple((i32::MIN, i32::MAX)), (i32::MAX, i32::MIN));
}
