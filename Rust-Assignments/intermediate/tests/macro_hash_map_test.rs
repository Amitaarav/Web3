#[path = "../src/medium/macro_hash_map.rs"]
mod macro_hash_map;

use std::collections::HashMap;


#[test]
fn test_basic() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "a" => 1, "b" => 2 };
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
}

#[test]
fn test_empty() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! {};
    assert!(m.is_empty());
}

#[test]
fn test_single() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, &str> = hash_map! { "key" => "value" };
    assert_eq!(m.get("key"), Some(&"value"));
}

#[test]
fn test_integer_keys() {
    use macro_hash_map::hash_map;
    let m: HashMap<i32, &str> = hash_map! { 1 => "one", 2 => "two", 3 => "three" };
    assert_eq!(m.len(), 3);
    assert_eq!(m.get(&2), Some(&"two"));
}

#[test]
fn test_missing_key_is_none() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "a" => 1 };
    assert_eq!(m.get("z"), None);
}

#[test]
fn test_overwrite_key() {
    use macro_hash_map::hash_map;
    let mut m: HashMap<&str, i32> = hash_map! { "x" => 1 };
    m.insert("x", 99);
    assert_eq!(m.get("x"), Some(&99));
}

#[test]
fn test_two_pairs() {
    use macro_hash_map::hash_map;
    let m: HashMap<i32, i32> = hash_map! { 10 => 100, 20 => 200 };
    assert_eq!(m.get(&10), Some(&100));
    assert_eq!(m.get(&20), Some(&200));
}

#[test]
fn test_length_correct() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "a" => 1, "b" => 2, "c" => 3 };
    assert_eq!(m.len(), 3);
}

#[test]
fn test_contains_key() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "rust" => 42 };
    assert!(m.contains_key("rust"));
}

#[test]
fn test_value_zero() {
    use macro_hash_map::hash_map;
    let m: HashMap<&str, i32> = hash_map! { "zero" => 0 };
    assert_eq!(m.get("zero"), Some(&0));
}
