# C# vs Rust Performance Comparisons

A comprehensive guide for C# developers learning Rust performance optimization patterns.

## 🔄 Memory Allocation Patterns

### List/Vector Initialization

| Pattern | C# | Rust |
|---------|-----|------|
| **Default** | `new List<int>()` | `Vec::new()` |
| **With Capacity** | `new List<int>(capacity)` | `Vec::with_capacity(capacity)` |
| **Collection Literal** | `new[] { 1, 2, 3 }` | `vec![1, 2, 3]` |
| **Pre-filled** | `Enumerable.Repeat(0, count).ToList()` | `vec![0; count]` |

### String Building

| Pattern | C# | Rust |
|---------|-----|------|
| **Concatenation (slow)** | `str1 + str2` | `string1 + &string2` |
| **Builder pattern** | `StringBuilder.Append()` | `String::push_str()` |
| **Formatting** | `$"Value: {value}"` | `format!("Value: {}", value)` |
| **With capacity** | `new StringBuilder(capacity)` | `String::with_capacity(capacity)` |

## 🚀 Iterator/LINQ Performance

### Basic Operations

| Operation | C# | Rust |
|-----------|-----|------|
| **Map** | `.Select(x => x * 2)` | `.map(\|x\| x * 2)` |
| **Filter** | `.Where(x => x > 10)` | `.filter(\|&x\| x > 10)` |
| **Reduce** | `.Aggregate((a, b) => a + b)` | `.fold(0, \|a, b\| a + b)` |
| **Take** | `.Take(10)` | `.take(10)` |
| **Sum** | `.Sum()` | `.sum()` |

### Performance Anti-patterns

| Anti-pattern | C# Bad | C# Good | Rust Bad | Rust Good |
|--------------|--------|---------|----------|-----------|
| **Intermediate collections** | `.Where().ToList().Select().ToList()` | `.Where().Select()` | `.filter().collect().map().collect()` | `.filter().map().collect()` |
| **Multiple enumeration** | `var list = query.ToList(); var sum = list.Sum(); var count = list.Count();` | `var results = query.ToList(); // enumerate once` | `let vec: Vec<_> = iter.collect(); let sum = vec.iter().sum();` | `let vec: Vec<_> = iter.collect();` |

## 🧵 Threading and Parallelism

### Basic Threading

| Pattern | C# | Rust |
|---------|-----|------|
| **Create thread** | `new Thread(() => work()).Start()` | `thread::spawn(\|\| work())` |
| **Join thread** | `thread.Join()` | `handle.join().unwrap()` |
| **Parallel loops** | `Parallel.ForEach(items, ProcessItem)` | Items split across threads manually |

### Synchronization

| Pattern | C# | Rust |
|---------|-----|------|
| **Mutex** | `lock(obj) { /* work */ }` | `mutex.lock().unwrap()` |
| **Shared state** | `Concurrent*` collections | `Arc<Mutex<T>>` |
| **Thread-local** | `ThreadLocal<T>` | Thread-local aggregation pattern |

### Performance Guidelines

| Scenario | C# Best Practice | Rust Best Practice |
|----------|------------------|-------------------|
| **Small collections** | Use serial `foreach` | Use serial iterators |
| **CPU-bound work** | `Parallel.ForEach` or `PLINQ` | Manual thread spawning |
| **I/O-bound work** | `async/await` | Thread spawning or async |

## 🏎️ Algorithm Performance

### Search Operations

| Algorithm | C# | Rust | Complexity |
|-----------|-----|------|------------|
| **Linear search** | `.FirstOrDefault(x => x == target)` | `.find(\|&&x\| x == target)` | O(n) |
| **Binary search** | `Array.BinarySearch()` | `.binary_search()` | O(log n) |
| **Hash lookup** | `dictionary[key]` | `map[&key]` | O(1) average |

### Sorting

| Algorithm | C# | Rust | Performance |
|-----------|-----|------|-------------|
| **Default sort** | `Array.Sort()` | `.sort()` | O(n log n) |
| **Custom comparison** | `.OrderBy(keySelector)` | `.sort_by_key(\|x\| x.field)` | O(n log n) |
| **Partial sort** | `.OrderBy().Take(k)` | `.select_nth_unstable()` | O(n) average |

## 📊 Data Structure Performance

### Collection Performance

| Operation | List<T> | Vec<T> | Dictionary<K,V> | HashMap<K,V> |
|-----------|---------|--------|-----------------|--------------|
| **Index access** | O(1) | O(1) | N/A | N/A |
| **Append** | O(1) amortized | O(1) amortized | N/A | N/A |
| **Insert beginning** | O(n) | O(n) | N/A | N/A |
| **Lookup** | O(n) | O(n) | O(1) average | O(1) average |
| **Insert** | O(n) | O(n) | O(1) average | O(1) average |

### Capacity Management

| Scenario | C# Pattern | Rust Pattern |
|----------|------------|--------------|
| **Known size** | `new List<T>(knownSize)` | `Vec::with_capacity(known_size)` |
| **Growing collections** | `list.Capacity = newSize` | `vec.reserve(additional)` |
| **Shrinking** | `list.TrimExcess()` | `vec.shrink_to_fit()` |

## 🎯 Performance Optimization Patterns

### Memory Optimization

| Pattern | C# Example | Rust Example |
|---------|------------|--------------|
| **Object pooling** | `ObjectPool<T>` | Manual buffer reuse |
| **Span/Memory** | `Span<T>`, `Memory<T>` | Slices `&[T]` |
| **ArrayPool** | `ArrayPool<T>.Shared.Rent()` | Manual array reuse |
| **stackalloc** | `stackalloc int[size]` | Arrays on stack `[T; N]` |

### Cache-Friendly Patterns

| Pattern | C# | Rust |
|---------|-----|------|
| **Sequential access** | `foreach(var item in array)` | `for item in array.iter()` |
| **Array of Structures** | `struct Point { X, Y }; Point[]` | `struct Point { x, y }; Vec<Point>` |
| **Structure of Arrays** | `class Points { float[] X, Y; }` | `struct Points { x: Vec<f32>, y: Vec<f32> }` |

## 🔍 Profiling and Measurement

### Timing Code

| Purpose | C# | Rust |
|---------|-----|------|
| **Basic timing** | `Stopwatch.StartNew()` | `Instant::now()` |
| **High precision** | `Stopwatch` | `Instant` (nanosecond precision) |
| **Benchmarking** | `BenchmarkDotNet` | `criterion` crate |

### Memory Profiling

| Tool Type | C# Tools | Rust Tools |
|-----------|----------|------------|
| **Built-in** | Visual Studio Diagnostics | `cargo flamegraph` |
| **External** | JetBrains dotMemory | `heaptrack`, `valgrind` |
| **Allocation tracking** | ETW, PerfView | `dhat`, allocation profiling |

## 🎓 Key Takeaways for C# Developers

### Similarities
- Iterator methods are often auto-vectorized (like LINQ)
- Capacity pre-allocation is important for performance
- Algorithm choice matters more than micro-optimizations
- Profiling is essential for performance work

### Differences
- Rust has zero-cost abstractions (truly no runtime overhead)
- Manual memory management enables better control
- No garbage collector means predictable performance
- Borrowing enables optimizations impossible in C#

### Mental Model Translation
1. **LINQ → Iterators**: Same patterns, often better performance
2. **StringBuilder → String::push_str()**: Similar pattern, similar performance
3. **Parallel.ForEach → Manual threading**: More work, more control
4. **async/await → Threading**: Different concurrency model
5. **Span<T> → &[T]**: Similar zero-copy slicing concepts

The performance optimization mindset transfers directly from C# to Rust - measure first, optimize systematically, and focus on algorithms before micro-optimizations!

## 🏎️ Real-World Performance Comparisons

### Startup Time and Memory Footprint

| Metric | C# (.NET 8) | Rust | Notes |
|--------|-------------|------|-------|
| **Hello World Binary** | ~140 MB | ~200 KB | Rust has no runtime |
| **Startup Time** | 50-200ms | 1-2ms | No JIT compilation |
| **Base Memory Usage** | 20-50 MB | 1-3 MB | No GC overhead |
| **Container Size** | 200+ MB | 10-20 MB | Minimal dependencies |

### Throughput Performance

| Scenario | C# Performance | Rust Performance | Comparison |
|----------|----------------|------------------|------------|
| **JSON Parsing** | 100 MB/s | 300-500 MB/s | 3-5x faster |
| **HTTP Server (RPS)** | 500K req/s | 2M+ req/s | 4x+ throughput |
| **Regex Matching** | Good | Excellent | 2-10x faster |
| **Cryptography** | Hardware accelerated | Hardware accelerated | Similar |

### Memory Allocation Performance

| Operation | C# | Rust | Impact |
|-----------|-----|------|--------|
| **Object Allocation** | ~20ns + GC pressure | Stack: 0ns, Heap: ~50ns | Predictable |
| **Collection Resize** | Amortized O(1) + GC | Amortized O(1) | No GC pauses |
| **String Concatenation** | Creates garbage | Can reuse buffers | Less memory pressure |
| **Temporary Objects** | GC handles cleanup | Stack allocated | Zero overhead |

### Concurrent Performance

| Pattern | C# | Rust | Advantage |
|---------|-----|------|-----------|
| **Thread Creation** | ~1ms | ~50μs | 20x faster |
| **Lock Contention** | Monitor locks | Parking lot mutexes | 2-5x faster |
| **Async Overhead** | State machine allocation | Zero-cost futures | Less memory |
| **Channel Performance** | Concurrent collections | MPSC channels | Similar |

### Common Optimization Wins When Moving from C# to Rust

1. **Zero-Cost Abstractions**
   - C#: LINQ has allocation overhead
   - Rust: Iterators compile to optimal loops

2. **Predictable Performance**
   - C#: GC pauses can cause latency spikes
   - Rust: Consistent sub-millisecond response times

3. **Memory Efficiency**
   - C#: Objects have headers, padding, GC overhead
   - Rust: Exact memory layout control, no overhead

4. **CPU Cache Utilization**
   - C#: Reference types cause pointer chasing
   - Rust: Data locality with value types by default

### When C# Might Be Faster

1. **Development Speed**: Faster prototyping and iteration
2. **Complex Business Logic**: GC simplifies memory management
3. **Dynamic Scenarios**: Reflection and runtime code generation
4. **Existing Ecosystem**: Mature libraries and tooling

### Performance Migration Strategy

When converting performance-critical C# code to Rust:

1. **Profile First**: Identify actual bottlenecks in C#
2. **Port Hot Paths**: Convert only performance-critical sections
3. **Measure Impact**: Benchmark before celebrating
4. **Consider Interop**: Use Rust for performance-critical libraries

Remember: Rust's performance advantages come from:
- No garbage collector (predictable latency)
- Zero-cost abstractions (no runtime overhead)
- Memory safety without overhead (compile-time checks)
- Fine-grained control (when you need it)