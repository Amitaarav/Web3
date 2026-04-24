#[path = "../src/medium/unsafe_swap.rs"]
mod unsafe_swap;
use unsafe_swap::unsafe_swap;


#[test]
fn test_swap() {
    let (mut a, mut b) = (5, 10);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 10);
    assert_eq!(b, 5);
}

#[test]
fn test_same_values() {
    let (mut a, mut b) = (7, 7);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 7);
    assert_eq!(b, 7);
}

#[test]
fn test_negative() {
    let (mut a, mut b) = (-1, 1);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 1);
    assert_eq!(b, -1);
}

#[test]
fn test_both_negative() {
    let (mut a, mut b) = (-5, -10);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, -10);
    assert_eq!(b, -5);
}

#[test]
fn test_zero_and_positive() {
    let (mut a, mut b) = (0, 42);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 42);
    assert_eq!(b, 0);
}

#[test]
fn test_large_values() {
    let (mut a, mut b) = (1000, 9999);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 9999);
    assert_eq!(b, 1000);
}

#[test]
fn test_double_swap_identity() {
    let (mut a, mut b) = (3, 7);
    unsafe_swap(&mut a, &mut b);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 3);
    assert_eq!(b, 7);
}

#[test]
fn test_first_becomes_second() {
    let (mut a, mut b) = (111, 222);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(a, 222);
}

#[test]
fn test_second_becomes_first() {
    let (mut a, mut b) = (111, 222);
    unsafe_swap(&mut a, &mut b);
    assert_eq!(b, 111);
}
