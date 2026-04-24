/*
  Problem 89: Message Passing — Broadcast Channel

  Write an async function broadcast_demo() that creates a
  tokio::sync::broadcast channel. Spawn 3 tasks that each subscribe to the
  channel and receive a message. The main function should send a message
  and ensure all tasks receive it. Return the sum of received values.

  Run the tests for this problem with:
    cargo test --test broadcast_message_test
*/

use tokio::sync::broadcast;

pub async fn broadcast_demo() -> i32 {
    todo!()
}
