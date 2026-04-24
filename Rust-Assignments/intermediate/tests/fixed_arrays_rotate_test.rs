#[path = "../src/medium/fixed_arrays_rotate.rs"]
mod fixed_arrays_rotate;
use fixed_arrays_rotate::rotate_left;


#[test]
fn test_rotate_1() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 1), [2, 3, 4, 5, 6, 7, 8, 1]);
}

#[test]
fn test_rotate_0() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 0), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_rotate_8() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 8), [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn test_rotate_3() {
    assert_eq!(rotate_left([10, 20, 30, 40, 50, 60, 70, 80], 3), [40, 50, 60, 70, 80, 10, 20, 30]);
}

#[test]
fn test_rotate_large() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 10), [3, 4, 5, 6, 7, 8, 1, 2]);
}

#[test]
fn test_rotate_4() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 4), [5, 6, 7, 8, 1, 2, 3, 4]);
}

#[test]
fn test_rotate_7() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 7), [8, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_rotate_2() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 2), [3, 4, 5, 6, 7, 8, 1, 2]);
}

#[test]
fn test_rotate_same_elements() {
    assert_eq!(rotate_left([5, 5, 5, 5, 5, 5, 5, 5], 3), [5, 5, 5, 5, 5, 5, 5, 5]);
}

#[test]
fn test_rotate_16_is_same_as_0() {
    assert_eq!(rotate_left([1, 2, 3, 4, 5, 6, 7, 8], 16), [1, 2, 3, 4, 5, 6, 7, 8]);
}
