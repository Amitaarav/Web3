/*
  Problem 72: Threads — Move Closure

  Write a function that takes a String, spawns a thread that takes ownership
  of that string (using a move closure), and returns the thread's JoinHandle.
  The thread should simply return the length of the string.

  Run the tests for this problem with:
    cargo test --test move_closure_test
*/

use std::thread;

pub fn spawn_move_thread(s: String) -> thread::JoinHandle<usize> {
    todo!()
}
