# Final Project Hints - Level 3: Production-Ready Features

## 🎯 Prerequisites from Level 2
- Request parsing handles large requests properly
- Router supports path parameters 
- Load tester shows true concurrent performance
- Integration tests pass consistently

## 🚀 Advanced Production Features

### 1. **HTTP Keep-Alive Connections**

**Current Problem**: Server closes connection after each request
```rust
// Current: Connection: close
// Should support: Connection: keep-alive
```

**Implementation Strategy**:
```rust
async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Read request
        let request = parse_request(&mut stream).await?;
        
        // Check if keep-alive
        let keep_alive = request.headers.get("Connection")
            .map(|v| v.to_lowercase() == "keep-alive")
            .unwrap_or(false);
        
        // Generate response
        let response = router.handle_request(&request);
        
        // Send response
        stream.write_all(&response.to_bytes()).await?;
        
        if !keep_alive {
            break;
        }
    }
    Ok(())
}
```

### 2. **Static File Serving with Zero-Copy**

**Add a new route for static files**:
```rust
router.add_route("/static/*", |req| {
    serve_static_file(&req.path)
});
```

**Zero-Copy Implementation**:
```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

async fn serve_static_file(path: &str) -> Response {
    // Strip /static/ prefix
    let file_path = path.strip_prefix("/static/").unwrap_or(path);
    
    // Security: Prevent directory traversal
    if file_path.contains("..") {
        return Response::new(403, "Forbidden".to_string());
    }
    
    match File::open(format!("static/{}", file_path)).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            match file.read_to_end(&mut contents).await {
                Ok(_) => {
                    // TODO: Use proper MIME type detection
                    Response::new(200, String::from_utf8_lossy(&contents).to_string())
                }
                Err(_) => Response::new(500, "Internal Server Error".to_string()),
            }
        }
        Err(_) => Response::new(404, "File Not Found".to_string()),
    }
}
```

### 3. **Advanced Router with Trie-Based Matching**

**Current O(n) routing → O(log n) with trie**:
```rust
use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<String, TrieNode>,
    handler: Option<Box<dyn Fn(&Request) -> Response + Send + Sync>>,
    is_parameter: bool,
    parameter_name: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            handler: None,
            is_parameter: false,
            parameter_name: None,
        }
    }
}

struct AdvancedRouter {
    root: TrieNode,
}

impl AdvancedRouter {
    fn new() -> Self {
        AdvancedRouter {
            root: TrieNode::new(),
        }
    }
    
    fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &mut self.root;
        
        for segment in segments {
            if segment.starts_with(':') {
                // Parameter route
                let param_name = segment.trim_start_matches(':');
                let key = ":param".to_string();
                
                current.children.entry(key.clone()).or_insert_with(|| {
                    let mut node = TrieNode::new();
                    node.is_parameter = true;
                    node.parameter_name = Some(param_name.to_string());
                    node
                });
                
                current = current.children.get_mut(&key).unwrap();
            } else {
                current.children.entry(segment.to_string()).or_insert_with(TrieNode::new);
                current = current.children.get_mut(segment).unwrap();
            }
        }
        
        current.handler = Some(Box::new(handler));
    }
    
    fn handle_request(&self, request: &Request) -> Response {
        let segments: Vec<&str> = request.path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;
        let mut params = HashMap::new();
        
        for segment in segments {
            if let Some(child) = current.children.get(segment) {
                current = child;
            } else if let Some(param_node) = current.children.get(":param") {
                if let Some(param_name) = &param_node.parameter_name {
                    params.insert(param_name.clone(), segment.to_string());
                }
                current = param_node;
            } else {
                return Response::new(404, "Not Found".to_string());
            }
        }
        
        if let Some(handler) = &current.handler {
            // TODO: Pass parameters to handler
            handler(request)
        } else {
            Response::new(404, "Not Found".to_string())
        }
    }
}
```

### 4. **Error Handling and Logging**

**Structured Error Types**:
```rust
#[derive(Debug)]
enum ServerError {
    IoError(std::io::Error),
    ParseError(String),
    TimeoutError,
    TooManyConnections,
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::IoError(e) => write!(f, "IO error: {}", e),
            ServerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ServerError::TimeoutError => write!(f, "Request timeout"),
            ServerError::TooManyConnections => write!(f, "Too many connections"),
        }
    }
}

impl std::error::Error for ServerError {}
```

**Comprehensive Logging**:
```rust
use tracing::{info, warn, error, debug};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
    
    info!("Starting Rust Web Server v1.0");
    
    // ... rest of main function
}
```

### 5. **Connection Limiting and Rate Limiting**

**Connection Counting**:
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct ConnectionCounter {
    count: AtomicUsize,
    max_connections: usize,
}

impl ConnectionCounter {
    fn new(max_connections: usize) -> Self {
        ConnectionCounter {
            count: AtomicUsize::new(0),
            max_connections,
        }
    }
    
    fn try_acquire(&self) -> bool {
        let current = self.count.load(Ordering::SeqCst);
        if current < self.max_connections {
            self.count.fetch_add(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
    
    fn release(&self) {
        self.count.fetch_sub(1, Ordering::SeqCst);
    }
}
```

**Rate Limiting per IP**:
```rust
use dashmap::DashMap;
use std::time::{Duration, Instant};

struct RateLimiter {
    requests: DashMap<String, Vec<Instant>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: usize, window: Duration) -> Self {
        RateLimiter {
            requests: DashMap::new(),
            max_requests,
            window,
        }
    }
    
    fn is_allowed(&self, ip: &str) -> bool {
        let now = Instant::now();
        let mut entry = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);
        
        // Remove old entries
        entry.retain(|&time| now.duration_since(time) < self.window);
        
        if entry.len() < self.max_requests {
            entry.push(now);
            true
        } else {
            false
        }
    }
}
```

## 🎯 Success Criteria for Level 3

Your production-ready server should have:
- [ ] HTTP keep-alive connection support
- [ ] Static file serving with proper MIME types
- [ ] Advanced router with path parameters
- [ ] Structured error handling and logging
- [ ] Connection limiting (max 1000 concurrent)
- [ ] Rate limiting (100 requests/minute per IP)
- [ ] Graceful shutdown handling
- [ ] Health check endpoint (`/health`)

## 📊 Performance Benchmarks

**Target Performance** (compared to your original broken server):
- **Throughput**: 10,000+ requests/second (vs ~10 req/sec)
- **Latency**: p99 < 50ms (vs timeouts)
- **Concurrency**: 1,000+ concurrent connections (vs 1)
- **Memory**: < 50MB RSS (vs growing endlessly)

## 🧪 Advanced Testing

### Load Testing with Keep-Alive:
```bash
# Test with apache bench
ab -n 10000 -c 100 -k http://localhost:8080/

# Test with wrk
wrk -t12 -c400 -d30s --latency http://localhost:8080/
```

### Static File Performance:
```bash
# Create test files
mkdir -p static
echo "Hello, World!" > static/hello.txt
dd if=/dev/zero of=static/large.bin bs=1M count=10

# Test serving
curl http://localhost:8080/static/hello.txt
curl http://localhost:8080/static/large.bin | wc -c
```

## 🤔 Advanced Questions

1. "How can I implement zero-copy file serving with `sendfile()`?"
2. "What's the difference between connection pooling and keep-alive?"
3. "How do I implement proper graceful shutdown with connection draining?"
4. "What's the optimal buffer size for different types of requests?"

## 📚 Advanced Topics to Explore

- **HTTP/2 Support**: Add h2 crate integration
- **WebSocket Support**: Upgrade connections for real-time features
- **Compression**: Implement gzip/deflate response compression
- **Caching**: Add in-memory response caching with TTL
- **TLS/SSL**: Add rustls for HTTPS support

---

**Time Investment**: Plan 4-6 hours for this level. This is production-ready server territory!

**Completion**: You'll have built a web server that rivals established solutions for specific use cases. Compare your performance metrics with nginx or Apache for similar workloads!