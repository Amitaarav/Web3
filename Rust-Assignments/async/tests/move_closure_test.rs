#[path = "../src/easy/move_closure.rs"]
mod move_closure;
use move_closure::spawn_move_thread;


#[test]
fn test_string_length() {
    let handle = spawn_move_thread("rust".to_string());
    assert_eq!(handle.join().unwrap(), 4);
}

#[test]
fn test_empty_string() {
    let handle = spawn_move_thread("".to_string());
    assert_eq!(handle.join().unwrap(), 0);
}

#[test]
fn test_unicode() {
    let handle = spawn_move_thread("🦀".to_string());
    assert_eq!(handle.join().unwrap(), 4); // bytes, not chars
}

#[test]
fn test_single_char() {
    let handle = spawn_move_thread("a".to_string());
    assert_eq!(handle.join().unwrap(), 1);
}

#[test]
fn test_spaces() {
    let handle = spawn_move_thread("   ".to_string());
    assert_eq!(handle.join().unwrap(), 3);
}

#[test]
fn test_numbers_string() {
    let handle = spawn_move_thread("12345".to_string());
    assert_eq!(handle.join().unwrap(), 5);
}

#[test]
fn test_long_string() {
    let s = "a".repeat(100);
    let handle = spawn_move_thread(s);
    assert_eq!(handle.join().unwrap(), 100);
}

#[test]
fn test_handle_completes() {
    let handle = spawn_move_thread("test".to_string());
    assert!(handle.join().is_ok());
}

#[test]
fn test_hello_world() {
    let handle = spawn_move_thread("hello world".to_string());
    assert_eq!(handle.join().unwrap(), 11);
}

#[test]
fn test_ascii_byte_count() {
    let handle = spawn_move_thread("abc".to_string());
    assert_eq!(handle.join().unwrap(), 3);
}
