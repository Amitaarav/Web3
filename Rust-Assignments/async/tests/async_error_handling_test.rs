#[path = "../src/medium/async_error_handling.rs"]
mod async_error_handling;
use async_error_handling::{fetch_and_process, fetch_data};


#[tokio::test]
async fn test_ok() {
    assert_eq!(fetch_and_process(10).await, Ok(7)); // "Data_10"
}

#[tokio::test]
async fn test_err() {
    assert_eq!(fetch_and_process(0).await, Err("Invalid ID".to_string()));
}

#[tokio::test]
async fn test_data_ok() {
    assert_eq!(fetch_data(1).await, Ok("Data_1".to_string()));
}

#[tokio::test]
async fn test_data_5() {
    assert_eq!(fetch_data(5).await, Ok("Data_5".to_string()));
}

#[tokio::test]
async fn test_data_zero_err() {
    assert!(fetch_data(0).await.is_err());
}

#[tokio::test]
async fn test_fetch_and_process_100() {
    assert_eq!(fetch_and_process(100).await, Ok(8)); // "Data_100".len() == 8
}

#[tokio::test]
async fn test_fetch_and_process_err_msg() {
    let result = fetch_and_process(0).await;
    assert_eq!(result, Err("Invalid ID".to_string()));
}

#[tokio::test]
async fn test_data_returns_ok_for_nonzero() {
    assert!(fetch_data(99).await.is_ok());
}

#[tokio::test]
async fn test_process_returns_string_length() {
    let len = fetch_and_process(1).await.unwrap();
    assert_eq!(len, "Data_1".len());
}
