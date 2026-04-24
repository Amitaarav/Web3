#[path = "../src/medium/box_list.rs"]
mod box_list;
use box_list::List;


#[test]
fn test_sum() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    assert_eq!(list.sum(), 6);
}

#[test]
fn test_len() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_empty() {
    assert_eq!(List::Nil.sum(), 0);
    assert_eq!(List::Nil.len(), 0);
}

#[test]
fn test_single() {
    let list = List::Cons(42, Box::new(List::Nil));
    assert_eq!(list.sum(), 42);
    assert_eq!(list.len(), 1);
}

#[test]
fn test_three_elements_len() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    assert_eq!(list.len(), 3);
}

#[test]
fn test_negative_values() {
    let list = List::Cons(-1, Box::new(List::Cons(-2, Box::new(List::Nil))));
    assert_eq!(list.sum(), -3);
    assert_eq!(list.len(), 2);
}

#[test]
fn test_zero_sum() {
    let list = List::Cons(5, Box::new(List::Cons(-5, Box::new(List::Nil))));
    assert_eq!(list.sum(), 0);
}

#[test]
fn test_large_list_len() {
    let mut list = List::Nil;
    for i in 0..10 {
        list = List::Cons(i, Box::new(list));
    }
    assert_eq!(list.len(), 10);
}

#[test]
fn test_sum_of_zeros() {
    let list = List::Cons(0, Box::new(List::Cons(0, Box::new(List::Cons(0, Box::new(List::Nil))))));
    assert_eq!(list.sum(), 0);
}

#[test]
fn test_nil_sum_and_len() {
    let nil = List::Nil;
    assert_eq!(nil.sum(), 0);
    assert_eq!(nil.len(), 0);
}
