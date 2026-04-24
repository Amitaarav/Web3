/*
  Problem 75: Shared State — RwLock Configuration

  Define a Config struct with a HashMap<String, String>. Wrap it in Arc<RwLock<Config>>.
  Write a function that spawns 5 reader threads and 1 writer thread.
  Readers should read a specific key, while the writer updates it.
  Return the final value of the configuration key.

  Run the tests for this problem with:
    cargo test --test rwlock_config_test
*/

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Config {
    pub settings: HashMap<String, String>,
}

pub fn update_and_read_config() -> String {
    todo!()
}
