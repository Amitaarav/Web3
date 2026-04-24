#[path = "../src/medium/workspace_deps.rs"]
mod workspace_deps;
use workspace_deps::is_crate_active;


#[test]
fn test_active_crates() {
    assert!(is_crate_active("fundamentals"));
    assert!(is_crate_active("intermediate"));
    assert!(is_crate_active("async"));
}

#[test]
fn test_inactive_crates() {
    assert!(!is_crate_active("unknown-crate"));
    assert!(!is_crate_active(""));
}

#[test]
fn test_fundamentals_active() {
    assert!(is_crate_active("fundamentals"));
}

#[test]
fn test_intermediate_active() {
    assert!(is_crate_active("intermediate"));
}

#[test]
fn test_async_active() {
    assert!(is_crate_active("async"));
}

#[test]
fn test_random_name_inactive() {
    assert!(!is_crate_active("notacrate"));
}

#[test]
fn test_partial_name_inactive() {
    assert!(!is_crate_active("fund"));
}

#[test]
fn test_uppercase_inactive() {
    assert!(!is_crate_active("Fundamentals"));
}

#[test]
fn test_space_name_inactive() {
    assert!(!is_crate_active(" fundamentals"));
}

#[test]
fn test_numeric_name_inactive() {
    assert!(!is_crate_active("123"));
}
