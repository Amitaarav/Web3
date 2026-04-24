#[path = "../src/hard/bump_allocator.rs"]
mod bump_allocator;
use bump_allocator::BumpAllocator;


#[test]
fn test_alloc_ok() {
    let mut bump = BumpAllocator::new();
    let slice = bump.alloc(10).unwrap();
    assert_eq!(slice.len(), 10);
    assert_eq!(bump.cursor, 10);
}

#[test]
fn test_multiple_allocs() {
    let mut bump = BumpAllocator::new();
    let _s1 = bump.alloc(100).unwrap();
    let _s2 = bump.alloc(50).unwrap();
    assert_eq!(bump.cursor, 150);
}

#[test]
fn test_out_of_memory() {
    let mut bump = BumpAllocator::new();
    let _s1 = bump.alloc(1000).unwrap();
    assert!(bump.alloc(50).is_err());
}

#[test]
fn test_exact_memory() {
    let mut bump = BumpAllocator::new();
    assert!(bump.alloc(1024).is_ok());
    assert_eq!(bump.cursor, 1024);
}

#[test]
fn test_initial_cursor_is_zero() {
    let bump = BumpAllocator::new();
    assert_eq!(bump.cursor, 0);
}

#[test]
fn test_zero_size_alloc() {
    let mut bump = BumpAllocator::new();
    let slice = bump.alloc(0).unwrap();
    assert_eq!(slice.len(), 0);
    assert_eq!(bump.cursor, 0);
}

#[test]
fn test_alloc_one_byte() {
    let mut bump = BumpAllocator::new();
    let slice = bump.alloc(1).unwrap();
    assert_eq!(slice.len(), 1);
    assert_eq!(bump.cursor, 1);
}

#[test]
fn test_three_allocs_cursor() {
    let mut bump = BumpAllocator::new();
    bump.alloc(10).unwrap();
    bump.alloc(20).unwrap();
    bump.alloc(30).unwrap();
    assert_eq!(bump.cursor, 60);
}

#[test]
fn test_over_limit_fails() {
    let mut bump = BumpAllocator::new();
    assert!(bump.alloc(1025).is_err());
}

#[test]
fn test_alloc_returns_correct_length() {
    let mut bump = BumpAllocator::new();
    let slice = bump.alloc(64).unwrap();
    assert_eq!(slice.len(), 64);
}
