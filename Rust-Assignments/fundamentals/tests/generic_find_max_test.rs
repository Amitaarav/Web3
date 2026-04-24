#[path = "../src/medium/generic_find_max.rs"]
mod generic_find_max;
use generic_find_max::find_max;


#[test]
fn test_ints() {
    assert_eq!(find_max(&[1, 5, 3, 9, 2]), Some(&9));
}

#[test]
fn test_floats() {
    assert_eq!(find_max(&[1.5, 3.7, 2.1]), Some(&3.7));
}

#[test]
fn test_single() {
    assert_eq!(find_max(&[42]), Some(&42));
}

#[test]
fn test_empty() {
    let empty: &[i32] = &[];
    assert_eq!(find_max(empty), None);
}

#[test]
fn test_strings() {
    assert_eq!(find_max(&["apple", "banana", "cherry"]), Some(&"cherry"));
}

#[test]
fn test_all_equal() {
    assert_eq!(find_max(&[5, 5, 5]), Some(&5));
}

#[test]
fn test_all_negative() {
    assert_eq!(find_max(&[-3, -1, -5]), Some(&-1));
}

#[test]
fn test_two_elements() {
    assert_eq!(find_max(&[1, 2]), Some(&2));
}

#[test]
fn test_descending_order() {
    assert_eq!(find_max(&[5, 4, 3, 2, 1]), Some(&5));
}

#[test]
fn test_max_at_start() {
    assert_eq!(find_max(&[100, 1, 2, 3]), Some(&100));
}
