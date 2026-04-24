/*
  Problem 57: Macro — hash_map!

  Write a declarative macro hash_map! that creates a HashMap from a list of
  key => value pairs. For example: hash_map!{ "a" => 1, "b" => 2 }.

  Run the tests for this problem with:
    cargo test --test macro_hash_map_test
*/

#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $val:expr),* $(,)?) => {
        todo!()
    };
}
