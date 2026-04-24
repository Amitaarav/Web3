/*
  Problem 93: Async Error Handling

  Write an async function fetch_data(id: u32) -> Result<String, String>.
  If id is 0, return Err("Invalid ID"). Otherwise, sleep for 5ms and return
  Ok("Data_<id>"). Use the ? operator in another async function
  fetch_and_process(id: u32) -> Result<usize, String> to call fetch_data
  and return the length of the string.

  Run the tests for this problem with:
    cargo test --test async_error_handling_test
*/

pub async fn fetch_data(id: u32) -> Result<String, String> {
    todo!()
}

pub async fn fetch_and_process(id: u32) -> Result<usize, String> {
    todo!()
}
