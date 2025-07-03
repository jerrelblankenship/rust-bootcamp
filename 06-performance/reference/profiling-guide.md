# Profiling Guide

How to measure and profile Rust performance effectively.

## 🎯 Profiling Philosophy

> "Measurement is the first step that leads to control and eventually to improvement." - H. James Harrington

### The Golden Rule of Performance

**Never optimize without measuring first.** Performance intuition is notoriously unreliable, and the bottleneck is rarely where you think it is.

### Profiling Workflow

1. **Establish baseline** - Measure current performance
2. **Identify bottlenecks** - Find where time is actually spent
3. **Hypothesize** - What might be causing the bottleneck?
4. **Optimize** - Make targeted improvements
5. **Measure again** - Verify the improvement
6. **Repeat** - Continue until performance is acceptable

## 🔧 Profiling Tools Ecosystem

### Built-in Rust Tools

#### Cargo Bench (Basic Benchmarking)

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_recursive(n-1) + fibonacci_recursive(n-2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for i in [10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", i), i, 
            |b, i| b.iter(|| fibonacci_recursive(black_box(*i))));
        group.bench_with_input(BenchmarkId::new("iterative", i), i,
            |b, i| b.iter(|| fibonacci_iterative(black_box(*i))));
    }
    
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

```bash
# Run benchmarks
cargo bench

# Generate detailed reports
cargo bench -- --output-format html
```

#### Time Profiling with std::time

```rust
use std::time::Instant;

fn time_operation<F, R>(name: &str, operation: F) -> R 
where 
    F: FnOnce() -> R 
{
    let start = Instant::now();
    let result = operation();
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
    result
}

// Usage
fn main() {
    let data = vec![1; 1_000_000];
    
    let sum = time_operation("vector sum", || {
        data.iter().sum::<i32>()
    });
    
    println!("Sum: {}", sum);
}
```

### External Profiling Tools

#### 1. perf (Linux CPU Profiling)

```bash
# Install perf
sudo apt install linux-perf

# Record CPU profile
perf record --call-graph dwarf ./target/release/my_program

# View results interactively
perf report

# Generate text report
perf report --stdio

# Top functions by CPU usage
perf top
```

**Sample perf output:**
```
Samples: 8K of event 'cycles:u', Event count (approx.): 6956750000
Overhead  Command  Shared Object      Symbol                    
  42.50%  my_prog  my_prog            [.] expensive_function
  23.14%  my_prog  libc-2.31.so       [.] __memcpy_avx_unaligned
  12.30%  my_prog  my_prog            [.] data_processing_loop
   8.91%  my_prog  my_prog            [.] memory_allocation
```

#### 2. Flamegraph (Visual Call Stack Profiling)

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph (Linux)
cargo flamegraph --bin my_program

# Generate flamegraph (macOS)
cargo flamegraph --bin my_program -- arg1 arg2

# For specific workload
cargo flamegraph --bin my_program -- --input large_file.txt
```

Flamegraphs show:
- **Width** = Time spent in function
- **Height** = Call stack depth
- **Color** = Different functions

#### 3. Valgrind (Memory Profiling)

```bash
# Install valgrind
sudo apt install valgrind

# Memory error detection
valgrind --tool=memcheck ./target/release/my_program

# Heap profiling
valgrind --tool=massif ./target/release/my_program
ms_print massif.out.12345

# Cache profiling
valgrind --tool=cachegrind ./target/release/my_program
cg_annotate cachegrind.out.12345
```

#### 4. heaptrack (Heap Allocation Profiling)

```bash
# Install heaptrack
sudo apt install heaptrack

# Record heap allocations
heaptrack ./target/release/my_program

# Analyze results
heaptrack_gui heaptrack.my_program.12345.gz
```

### Rust-Specific Tools

#### 1. cargo-profdata (LLVM PGO)

```bash
# Install
cargo install cargo-profdata

# Build with instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# Run with representative workload
./target/release/my_program typical_input.txt

# Merge profile data
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# Build optimized binary
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

#### 2. cargo-asm (Assembly Inspection)

```bash
# Install
cargo install cargo-asm

# View assembly for specific function
cargo asm my_crate::my_function

# View LLVM IR
cargo llvm-ir my_crate::my_function
```

## 📊 Memory Profiling

### Allocation Tracking

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn current_memory_usage() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

// Usage
fn main() {
    println!("Initial: {} bytes", current_memory_usage());
    
    let data = vec![0; 1_000_000];
    println!("After allocation: {} bytes", current_memory_usage());
    
    drop(data);
    println!("After drop: {} bytes", current_memory_usage());
}
```

### Memory Layout Analysis

```rust
use std::mem;

fn analyze_memory_layout<T>() {
    println!("Type: {}", std::any::type_name::<T>());
    println!("Size: {} bytes", mem::size_of::<T>());
    println!("Alignment: {} bytes", mem::align_of::<T>());
    println!("---");
}

#[repr(C)]
struct BadLayout {
    a: u8,    // 1 byte
    b: u64,   // 8 bytes (7 bytes padding after a)
    c: u8,    // 1 byte (7 bytes padding after c)
}

#[repr(C)]
struct GoodLayout {
    b: u64,   // 8 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte (6 bytes padding at end)
}

fn main() {
    analyze_memory_layout::<BadLayout>();   // 24 bytes
    analyze_memory_layout::<GoodLayout>();  // 16 bytes
}
```

## 🏎️ CPU Profiling Techniques

### Instruction-Level Profiling

```bash
# Count instructions with perf
perf stat -e instructions,cycles,cache-misses ./my_program

# Output:
#  1,234,567,890      instructions              #    1.23  insn per cycle
#    987,654,321      cycles                    
#        123,456      cache-misses              #    0.01% of all cache refs
```

### Branch Prediction Analysis

```bash
# Analyze branch prediction
perf stat -e branches,branch-misses ./my_program

# Output:
#    100,000,000      branches                  
#      1,000,000      branch-misses             #    1.00% of all branches
```

### Cache Performance

```bash
# Cache miss analysis
perf stat -e cache-references,cache-misses,L1-dcache-loads,L1-dcache-load-misses ./my_program
```

## 🔬 Micro-benchmarking Best Practices

### Avoiding Common Pitfalls

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// ❌ Bad: Compiler can optimize away the work
fn bad_benchmark(c: &mut Criterion) {
    c.bench_function("bad", |b| {
        b.iter(|| {
            let result = expensive_computation(42);
            // Result is never used - compiler might optimize away!
        })
    });
}

// ✅ Good: Use black_box to prevent optimization
fn good_benchmark(c: &mut Criterion) {
    c.bench_function("good", |b| {
        b.iter(|| {
            let result = expensive_computation(black_box(42));
            black_box(result); // Prevent optimization
        })
    });
}

// ✅ Better: Benchmark with realistic data
fn realistic_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic");
    
    // Test multiple input sizes
    for size in [100, 1_000, 10_000, 100_000].iter() {
        let data = generate_test_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("my_function", size), 
            &data,
            |b, data| b.iter(|| my_function(black_box(data)))
        );
    }
    
    group.finish();
}
```

### Statistical Rigor

```rust
use criterion::{Criterion, SamplingMode, MeasurementTime};

fn rigorous_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rigorous");
    
    // Increase sample size for more reliable results
    group.sample_size(1000);
    
    // Increase measurement time for stable results
    group.measurement_time(std::time::Duration::from_secs(10));
    
    // Use linear sampling for consistent timing
    group.sampling_mode(SamplingMode::Linear);
    
    group.bench_function("my_function", |b| {
        b.iter(|| expensive_function(black_box(42)))
    });
    
    group.finish();
}
```

## 📈 Performance Regression Detection

### Automated Performance Testing

```rust
// tests/performance_tests.rs
use std::time::{Duration, Instant};

#[test]
fn performance_regression_test() {
    let data = generate_large_dataset();
    
    let start = Instant::now();
    let result = my_algorithm(&data);
    let duration = start.elapsed();
    
    // Fail if performance regresses by more than 20%
    let max_duration = Duration::from_millis(100);
    assert!(duration < max_duration, 
        "Performance regression detected: {:?} > {:?}", 
        duration, max_duration);
    
    // Verify correctness
    assert_eq!(result, expected_result());
}
```

### Continuous Benchmarking

```bash
# .github/workflows/benchmark.yml
name: Benchmark
on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run benchmarks
        run: cargo bench -- --output-format json | tee bench_result.json
      - name: Store benchmark result
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: bench_result.json
```

## 🎛️ Production Profiling

### Low-Overhead Profiling

```rust
// Sampling profiler that doesn't impact performance significantly
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static SAMPLE_COUNTER: AtomicU64 = AtomicU64::new(0);

macro_rules! sample_profile {
    ($name:expr, $code:block) => {{
        let count = SAMPLE_COUNTER.fetch_add(1, Ordering::Relaxed);
        if count % 10000 == 0 {  // Sample 0.01% of executions
            let start = Instant::now();
            let result = $code;
            let duration = start.elapsed();
            eprintln!("PROFILE {}: {:?}", $name, duration);
            result
        } else {
            $code
        }
    }};
}

// Usage
fn hot_function(data: &[i32]) -> i32 {
    sample_profile!("hot_function", {
        data.iter().sum()
    })
}
```

### Feature-Gated Profiling

```rust
// Cargo.toml
[features]
profiling = []

// src/lib.rs
#[cfg(feature = "profiling")]
macro_rules! profile {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        eprintln!("PROFILE {}: {:?}", $name, duration);
        result
    }};
}

#[cfg(not(feature = "profiling"))]
macro_rules! profile {
    ($name:expr, $code:block) => {
        $code
    };
}

// Usage
fn my_function(data: &[i32]) -> i32 {
    profile!("my_function", {
        expensive_computation(data)
    })
}
```

```bash
# Build with profiling enabled
cargo build --features profiling
```

## 📋 Profiling Checklist

### Before Profiling
- [ ] Build in release mode (`cargo build --release`)
- [ ] Use realistic test data and workloads
- [ ] Ensure system is in consistent state (no other heavy processes)
- [ ] Warm up the system if measuring steady-state performance

### During Profiling
- [ ] Profile the actual bottleneck, not synthetic tests
- [ ] Use multiple tools to get different perspectives
- [ ] Profile both CPU and memory usage
- [ ] Consider cache behavior and memory access patterns
- [ ] Look at both hot paths and outliers

### After Profiling
- [ ] Verify optimizations with before/after measurements
- [ ] Test with different input sizes and characteristics
- [ ] Ensure optimizations don't break correctness
- [ ] Document the optimization rationale

## 🎯 Platform-Specific Considerations

### Linux
- Use `perf` for detailed CPU profiling
- `valgrind` for memory analysis
- Consider CPU governor settings for consistent results

### macOS
- Use `Instruments` from Xcode for comprehensive profiling
- `cargo flamegraph` works well for call stack visualization
- Consider thermal throttling on laptops

### Windows
- Use `Windows Performance Toolkit` (WPA/WPT)
- `Visual Studio Diagnostic Tools`
- Consider Windows Defender impact on benchmarks

## 🎓 Key Takeaways

1. **Profile first, optimize second** - Don't guess where the bottlenecks are
2. **Use the right tool for the job** - CPU profiling vs memory profiling vs micro-benchmarks
3. **Benchmark realistically** - Use real data and workloads
4. **Statistical rigor matters** - Multiple runs, confidence intervals
5. **Profile in production** - Development environment may not reflect real performance
6. **Automate regression detection** - Catch performance regressions early

Remember: The goal of profiling is insight, not numbers. Use profiling data to understand your program's behavior and guide optimization decisions!