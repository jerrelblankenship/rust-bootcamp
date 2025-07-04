# Final Project Hints - Level 1: Getting Started

## 🎯 Overview
You're building a web server, but don't panic! This is actually a debugging exercise that combines everything you've learned. The server is mostly written but has **intentional bugs** that prevent it from working properly.

## 🔍 First Steps - Identifying the Problems

### 1. **Try Running the Server**
```bash
cargo run --bin server
```
What happens? Does it compile? If not, start with the **compilation errors first**.

### 2. **The Major Concurrency Bug**
Look at the main server loop in `src/main.rs`. There's a **critical flaw** that makes this server completely unusable in production. 

**C# Analogy**: Imagine if your ASP.NET Core app processed requests one at a time, waiting for each to complete before handling the next. That's what's happening here!

**Hint**: The problem is in how connections are handled after `listener.accept().await`.

### 3. **Request Parsing Issues**
The `parse_request` function has several bugs:
- What happens with very small requests?
- What about requests larger than 1024 bytes?
- Are headers parsed correctly in all cases?

### 4. **Load Testing Problems**
Try running the load tester:
```bash
cargo run --bin loadtest
```
It probably won't work as expected. Why?

## 🚀 Getting Started Strategy

### Phase 1: Fix Compilation (if needed)
1. Run `cargo check` to see compilation errors
2. Fix them one by one
3. Don't worry about logic bugs yet

### Phase 2: Fix the Concurrency Bug
1. Identify why the server can't handle multiple connections
2. This is the **most critical** fix
3. Think about `tokio::spawn` and what it does

### Phase 3: Test Your Progress
1. Run the server: `cargo run --bin server`
2. In another terminal: `curl http://localhost:8080/`
3. Try multiple concurrent requests: `curl http://localhost:8080/ & curl http://localhost:8080/ &`

## 💡 Key Concepts to Remember

### From Module 5 (Concurrency):
- How do you handle multiple tasks concurrently in async Rust?
- What's the difference between sequential and concurrent execution?

### From Module 3 (Error Handling):
- How should you handle errors in async functions?
- What's the difference between `?` and `.unwrap()`?

### From Module 1 (Foundations):
- How do you handle ownership when sharing data between tasks?
- What's `Arc` and when do you need it?

## 🎯 Success Criteria for Level 1

Before moving to Level 2, you should have:
- [ ] Server compiles without errors
- [ ] Server can handle **multiple concurrent connections**
- [ ] Basic HTTP requests work (`curl http://localhost:8080/`)
- [ ] You understand the main concurrency bug and why it was broken

## 🤔 Stuck? Questions to Ask Yourself

1. "When I run two curl commands at the same time, does the second one have to wait for the first?"
2. "What's the difference between `handle_connection(stream, router).await` and `tokio::spawn(handle_connection(stream, router))`?"
3. "If I have a compilation error, what is the Rust compiler telling me exactly?"

## 📚 Review These Modules If Needed

- **Module 5**: Async programming and `tokio::spawn`
- **Module 2**: `Arc` for sharing data between threads
- **Module 3**: Proper error handling with `?` operator

---

**Time Investment**: Spend at least 30-60 minutes on this level before moving to Level 2. The goal is to understand the fundamental issues, not necessarily fix everything perfectly.

**Next**: Once you have concurrent connections working, move to Level 2 for more advanced debugging!