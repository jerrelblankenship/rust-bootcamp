# Concurrency Debugging Checklist 🔍

When encountering concurrency issues in Rust, work through this systematic checklist:

## 1. Thread Safety Compilation Errors

### ❌ "cannot be sent between threads safely"
```rust
// Error: `Rc<T>` cannot be sent between threads
let data = Rc::new(vec![1, 2, 3]);
thread::spawn(move || {
    println!("{:?}", data); // ERROR!
});
```
**Fix:** Use `Arc<T>` instead of `Rc<T>` for thread-safe reference counting

### ❌ "cannot be shared between threads safely"
```rust
// Error: `RefCell<T>` is not Sync
let cell = Arc::new(RefCell::new(0));
thread::spawn(move || {
    *cell.borrow_mut() += 1; // ERROR!
});
```
**Fix:** Use `Mutex<T>` or `RwLock<T>` for thread-safe interior mutability

## 2. Ownership and Move Semantics

### ❌ "value moved here"
```rust
let data = vec![1, 2, 3];
thread::spawn(|| {
    println!("{:?}", data); // ERROR: data moved
});
println!("{:?}", data); // Can't use data here
```
**Fix:** Clone data or use Arc for shared ownership

## 3. Lifetime Issues with Threads

### ❌ "borrowed value does not live long enough"
```rust
let data = vec![1, 2, 3];
thread::spawn(|| {
    println!("{:?}", &data); // ERROR: reference to local
});
```
**Fix:** Move ownership into thread or use scoped threads

## 4. Deadlock Prevention

### Common Deadlock Pattern
```rust
// Thread 1: locks A then B
// Thread 2: locks B then A
// DEADLOCK!
```
**Fix:** Always acquire locks in the same order across all threads

## 5. Data Race Prevention

### Pattern to Avoid
```rust
// Multiple threads modifying shared state without synchronization
static mut COUNTER: i32 = 0;
// NEVER do this without proper synchronization!
```
**Fix:** Use atomic types or Mutex for shared mutable state

## 6. Async/Await Common Issues

### ❌ "cannot be unpinned"
```rust
// Self-referential futures need pinning
let fut = async {
    let data = vec![1, 2, 3];
    let ref_to_data = &data;
    // ...
};
```
**Fix:** Use `Box::pin()` or `pin_utils::pin_mut!`

### ❌ "future cannot be sent between threads"
```rust
// Using non-Send types in async blocks
let rc = Rc::new(42);
tokio::spawn(async move {
    println!("{}", rc); // ERROR!
});
```
**Fix:** Use Send types like Arc instead

## C# to Rust Concurrency Mapping

| C# Concept | Rust Equivalent |
|------------|-----------------|
| `Task<T>` | `Future<Output = T>` |
| `async/await` | `async/await` |
| `lock(obj)` | `mutex.lock()` |
| `Thread` | `std::thread::Thread` |
| `Interlocked` | `std::sync::atomic` |
| `ConcurrentDictionary` | `DashMap` (external crate) |
| `SemaphoreSlim` | `tokio::sync::Semaphore` |
| `CancellationToken` | Drop the future |

## Quick Diagnostic Steps

1. **Check Send/Sync bounds**: Does your type implement Send/Sync?
2. **Verify ownership**: Who owns the data being shared?
3. **Lock ordering**: Are locks always acquired in the same order?
4. **Async runtime**: Are you mixing runtimes (tokio vs async-std)?
5. **Blocking in async**: Are you calling blocking code in async contexts?

## Pro Tips

- Start with message passing (`mpsc`) before shared state
- Use `Arc<Mutex<T>>` as your default for shared mutable state
- Prefer `tokio::sync` primitives in async code
- Use `#[tokio::test]` for async test functions
- Enable `loom` for advanced concurrency testing