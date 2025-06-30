# Performance Debugging Checklist 🏎️

When encountering performance issues in Rust, follow this systematic approach:

## 1. Profiling First, Optimizing Second

### ❌ Common Mistake: Guessing Performance Issues
```rust
// Don't assume this is slow without profiling!
for item in large_vec.iter() {
    process(item);
}
```
**Fix:** Use `cargo flamegraph` or `perf` to identify actual bottlenecks

## 2. Memory Allocation Issues

### ❌ Unnecessary Allocations in Loops
```rust
for i in 0..1000000 {
    let mut vec = Vec::new(); // Allocating every iteration!
    vec.push(i);
    process(&vec);
}
```
**Fix:** Pre-allocate or reuse buffers with `Vec::with_capacity()`

### ❌ Cloning When Borrowing Would Work
```rust
fn process_data(data: Vec<u8>) { // Takes ownership, forces clone
    // ...
}
```
**Fix:** Use references: `fn process_data(data: &[u8])`

## 3. Iterator Performance

### ❌ Collecting When Not Needed
```rust
let sum: i32 = vec.iter()
    .map(|x| x * 2)
    .collect::<Vec<_>>() // Unnecessary allocation!
    .iter()
    .sum();
```
**Fix:** Chain operations without collecting

### ❌ Not Using Iterator Adapters
```rust
let mut result = Vec::new();
for item in vec {
    if item > 10 {
        result.push(item * 2);
    }
}
```
**Fix:** Use `filter_map()` or `filter().map()`

## 4. String Performance

### ❌ String Concatenation in Loops
```rust
let mut s = String::new();
for i in 0..1000 {
    s = s + &i.to_string(); // Reallocates every time!
}
```
**Fix:** Use `String::with_capacity()` and `push_str()`

## 5. Bounds Checking

### ❌ Unnecessary Bounds Checks
```rust
for i in 0..vec.len() {
    process(vec[i]); // Bounds check every iteration
}
```
**Fix:** Use iterators or `get_unchecked()` in proven-safe code

## 6. Cache Misses

### ❌ Poor Data Layout
```rust
struct Point {
    x: f64,
    name: String, // 24 bytes in the middle!
    y: f64,
}
```
**Fix:** Group hot data together, cold data separately

## 7. Async Performance

### ❌ Blocking in Async Context
```rust
async fn bad_async() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks executor!
}
```
**Fix:** Use `tokio::time::sleep()` or similar async primitives

## C# to Rust Performance Patterns

| C# Pattern | Rust Equivalent | Performance Note |
|------------|-----------------|------------------|
| `List<T>` | `Vec<T>` | Pre-allocate with capacity |
| `StringBuilder` | `String::with_capacity()` | Avoid reallocation |
| `Parallel.For` | Rayon iterators | Zero-cost parallelism |
| `ArrayPool<T>` | Reuse `Vec<T>` | Manual memory reuse |
| `Span<T>` | `&[T]` slices | Zero-copy views |
| `stackalloc` | Arrays or `SmallVec` | Stack allocation |

## Quick Performance Wins

1. **Enable optimizations**: `cargo build --release`
2. **Use LTO**: Add `lto = true` to Cargo.toml
3. **Profile before optimizing**: Don't guess!
4. **Prefer borrows over clones**: `&T` instead of `T`
5. **Use const generics**: Compile-time sizes
6. **Leverage SIMD**: Use `packed_simd` crate

## Benchmarking Checklist

- [ ] Using `criterion` for micro-benchmarks?
- [ ] Running with `--release` flag?
- [ ] Warming up the cache?
- [ ] Testing with realistic data sizes?
- [ ] Checking assembly output for hot paths?

## Pro Tips

- Start with algorithmic improvements (O(n²) → O(n log n))
- Use `#[inline]` judiciously on small, hot functions
- Consider `Box<[T]>` over `Vec<T>` for fixed-size data
- Profile on target hardware, not just dev machine
- Remember: Rust often optimizes better than you think!