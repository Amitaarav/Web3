#[path = "../src/hard/fibonacci_iter.rs"]
mod fibonacci_iter;
use fibonacci_iter::Fibonacci;

#[test]
fn test_first_ten() {
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
}

#[test]
fn test_first() {
    let mut fib = Fibonacci::new();
    assert_eq!(fib.next(), Some(0));
}

#[test]
fn test_second() {
    let mut fib = Fibonacci::new();
    fib.next();
    assert_eq!(fib.next(), Some(1));
}

#[test]
fn test_sum_first_20() {
    let sum: u64 = Fibonacci::new().take(20).sum();
    assert_eq!(sum, 10945);
}

#[test]
fn test_third() {
    let fibs: Vec<u64> = Fibonacci::new().take(3).collect();
    assert_eq!(fibs[2], 1);
}

#[test]
fn test_seventh() {
    let fibs: Vec<u64> = Fibonacci::new().take(7).collect();
    assert_eq!(fibs[6], 8);
}

#[test]
fn test_is_iterator() {
    let count = Fibonacci::new().take(5).count();
    assert_eq!(count, 5);
}

#[test]
fn test_no_negatives() {
    let fibs: Vec<u64> = Fibonacci::new().take(20).collect();
    assert!(fibs.iter().all(|&x| x < u64::MAX));
}

#[test]
fn test_each_is_sum_of_prev_two() {
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    for i in 2..10 {
        assert_eq!(fibs[i], fibs[i-1] + fibs[i-2]);
    }
}

#[test]
fn test_fib_15() {
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    assert_eq!(fibs[14], 377);
}
