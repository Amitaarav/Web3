/*
  Problem 98: Async Fan-out / Fan-in

  Write an async function that takes a Vec<i32>, spawns a tokio task for each
  element to square it (fan-out), awaits all handles (fan-in), and returns
  the sum of the squared values.

  Run the tests for this problem with:
    cargo test --test fan_out_fan_in_test
*/

pub async fn fan_out_fan_in(v: Vec<i32>) -> i32 {
    todo!()
}
