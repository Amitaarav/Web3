#[path = "../src/easy/sum.rs"]
mod sum;
use sum::add;


#[test]
fn test_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_negative_numbers() {
    assert_eq!(add(-1, -1), -2);
}

#[test]
fn test_zero() {
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_mixed_signs() {
    assert_eq!(add(-5, 10), 5);
}

#[test]
fn test_identity_zero() {
    assert_eq!(add(7, 0), 7);
}

#[test]
fn test_commutative() {
    assert_eq!(add(3, 4), add(4, 3));
}

#[test]
fn test_large() {
    assert_eq!(add(1000, 2000), 3000);
}

#[test]
fn test_both_negative_large() {
    assert_eq!(add(-100, -200), -300);
}

#[test]
fn test_cancel() {
    assert_eq!(add(42, -42), 0);
}

#[test]
fn test_one_plus_one() {
    assert_eq!(add(1, 1), 2);
}
