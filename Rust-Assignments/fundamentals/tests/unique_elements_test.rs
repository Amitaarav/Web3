#[path = "../src/medium/unique_elements.rs"]
mod unique_elements;
use unique_elements::unique_elements;


#[test]
fn test_duplicates() {
    assert_eq!(unique_elements(vec![1, 2, 2, 3, 1, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_no_duplicates() {
    assert_eq!(unique_elements(vec![5, 4, 3]), vec![5, 4, 3]);
}

#[test]
fn test_empty() {
    assert_eq!(unique_elements(vec![]), vec![]);
}

#[test]
fn test_all_same() {
    assert_eq!(unique_elements(vec![7, 7, 7, 7]), vec![7]);
}

#[test]
fn test_single_element() {
    assert_eq!(unique_elements(vec![42]), vec![42]);
}

#[test]
fn test_two_same() {
    assert_eq!(unique_elements(vec![1, 1]), vec![1]);
}

#[test]
fn test_preserves_order() {
    assert_eq!(unique_elements(vec![3, 1, 2, 1, 3]), vec![3, 1, 2]);
}

#[test]
fn test_negatives() {
    assert_eq!(unique_elements(vec![-1, -2, -1, -3]), vec![-1, -2, -3]);
}

#[test]
fn test_large_input() {
    let v = vec![1; 100];
    assert_eq!(unique_elements(v), vec![1]);
}

#[test]
fn test_mixed_signs() {
    assert_eq!(unique_elements(vec![-1, 0, 1, 0, -1]), vec![-1, 0, 1]);
}
