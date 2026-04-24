#[path = "../src/hard/matrix_transpose.rs"]
mod matrix_transpose;
use matrix_transpose::transpose;


#[test]
fn test_2x3() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(transpose(m), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
}

#[test]
fn test_square() {
    let m = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(transpose(m), vec![vec![1, 3], vec![2, 4]]);
}

#[test]
fn test_single_row() {
    let m = vec![vec![1, 2, 3]];
    assert_eq!(transpose(m), vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn test_empty() {
    let m: Vec<Vec<i32>> = vec![];
    assert_eq!(transpose(m), Vec::<Vec<i32>>::new());
}

#[test]
fn test_single_column() {
    let m = vec![vec![1], vec![2], vec![3]];
    assert_eq!(transpose(m), vec![vec![1, 2, 3]]);
}

#[test]
fn test_1x1() {
    assert_eq!(transpose(vec![vec![9]]), vec![vec![9]]);
}

#[test]
fn test_3x3() {
    let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(transpose(m), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
}

#[test]
fn test_dimensions_flip() {
    let m = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
    let result = transpose(m);
    assert_eq!(result.len(), 4);
    assert_eq!(result[0].len(), 2);
}

#[test]
fn test_negative_values() {
    let m = vec![vec![-1, -2], vec![-3, -4]];
    assert_eq!(transpose(m), vec![vec![-1, -3], vec![-2, -4]]);
}

#[test]
fn test_all_zeros() {
    let m = vec![vec![0, 0], vec![0, 0]];
    assert_eq!(transpose(m), vec![vec![0, 0], vec![0, 0]]);
}
