# Final Project: Concurrent Web Server Capstone

## 🎯 Project Overview

This is the **capstone project** for your Rust bootcamp - a broken web server implementation that you'll debug and fix using **everything you've learned** from modules 1-10.

As a C# developer, you understand web servers from ASP.NET Core. This project demonstrates how Rust's ownership system, zero-cost abstractions, and fearless concurrency enable building high-performance servers without garbage collection overhead.

**This is NOT a "build from scratch" project** - it's a **"fix the broken code"** challenge that follows the bootcamp's proven pedagogy.

## 🚨 Current Status: BROKEN

The provided web server has **intentional bugs** that prevent it from working:

- ❌ **Major concurrency bug**: Server handles requests sequentially
- ❌ **Request parsing issues**: Fails on large requests and malformed headers  
- ❌ **Load tester problems**: Doesn't actually test concurrency
- ❌ **Integration tests fail**: Multiple test failures due to server bugs
- ❌ **Memory inefficiencies**: Unnecessary allocations in hot paths

## 📚 Knowledge Prerequisites

This project requires concepts from **all previous modules**:

- **Module 1**: Structs, enums, error handling basics
- **Module 2**: Ownership, borrowing, `Arc` for shared state
- **Module 3**: `Result<T, E>`, error propagation with `?`
- **Module 4**: Systems programming, unsafe code, memory layout
- **Module 5**: Async/await, `tokio::spawn`, channels
- **Module 6**: Performance optimization, allocation reduction
- **Module 7**: CLI tools, argument parsing, configuration
- **Module 8**: Testing strategies, mocking, integration tests
- **Module 9**: Crate ecosystem, dependency management
- **Module 10**: Advanced patterns, trait objects, macros

## 🔧 Getting Started

### Step 1: Try Running the Server
```bash
cd final-project
cargo run --bin server
```

### Step 2: Test Basic Functionality
```bash
# In another terminal
curl http://localhost:8080/
```

### Step 3: Identify the Problems
- Does it handle multiple requests concurrently?
- What happens with large requests?
- Do the integration tests pass?

### Step 4: Use the Hint System
- **Stuck after 15+ minutes?** Check `hints/project-level1.md`
- **Need deeper guidance?** Progress to `hints/project-level2.md`
- **Want production features?** Advance to `hints/project-level3.md`

## 📊 Success Metrics

Fix the server to achieve:

**Level 1 (Basic Fix)**:
- ✅ Handles concurrent requests without blocking
- ✅ Basic HTTP parsing works correctly
- ✅ Integration tests pass

**Level 2 (Advanced Fix)**:
- ✅ Supports large requests (>1KB)
- ✅ Path parameters work (`/users/:id`)
- ✅ Load tester shows true concurrency

**Level 3 (Production Ready)**:
- ✅ Keep-alive connections
- ✅ Static file serving
- ✅ Rate limiting and connection limits
- ✅ 1,000+ concurrent connections
- ✅ < 50ms p99 latency

## 🧪 Testing Your Progress

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration
```

### Load Testing
```bash
cargo run --bin loadtest
```

### Benchmarks
```bash
cargo bench
```

## 🔍 Debugging Tools

### Rust Compiler
Your best friend - read compiler errors carefully!

### Logging
```bash
RUST_LOG=debug cargo run --bin server
```

### Performance Profiling
```bash
# Install if needed
cargo install flamegraph
# Profile the server
FLAMEGRAPH=1 cargo run --bin server
```

## 📁 Project Structure

```
final-project/
├── src/
│   ├── main.rs           # Main server (has bugs!)
│   └── bin/
│       └── loadtest.rs   # Load tester (also broken!)
├── tests/
│   └── integration.rs    # Integration tests (currently failing)
├── benches/
│   └── server_benchmarks.rs # Performance benchmarks
├── hints/
│   ├── project-level1.md # Getting started hints
│   ├── project-level2.md # Advanced debugging hints
│   └── project-level3.md # Production features hints
└── README.md            # This file
```

## 🎯 Learning Objectives

By completing this project, you'll demonstrate:

1. **Debugging Skills**: Finding and fixing complex bugs in async Rust code
2. **Concurrency Mastery**: Proper use of `tokio::spawn` and async patterns
3. **Performance Optimization**: Reducing allocations and improving throughput
4. **Testing Proficiency**: Writing and fixing integration tests
5. **Real-world Application**: Building production-ready server features

## 🚀 Hint System Usage

**Progressive Disclosure Learning**:
- Try fixing issues independently first (15+ minutes)
- Use Level 1 hints for conceptual guidance
- Use Level 2 hints for specific technical direction
- Use Level 3 hints for complete solutions

**C# Developer Notes**:
- Hints include comparisons to ASP.NET Core concepts
- Focus on Rust-specific patterns and ownership
- Performance comparisons with .NET equivalents

## 📈 Extension Ideas (After Completion)

1. **HTTP/2 Support**: Add h2 crate integration
2. **WebSocket Support**: Real-time bidirectional communication
3. **TLS/SSL**: Add rustls for HTTPS
4. **Middleware System**: Composable request/response processing
5. **GraphQL Endpoint**: Add juniper or async-graphql
6. **Distributed Tracing**: OpenTelemetry integration

## 🎉 Completion Criteria

Your capstone is complete when:

- [ ] All integration tests pass
- [ ] Server handles 100+ concurrent connections
- [ ] Load tester shows realistic performance metrics
- [ ] Code is properly documented and follows Rust conventions
- [ ] You understand every fix you made and why it was necessary

---

**Remember**: This is a **debugging challenge**, not a design challenge. The architecture is provided - your job is to make it work efficiently!

**Start with Level 1 hints** if you need guidance. Good luck! 🦀