#[path = "../src/medium/hashmap_groupby.rs"]
mod hashmap_groupby;
use hashmap_groupby::group_by_first_letter;


#[test]
fn test_basic() {
    let words = vec!["apple", "avocado", "banana", "blueberry"]
        .into_iter().map(String::from).collect();
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'a').unwrap().len(), 2);
    assert_eq!(result.get(&'b').unwrap().len(), 2);
}

#[test]
fn test_empty() {
    let result = group_by_first_letter(vec![]);
    assert!(result.is_empty());
}

#[test]
fn test_single_word() {
    let words = vec![String::from("rust")];
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'r').unwrap(), &vec!["rust".to_string()]);
}

#[test]
fn test_ignores_empty_strings() {
    let words = vec![String::from(""), String::from("hello")];
    let result = group_by_first_letter(words);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_all_different_letters() {
    let words = vec!["alpha", "beta", "gamma"].into_iter().map(String::from).collect();
    let result = group_by_first_letter(words);
    assert_eq!(result.len(), 3);
}

#[test]
fn test_three_same_letter() {
    let words = vec!["ant", "ape", "arm"].into_iter().map(String::from).collect();
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'a').unwrap().len(), 3);
}

#[test]
fn test_key_is_first_char() {
    let words = vec![String::from("zebra")];
    let result = group_by_first_letter(words);
    assert!(result.contains_key(&'z'));
}

#[test]
fn test_multiple_groups() {
    let words = vec!["cat", "car", "dog", "deer", "eel"]
        .into_iter().map(String::from).collect();
    let result = group_by_first_letter(words);
    assert_eq!(result.get(&'c').unwrap().len(), 2);
    assert_eq!(result.get(&'d').unwrap().len(), 2);
    assert_eq!(result.get(&'e').unwrap().len(), 1);
}

#[test]
fn test_values_contain_full_word() {
    let words = vec![String::from("hello")];
    let result = group_by_first_letter(words);
    assert!(result.get(&'h').unwrap().contains(&"hello".to_string()));
}
