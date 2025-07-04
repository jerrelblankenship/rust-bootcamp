use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn, error};

#[derive(Debug, Clone)]
struct Config {
    port: u16,
    max_connections: usize,
    static_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8080,
            max_connections: 1000,
            static_dir: "static".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
struct Response {
    status_code: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn new(status_code: u16, body: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Server".to_string(), "RustWebServer/1.0".to_string());
        
        Response {
            status_code,
            headers,
            body,
        }
    }
    
    fn to_bytes(&self) -> Vec<u8> {
        let status_line = format!("HTTP/1.1 {} OK\r\n", self.status_code);
        let headers: String = self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .collect();
        
        format!("{}{}\r\n{}", status_line, headers, self.body).into_bytes()
    }
}

// FIXME: This function has multiple parsing bugs
fn parse_request(data: &[u8]) -> Result<Request, Box<dyn std::error::Error>> {
    let request_str = String::from_utf8_lossy(data);
    let lines: Vec<&str> = request_str.lines().collect();
    
    if lines.is_empty() {
        return Err("Empty request".into());
    }
    
    // TODO: Parse the request line properly
    let request_line_parts: Vec<&str> = lines[0].split(' ').collect();
    if request_line_parts.len() != 3 {
        return Err("Invalid request line".into());
    }
    
    let method = request_line_parts[0].to_string();
    let path = request_line_parts[1].to_string();
    
    // FIXME: Header parsing is incomplete
    let mut headers = HashMap::new();
    let mut body_start = 0;
    
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        
        // TODO: Handle header parsing edge cases
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }
    
    // TODO: Extract body properly
    let body = if body_start < lines.len() {
        lines[body_start..].join("\n")
    } else {
        String::new()
    };
    
    Ok(Request {
        method,
        path,
        headers,
        body,
    })
}

// FIXME: This router is extremely inefficient and broken
struct Router {
    routes: HashMap<String, Box<dyn Fn(&Request) -> Response + Send + Sync>>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }
    
    fn add_route<F>(&mut self, path: &str, handler: F) 
    where 
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }
    
    // TODO: Implement proper routing with path parameters
    fn handle_request(&self, request: &Request) -> Response {
        // FIXME: This doesn't handle path parameters or wildcards
        if let Some(handler) = self.routes.get(&request.path) {
            handler(request)
        } else {
            Response::new(404, "Not Found".to_string())
        }
    }
}

async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Fix buffer size issues
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    
    if bytes_read == 0 {
        return Ok(());
    }
    
    // FIXME: This doesn't handle partial reads or large requests
    let request = parse_request(&buffer[..bytes_read])?;
    let response = router.handle_request(&request);
    
    // TODO: Handle write failures properly
    stream.write_all(&response.to_bytes()).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let config = Config::default();
    let mut router = Router::new();
    
    // Add some basic routes
    router.add_route("/", |_req| {
        Response::new(200, "<h1>Welcome to Rust Web Server!</h1>".to_string())
    });
    
    router.add_route("/hello", |_req| {
        Response::new(200, "<h1>Hello, World!</h1>".to_string())
    });
    
    // TODO: Add more routes (/api/status, /api/health, etc.)
    
    let router = Arc::new(router);
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;
    info!("Server running on http://127.0.0.1:{}", config.port);
    
    loop {
        let (stream, addr) = listener.accept().await?;
        let router = Arc::clone(&router);
        
        // FIXME: This is a major concurrency bug!
        // The server handles connections sequentially instead of concurrently
        match handle_connection(stream, router).await {
            Ok(_) => info!("Handled connection from {}", addr),
            Err(e) => error!("Error handling connection from {}: {}", addr, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_request() {
        let request_data = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let request = parse_request(request_data).unwrap();
        
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/");
        assert_eq!(request.headers.get("Host"), Some(&"localhost".to_string()));
    }
    
    #[test]
    fn test_response_formatting() {
        let response = Response::new(200, "Hello".to_string());
        let bytes = response.to_bytes();
        let response_str = String::from_utf8(bytes).unwrap();
        
        assert!(response_str.contains("HTTP/1.1 200 OK"));
        assert!(response_str.contains("Hello"));
    }
    
    // TODO: Add more comprehensive tests
}