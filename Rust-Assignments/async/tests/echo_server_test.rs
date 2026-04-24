#[path = "../src/hard/echo_server.rs"]
mod echo_server;
use echo_server::run_echo_server;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_echo() {
    let port = 8080;
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        run_echo_server(port).await.unwrap()
    });

    // Wait a bit for server to start
    sleep(Duration::from_millis(50)).await;

    // Client connects
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"hello").await.unwrap();

    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"hello");

    let server_result = server_handle.await.unwrap();
    assert_eq!(server_result, b"hello".to_vec());
}

#[tokio::test]
async fn test_echo_world() {
    let port = 8081;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"world").await.unwrap();
    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"world");
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_echo_returns_vec() {
    let port = 8082;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"test").await.unwrap();
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await.unwrap();
    let result = server_handle.await.unwrap();
    assert_eq!(result, b"test".to_vec());
}

#[tokio::test]
async fn test_echo_single_byte() {
    let port = 8083;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"a").await.unwrap();
    let mut buf = [0u8; 1];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"a");
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_echo_response_length() {
    let port = 8084;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"rustlang").await.unwrap();
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(buf.len(), 8);
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_echo_nonempty_result() {
    let port = 8085;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"hi").await.unwrap();
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await.unwrap();
    let result: Vec<u8> = server_handle.await.unwrap();
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_echo_numeric() {
    let port = 8086;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"12345").await.unwrap();
    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"12345");
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_echo_response_matches_input() {
    let port = 8087;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"async").await.unwrap();
    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"async");
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_echo_result_len() {
    let port = 8088;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"rust").await.unwrap();
    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf).await.unwrap();
    let result = server_handle.await.unwrap();
    assert_eq!(result.len(), 4);
}

#[tokio::test]
async fn test_echo_three_chars() {
    let port = 8089;
    let server_handle = tokio::spawn(async move { run_echo_server(port).await.unwrap() });
    sleep(Duration::from_millis(50)).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();
    stream.write_all(b"abc").await.unwrap();
    let mut buf = [0u8; 3];
    stream.read_exact(&mut buf).await.unwrap();
    assert_eq!(&buf, b"abc");
    let _ = server_handle.await.unwrap();
}
