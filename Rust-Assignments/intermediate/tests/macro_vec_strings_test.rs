#[path = "../src/medium/macro_vec_strings.rs"]
mod macro_vec_strings;

#[test]

fn test_basic() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["hello", "world"];
    assert_eq!(v, vec!["hello".to_string(), "world".to_string()]);
}

#[test]
fn test_single() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["rust"];
    assert_eq!(v, vec!["rust".to_string()]);
}

#[test]
fn test_empty() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings![];
    assert!(v.is_empty());
}

#[test]
fn test_multiple() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["a", "b", "c", "d"];
    assert_eq!(v.len(), 4);
}

#[test]
fn test_elements_are_strings() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["foo"];
    assert_eq!(v[0], "foo".to_string());
}

#[test]
fn test_three_elements() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["x", "y", "z"];
    assert_eq!(v, vec!["x".to_string(), "y".to_string(), "z".to_string()]);
}

#[test]
fn test_unicode() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["🦀", "rust"];
    assert_eq!(v[0], "🦀");
}

#[test]
fn test_order_preserved() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["first", "second", "third"];
    assert_eq!(v[0], "first");
    assert_eq!(v[2], "third");
}

#[test]
fn test_contains() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["hello", "world"];
    assert!(v.contains(&"hello".to_string()));
}

#[test]
fn test_five_elements() {
    use macro_vec_strings::vec_of_strings;
    let v: Vec<String> = vec_of_strings!["a", "b", "c", "d", "e"];
    assert_eq!(v.len(), 5);
}
