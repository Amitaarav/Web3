/*
  Problem 8: Ownership Transfer — Append World

  Write a function that takes ownership of a String, appends " world" to it, and returns the modified String.
  The caller should not be able to use the original string after calling this function.

  Run the tests for this problem with:
    cargo test --test append_world_test
*/

pub fn append_world(mut s: String) -> String {
      s.push_str(" world");
      s
}
