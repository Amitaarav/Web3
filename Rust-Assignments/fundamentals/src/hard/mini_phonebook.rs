/*
  Problem 35: Mini Phonebook

  Build a Phonebook struct backed by a HashMap<String, String> (name -> phone).
  Implement methods new, add, lookup, and remove.
  Also implement the Display trait to print all entries sorted by name, one per line
  as "Name: Phone".

  Run the tests for this problem with:
    cargo test --test mini_phonebook_test
*/

use std::collections::HashMap;
use std::fmt;

pub struct Phonebook {
    pub entries: HashMap<String, String>,
}

impl Phonebook {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, name: &str, phone: &str) {
        todo!()
    }

    pub fn lookup(&self, name: &str) -> Option<&String> {
        todo!()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        todo!()
    }
}

impl fmt::Display for Phonebook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
