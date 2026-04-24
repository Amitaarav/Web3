#[path = "../src/easy/double_all.rs"]
mod double_all;
use double_all::double_all;


#[test]
fn test_basic() {
    let mut v = vec![1, 2, 3];
    double_all(&mut v);
    assert_eq!(v, vec![2, 4, 6]);
}

#[test]
fn test_empty() {
    let mut v: Vec<i32> = vec![];
    double_all(&mut v);
    assert_eq!(v, vec![]);
}

#[test]
fn test_negative() {
    let mut v = vec![-1, -2, -3];
    double_all(&mut v);
    assert_eq!(v, vec![-2, -4, -6]);
}

#[test]
fn test_zeros() {
    let mut v = vec![0, 0, 0];
    double_all(&mut v);
    assert_eq!(v, vec![0, 0, 0]);
}

#[test]
fn test_single() {
    let mut v = vec![5];
    double_all(&mut v);
    assert_eq!(v, vec![10]);
}

#[test]
fn test_large() {
    let mut v = vec![100, 200, 300];
    double_all(&mut v);
    assert_eq!(v, vec![200, 400, 600]);
}

#[test]
fn test_mixed() {
    let mut v = vec![-1, 0, 1];
    double_all(&mut v);
    assert_eq!(v, vec![-2, 0, 2]);
}

#[test]
fn test_length_unchanged() {
    let mut v = vec![1, 2, 3, 4, 5];
    double_all(&mut v);
    assert_eq!(v.len(), 5);
}

#[test]
fn test_idempotent_after_halve() {
    let mut v = vec![4, 6, 8];
    double_all(&mut v);
    assert_eq!(v, vec![8, 12, 16]);
}

#[test]
fn test_single_negative() {
    let mut v = vec![-7];
    double_all(&mut v);
    assert_eq!(v, vec![-14]);
}
