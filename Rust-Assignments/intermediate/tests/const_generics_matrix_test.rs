#[path = "../src/hard/const_generics_matrix.rs"]
mod const_generics_matrix;
use const_generics_matrix::Matrix;


#[test]
fn test_2x2_matrix() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(0, 0), Some(&1));
    assert_eq!(m.get(1, 1), Some(&4));
}

#[test]
fn test_out_of_bounds() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(2, 0), None);
    assert_eq!(m.get(0, 2), None);
}

#[test]
fn test_1x3_matrix() {
    let m = Matrix::new([[10, 20, 30]]);
    assert_eq!(m.get(0, 2), Some(&30));
}

#[test]
fn test_copy_types() {
    let m = Matrix::new([[1.5, 2.5]]);
    assert_eq!(m.get(0, 0), Some(&1.5));
}

#[test]
fn test_get_first_element() {
    let m = Matrix::new([[10, 20], [30, 40]]);
    assert_eq!(m.get(0, 0), Some(&10));
}

#[test]
fn test_get_last_element() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(1, 1), Some(&4));
}

#[test]
fn test_2x3_matrix() {
    let m = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(m.get(1, 2), Some(&6));
}

#[test]
fn test_negative_values() {
    let m = Matrix::new([[-1, -2], [-3, -4]]);
    assert_eq!(m.get(0, 1), Some(&-2));
}

#[test]
fn test_out_of_bounds_row() {
    let m = Matrix::new([[1, 2], [3, 4]]);
    assert_eq!(m.get(5, 0), None);
}

#[test]
fn test_3x3_center() {
    let m = Matrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(m.get(1, 1), Some(&5));
}
