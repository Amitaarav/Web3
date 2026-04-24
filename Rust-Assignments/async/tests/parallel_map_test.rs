#[path = "../src/medium/parallel_map.rs"]
mod parallel_map;
use parallel_map::parallel_map;


#[test]
fn test_double() {
    let result = parallel_map(vec![1, 2, 3], |x| x * 2);
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_square() {
    let result = parallel_map(vec![1, 2, 3, 4, 5], |x| x * x);
    assert_eq!(result, vec![1, 4, 9, 16, 25]);
}

#[test]
fn test_empty() {
    let result = parallel_map(vec![], |x| x);
    assert!(result.is_empty());
}

#[test]
fn test_identity() {
    let result = parallel_map(vec![1, 2, 3], |x| x);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_add_one() {
    let result = parallel_map(vec![0, 1, 2], |x| x + 1);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_negate() {
    let result = parallel_map(vec![1, 2, 3], |x| -x);
    assert_eq!(result, vec![-1, -2, -3]);
}

#[test]
fn test_single_element() {
    let result = parallel_map(vec![42], |x| x * 2);
    assert_eq!(result, vec![84]);
}

#[test]
fn test_length_preserved() {
    let result = parallel_map(vec![1, 2, 3, 4, 5], |x| x);
    assert_eq!(result.len(), 5);
}

#[test]
fn test_zeros() {
    let result = parallel_map(vec![0, 0, 0], |x| x + 99);
    assert_eq!(result, vec![99, 99, 99]);
}
