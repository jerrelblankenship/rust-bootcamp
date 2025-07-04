use std::time::Duration;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;

// Helper function to start the server for testing
async fn start_test_server() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Start server in background for testing
    // This is currently broken - need to implement proper test server setup
    Ok(())
}

async fn send_http_request(url: &str, request: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(url).await?;
    
    stream.write_all(request.as_bytes()).await?;
    
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer).await?;
    
    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
}

#[tokio::test]
async fn test_basic_http_request() {
    // FIXME: This test will fail because no server is running
    let request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    
    match timeout(Duration::from_secs(5), send_http_request("127.0.0.1:8080", request)).await {
        Ok(Ok(response)) => {
            assert!(response.contains("HTTP/1.1 200 OK"));
            assert!(response.contains("Welcome to Rust Web Server!"));
        }
        Ok(Err(e)) => {
            panic!("Request failed: {}", e);
        }
        Err(_) => {
            panic!("Request timed out - is the server running?");
        }
    }
}

#[tokio::test]
async fn test_hello_endpoint() {
    // FIXME: This test assumes server is running
    let request = "GET /hello HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    
    match timeout(Duration::from_secs(5), send_http_request("127.0.0.1:8080", request)).await {
        Ok(Ok(response)) => {
            assert!(response.contains("HTTP/1.1 200 OK"));
            assert!(response.contains("Hello, World!"));
        }
        Ok(Err(e)) => {
            panic!("Request failed: {}", e);
        }
        Err(_) => {
            panic!("Request timed out - is the server running?");
        }
    }
}

#[tokio::test]
async fn test_404_response() {
    // FIXME: This test needs server running
    let request = "GET /nonexistent HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    
    match timeout(Duration::from_secs(5), send_http_request("127.0.0.1:8080", request)).await {
        Ok(Ok(response)) => {
            assert!(response.contains("HTTP/1.1 404"));
            assert!(response.contains("Not Found"));
        }
        Ok(Err(e)) => {
            panic!("Request failed: {}", e);
        }
        Err(_) => {
            panic!("Request timed out - is the server running?");
        }
    }
}

#[tokio::test]
async fn test_concurrent_requests() {
    // FIXME: This test will fail due to server's concurrency bug
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            let request = format!("GET /hello HTTP/1.1\r\nHost: localhost\r\nUser-Agent: test-{}\r\nConnection: close\r\n\r\n", i);
            
            match timeout(Duration::from_secs(10), send_http_request("127.0.0.1:8080", &request)).await {
                Ok(Ok(response)) => {
                    assert!(response.contains("HTTP/1.1 200 OK"));
                    Ok(())
                }
                Ok(Err(e)) => Err(format!("Request {} failed: {}", i, e)),
                Err(_) => Err(format!("Request {} timed out", i)),
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => {
                // Success
            }
            Ok(Err(e)) => {
                panic!("Concurrent request failed: {}", e);
            }
            Err(e) => {
                panic!("Task join failed: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_malformed_request() {
    // TODO: Test server's handling of malformed requests
    let malformed_request = "INVALID REQUEST\r\n\r\n";
    
    match timeout(Duration::from_secs(5), send_http_request("127.0.0.1:8080", malformed_request)).await {
        Ok(Ok(response)) => {
            // TODO: Server should return 400 Bad Request
            println!("Response to malformed request: {}", response);
        }
        Ok(Err(_)) => {
            // Expected - server might close connection
        }
        Err(_) => {
            panic!("Request timed out");
        }
    }
}

#[tokio::test]
async fn test_large_request() {
    // FIXME: This will fail due to small buffer size in server
    let large_body = "x".repeat(2048);
    let request = format!("POST /echo HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", large_body.len(), large_body);
    
    match timeout(Duration::from_secs(5), send_http_request("127.0.0.1:8080", &request)).await {
        Ok(Ok(response)) => {
            // TODO: Server should handle large requests properly
            println!("Response to large request: {}", response);
        }
        Ok(Err(e)) => {
            println!("Large request failed (expected): {}", e);
        }
        Err(_) => {
            panic!("Request timed out");
        }
    }
}

// TODO: Add more comprehensive tests:
// - Test keep-alive connections
// - Test different HTTP methods
// - Test header parsing edge cases
// - Test error handling
// - Test performance under load