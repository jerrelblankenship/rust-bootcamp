# Web Server Debugging Checklist

## 🔍 Common Issues and Solutions

### 1. **Server Won't Start**
- [ ] Check if port 8080 is already in use: `lsof -i :8080`
- [ ] Verify dependencies are installed: `cargo check`
- [ ] Check for compilation errors: `cargo build`
- [ ] Ensure Tokio runtime is properly configured

### 2. **Requests Time Out**
- [ ] Is the server handling requests sequentially instead of concurrently?
- [ ] Check for blocking operations in async code
- [ ] Verify `tokio::spawn` is used for concurrent connections
- [ ] Look for deadlocks in shared state access

### 3. **Connection Refused Errors**
- [ ] Server is binding to correct address (`127.0.0.1:8080`)
- [ ] Firewall not blocking the port
- [ ] Server is actually running and listening
- [ ] Check server logs for crash/panic messages

### 4. **HTTP Parsing Failures**
- [ ] Buffer size too small for large requests
- [ ] Incomplete request reading (partial TCP packets)
- [ ] Header parsing doesn't handle edge cases
- [ ] Missing `\r\n\r\n` delimiter handling

### 5. **Memory Issues**
- [ ] Unnecessary string allocations in hot paths
- [ ] Large buffers allocated per connection
- [ ] Memory leaks from unclosed connections
- [ ] Excessive cloning of request/response data

### 6. **Performance Problems**
- [ ] Router using linear search instead of hash map
- [ ] Response generation creates multiple string allocations
- [ ] No connection reuse (creating new connections per request)
- [ ] Blocking I/O operations in async context

### 7. **Integration Test Failures**
- [ ] Server not actually running during tests
- [ ] Race conditions in test setup/teardown
- [ ] Tests expecting specific response format
- [ ] Timeout values too low for slow CI systems

### 8. **Load Testing Issues**
- [ ] Load tester not actually testing concurrency
- [ ] Sequential awaiting of concurrent operations
- [ ] Connection overhead not accounted for
- [ ] Unrealistic test parameters

## 🚨 Critical Bugs to Fix First

### Priority 1: Concurrency Bug
**Symptom**: Server can only handle one request at a time
**Location**: Main server loop in `src/main.rs`
**Fix**: Use `tokio::spawn` to handle connections concurrently

### Priority 2: Request Parsing
**Symptom**: Server crashes or returns errors on various requests
**Location**: `parse_request` function
**Fix**: Handle large requests and edge cases properly

### Priority 3: Load Tester Concurrency
**Symptom**: Load tester shows false concurrency metrics
**Location**: `src/bin/loadtest.rs`
**Fix**: Properly await concurrent operations

## 🔧 Debugging Techniques

### 1. **Add Logging**
```rust
use tracing::{info, warn, error, debug};

// Add to connection handler
info!("Handling connection from {}", addr);
debug!("Request: {:?}", request);
```

### 2. **Use Rust Compiler**
```bash
# Check for compilation errors
cargo check

# Get more detailed error info
cargo check --verbose
```

### 3. **Run Tests Individually**
```bash
# Run specific test
cargo test test_concurrent_requests

# Run with output
cargo test -- --nocapture
```

### 4. **Monitor System Resources**
```bash
# Check server resource usage
top -p $(pgrep rust-web-server)

# Monitor network connections
netstat -an | grep 8080
```

### 5. **Profile Performance**
```bash
# Install flamegraph
cargo install flamegraph

# Profile the server
FLAMEGRAPH=1 cargo run --bin server
```

## 📊 Performance Benchmarks

### Expected Metrics (After Fixes)
- **Concurrency**: 100+ simultaneous connections
- **Throughput**: 1,000+ requests/second
- **Latency**: < 100ms for simple requests
- **Memory**: < 10MB for idle server

### Comparison Points
- **Before Fix**: ~1 req/sec (sequential)
- **After Level 1**: ~1,000 req/sec (concurrent)
- **After Level 2**: ~5,000 req/sec (optimized)
- **After Level 3**: ~10,000 req/sec (production)

## 🧪 Testing Strategy

### Unit Tests
- Test individual functions in isolation
- Mock external dependencies
- Focus on edge cases and error conditions

### Integration Tests
- Test full request/response cycle
- Verify concurrent behavior
- Test error handling and recovery

### Load Tests
- Measure performance under realistic load
- Test connection limits and timeouts
- Verify memory usage stays bounded

### Manual Testing
```bash
# Basic functionality
curl http://localhost:8080/

# Concurrent requests
curl http://localhost:8080/ & curl http://localhost:8080/ &

# Large request
curl -X POST -d "$(printf 'x%.0s' {1..2000})" http://localhost:8080/

# Malformed request
echo "INVALID" | nc localhost 8080
```

## 🎯 Success Indicators

### Level 1 Complete
- [ ] `cargo test` passes
- [ ] Server handles multiple curl requests simultaneously
- [ ] No compilation errors or warnings
- [ ] Basic HTTP parsing works correctly

### Level 2 Complete
- [ ] Load tester shows >100 req/sec
- [ ] Large requests (>1KB) work correctly
- [ ] Path parameters functional
- [ ] Integration tests consistently pass

### Level 3 Complete
- [ ] Keep-alive connections working
- [ ] Static file serving implemented
- [ ] Rate limiting functional
- [ ] Production-ready error handling

## 🔄 Iterative Debugging Process

1. **Identify**: Run tests, find what's failing
2. **Isolate**: Create minimal reproduction case
3. **Fix**: Make targeted changes
4. **Test**: Verify fix works and doesn't break other things
5. **Repeat**: Move to next issue

## 📚 Common Rust Patterns for Web Servers

### Sharing State Between Connections
```rust
let shared_state = Arc::new(AppState::new());
let state_clone = Arc::clone(&shared_state);
```

### Async Error Handling
```rust
match handle_request().await {
    Ok(response) => send_response(response).await,
    Err(e) => send_error_response(e).await,
}
```

### Graceful Shutdown
```rust
tokio::select! {
    _ = server_task => {},
    _ = shutdown_signal => {
        info!("Graceful shutdown initiated");
    }
}
```

Remember: **Fix one issue at a time** and test thoroughly before moving to the next problem!