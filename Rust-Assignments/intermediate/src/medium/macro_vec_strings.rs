/*
  Problem 56: Declarative Macro — vec_of_strings!

  Write a declarative macro vec_of_strings! that takes a comma-separated list of
  string literals and produces a Vec<String>. For example,
  vec_of_strings!["hello", "world"] should produce
  vec!["hello".to_string(), "world".to_string()].

  Run the tests for this problem with:
    cargo test --test macro_vec_strings_test
*/

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        todo!()
    };
}
