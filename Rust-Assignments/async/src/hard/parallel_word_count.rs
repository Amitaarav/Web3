/*
  Problem 81: Parallel Word Count

  Write a function that takes a Vec<String> (lines of text) and counts the
  occurrences of words in parallel. Split the lines among 4 threads. Each thread
  computes a local HashMap, and then the main thread merges them into a final
  HashMap<String, usize>.

  Run the tests for this problem with:
    cargo test --test parallel_word_count_test
*/

use std::collections::HashMap;
use std::thread;

pub fn parallel_word_count(lines: Vec<String>) -> HashMap<String, usize> {
    todo!()
}
