#[path = "../src/hard/mini_phonebook.rs"]
mod mini_phonebook;
use mini_phonebook::Phonebook;


#[test]
fn test_add_and_lookup() {
    let mut pb = Phonebook::new();
    pb.add("Alice", "123-4567");
    assert_eq!(pb.lookup("Alice"), Some(&"123-4567".to_string()));
}

#[test]
fn test_lookup_missing() {
    let pb = Phonebook::new();
    assert_eq!(pb.lookup("Nobody"), None);
}

#[test]
fn test_remove() {
    let mut pb = Phonebook::new();
    pb.add("Bob", "999-0000");
    assert!(pb.remove("Bob"));
    assert_eq!(pb.lookup("Bob"), None);
}

#[test]
fn test_remove_missing() {
    let mut pb = Phonebook::new();
    assert!(!pb.remove("Ghost"));
}

#[test]
fn test_display_sorted() {
    let mut pb = Phonebook::new();
    pb.add("Charlie", "111");
    pb.add("Alice", "222");
    let display = format!("{}", pb);
    assert!(display.starts_with("Alice: 222"));
    assert!(display.contains("Charlie: 111"));
}

#[test]
fn test_overwrite_entry() {
    let mut pb = Phonebook::new();
    pb.add("Alice", "123-0000");
    pb.add("Alice", "999-1111");
    assert_eq!(pb.lookup("Alice"), Some(&"999-1111".to_string()));
}

#[test]
fn test_multiple_entries() {
    let mut pb = Phonebook::new();
    pb.add("Alice", "111");
    pb.add("Bob", "222");
    pb.add("Carol", "333");
    assert_eq!(pb.lookup("Bob"), Some(&"222".to_string()));
}

#[test]
fn test_remove_then_readd() {
    let mut pb = Phonebook::new();
    pb.add("Dave", "555");
    pb.remove("Dave");
    pb.add("Dave", "999");
    assert_eq!(pb.lookup("Dave"), Some(&"999".to_string()));
}

#[test]
fn test_lookup_case_sensitive() {
    let mut pb = Phonebook::new();
    pb.add("alice", "123");
    assert_eq!(pb.lookup("Alice"), None);
}

#[test]
fn test_remove_returns_false_second_time() {
    let mut pb = Phonebook::new();
    pb.add("Eve", "000");
    pb.remove("Eve");
    assert!(!pb.remove("Eve"));
}
