# Performance Testing in Rust 🏎️

*A comprehensive guide to benchmarking, profiling, and performance testing with cargo bench, criterion, and advanced tools*

## 🚀 Quick Reference

### Essential Benchmarking Setup

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

### Quick Commands

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench my_benchmark

# Generate HTML reports
cargo bench -- --output-format html

# Compare with baseline
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

---

## Introduction to Performance Testing

### Performance Testing vs Unit Testing

| Aspect | Unit Testing | Performance Testing |
|--------|-------------|-------------------|
| **Purpose** | Correctness | Speed, memory, scalability |
| **Metrics** | Pass/Fail | Time, throughput, memory usage |
| **Stability** | Deterministic | Statistical, environment-dependent |
| **Duration** | Milliseconds | Seconds to minutes |
| **Frequency** | Every commit | Periodic, on performance-critical changes |

### Types of Performance Tests

1. **Microbenchmarks** - Individual function performance
2. **Integration benchmarks** - Component interaction performance  
3. **Load tests** - System behavior under load
4. **Stress tests** - System limits and failure points
5. **Memory profiling** - Memory usage and leak detection
6. **CPU profiling** - Hotspot identification

---

## Criterion.rs Benchmarking

### 1. Basic Setup and Configuration

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
plotters = "0.3"  # For chart generation
rayon = "1.0"     # For parallel benchmarks

[[bench]]
name = "core_functions"
harness = false

[[bench]]
name = "data_structures"
harness = false
```

### 2. Basic Benchmarking Patterns

```rust
// benches/core_functions.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Simple function benchmark
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
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

// Basic benchmark
fn bench_fibonacci_recursive(c: &mut Criterion) {
    c.bench_function("fibonacci_recursive 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
}

// Parameterized benchmark
fn bench_fibonacci_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_comparison");
    
    for i in [10, 15, 20, 25].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", i), i, |b, i| {
            b.iter(|| fibonacci_recursive(black_box(*i)))
        });
        
        group.bench_with_input(BenchmarkId::new("iterative", i), i, |b, i| {
            b.iter(|| fibonacci_iterative(black_box(*i)))
        });
    }
    
    group.finish();
}

// Throughput benchmark
fn bench_data_processing(c: &mut Criterion) {
    let data: Vec<u32> = (0..10000).collect();
    
    let mut group = c.benchmark_group("data_processing");
    group.throughput(criterion::Throughput::Elements(data.len() as u64));
    
    group.bench_function("sum", |b| {
        b.iter(|| {
            data.iter().sum::<u32>()
        })
    });
    
    group.bench_function("parallel_sum", |b| {
        b.iter(|| {
            use rayon::prelude::*;
            data.par_iter().sum::<u32>()
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_fibonacci_recursive,
    bench_fibonacci_comparison,
    bench_data_processing
);
criterion_main!(benches);
```

### 3. Advanced Criterion Features

```rust
// benches/advanced_benchmarks.rs
use criterion::{
    criterion_group, criterion_main, 
    BenchmarkId, Criterion, PlotConfiguration, AxisScale
};
use std::time::Duration;

// Custom configuration
fn create_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .sample_size(100)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic))
}

// Memory allocation benchmark
fn bench_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocations");
    
    // Vec pre-allocation vs dynamic growth
    group.bench_function("vec_dynamic", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(black_box(i));
            }
            vec
        })
    });
    
    group.bench_function("vec_pre_allocated", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(black_box(i));
            }
            vec
        })
    });
    
    group.finish();
}

// Complex data structure benchmarks
use std::collections::{HashMap, BTreeMap};

fn bench_map_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_operations");
    
    // Setup data
    let keys: Vec<String> = (0..1000).map(|i| format!("key_{}", i)).collect();
    let values: Vec<i32> = (0..1000).collect();
    
    // HashMap insertion
    group.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for (k, v) in keys.iter().zip(values.iter()) {
                map.insert(k.clone(), *v);
            }
            map
        })
    });
    
    // BTreeMap insertion
    group.bench_function("btreemap_insert", |b| {
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (k, v) in keys.iter().zip(values.iter()) {
                map.insert(k.clone(), *v);
            }
            map
        })
    });
    
    // Setup maps for lookup benchmarks
    let hashmap: HashMap<String, i32> = keys.iter().zip(values.iter())
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    let btreemap: BTreeMap<String, i32> = keys.iter().zip(values.iter())
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    
    // Lookup benchmarks
    group.bench_function("hashmap_lookup", |b| {
        b.iter(|| {
            let key = black_box(&keys[500]);
            hashmap.get(key)
        })
    });
    
    group.bench_function("btreemap_lookup", |b| {
        b.iter(|| {
            let key = black_box(&keys[500]);
            btreemap.get(key)
        })
    });
    
    group.finish();
}

criterion_group!(
    name = benches;
    config = create_criterion();
    targets = bench_allocations, bench_map_operations
);
criterion_main!(benches);
```

### 4. Async Benchmarking

```rust
// benches/async_benchmarks.rs
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

async fn async_computation(n: u64) -> u64 {
    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_nanos(n * 100)).await;
    n * 2
}

fn bench_async_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("async_computation", |b| {
        b.to_async(&rt).iter(|| async {
            async_computation(black_box(100)).await
        })
    });
}

// Concurrent async benchmarks
async fn concurrent_work(tasks: usize) -> Vec<u64> {
    let handles: Vec<_> = (0..tasks)
        .map(|i| tokio::spawn(async_computation(i as u64)))
        .collect();
    
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}

fn bench_concurrency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("concurrency");
    
    for task_count in [1, 10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_tasks", task_count),
            task_count,
            |b, &task_count| {
                b.to_async(&rt).iter(|| async move {
                    concurrent_work(task_count).await
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_async_operations, bench_concurrency);
criterion_main!(benches);
```

---

## Built-in Rust Benchmarking

### 1. Unstable Bench Feature

```rust
// src/lib.rs
#![feature(test)]  // Requires nightly Rust

extern crate test;

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_fibonacci_10(b: &mut Bencher) {
        b.iter(|| fibonacci(test::black_box(10)));
    }
    
    #[bench]
    fn bench_fibonacci_20(b: &mut Bencher) {
        b.iter(|| fibonacci(test::black_box(20)));
    }
}
```

```bash
# Run with nightly Rust
rustup run nightly cargo bench
```

### 2. Cargo Bench Integration

```toml
# Cargo.toml for built-in benches
[profile.bench]
debug = true  # Include debug info for profiling
```

---

## Memory Profiling and Analysis

### 1. Memory Usage Benchmarking

```rust
// Memory-focused benchmarks
use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Custom allocator for tracking
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

fn get_allocated_bytes() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

fn bench_memory_usage(c: &mut Criterion) {
    c.bench_function("vector_growth", |b| {
        b.iter_custom(|iters| {
            let start_memory = get_allocated_bytes();
            let start_time = std::time::Instant::now();
            
            for _ in 0..iters {
                let mut vec = Vec::new();
                for i in 0..1000 {
                    vec.push(black_box(i));
                }
                // Force deallocation
                drop(vec);
            }
            
            let end_time = start_time.elapsed();
            let end_memory = get_allocated_bytes();
            
            println!("Memory delta: {} bytes", end_memory - start_memory);
            end_time
        })
    });
}

criterion_group!(benches, bench_memory_usage);
criterion_main!(benches);
```

### 2. Heap Profiling with jemalloc

```toml
# Cargo.toml
[dependencies]
jemallocator = "0.5"

[profile.bench]
debug = true
```

```rust
// src/main.rs
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// Enable heap profiling
#[cfg(not(target_env = "msvc"))]
fn enable_heap_profiling() {
    use std::env;
    env::set_var("MALLOC_CONF", "prof:true,prof_active:true,prof_prefix:jeprof");
}
```

---

## CPU Profiling and Flamegraphs

### 1. Perf Integration

```bash
# Install perf (Linux)
sudo apt install linux-tools-common linux-tools-generic

# Record performance data
sudo perf record --call-graph=dwarf cargo bench

# Generate flamegraph
git clone https://github.com/flamegraph-rs/flamegraph
cargo install flamegraph
cargo flamegraph --bench my_benchmark
```

### 2. Built-in Profiling Support

```rust
// benches/profiling_benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion, Profiler};
use std::fs::File;
use std::os::unix::io::AsRawFd;

// Custom profiler integration
pub struct CpuProfiler;

impl Profiler for CpuProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &std::path::Path) {
        // Start profiling (implementation depends on profiler)
    }

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &std::path::Path) {
        // Stop profiling and save results
    }
}

fn profile_heavy_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_intensive");
    
    // Enable profiling
    #[cfg(unix)]
    {
        group.with_profiler(CpuProfiler);
    }
    
    group.bench_function("matrix_multiplication", |b| {
        let matrix_a = vec![vec![1.0; 100]; 100];
        let matrix_b = vec![vec![2.0; 100]; 100];
        
        b.iter(|| {
            matrix_multiply(&matrix_a, &matrix_b)
        })
    });
    
    group.finish();
}

fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut result = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    result
}

criterion_group!(benches, profile_heavy_computation);
criterion_main!(benches);
```

---

## Advanced Performance Testing Patterns

### 1. Statistical Analysis

```rust
// benches/statistical_analysis.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;

fn bench_sorting_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting_comparison");
    
    // Test different data sizes
    for size in [100, 1000, 10000].iter() {
        // Random data
        let mut rng = thread_rng();
        let data: Vec<i32> = (0..*size).map(|_| rng.gen()).collect();
        
        group.bench_with_input(
            BenchmarkId::new("quicksort", size),
            &data,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut data| quicksort(&mut data),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("stdlib_sort", size),
            &data,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut data| data.sort(),
                    criterion::BatchSize::SmallInput,
                )
            },
        );
    }
    
    group.finish();
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, arr.len() - 1);
    i
}

criterion_group!(benches, bench_sorting_algorithms);
criterion_main!(benches);
```

### 2. Regression Testing

```rust
// benches/regression_testing.rs
use criterion::{criterion_group, criterion_main, Criterion};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct BenchmarkResult {
    name: String,
    mean_ns: f64,
    std_dev_ns: f64,
}

fn save_baseline_results(results: &[BenchmarkResult]) {
    let json = serde_json::to_string_pretty(results).unwrap();
    fs::write("benchmark_baseline.json", json).unwrap();
}

fn load_baseline_results() -> Vec<BenchmarkResult> {
    if let Ok(content) = fs::read_to_string("benchmark_baseline.json") {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn check_performance_regression(
    current: &BenchmarkResult,
    baseline: &BenchmarkResult,
    threshold: f64,
) -> bool {
    let regression_ratio = current.mean_ns / baseline.mean_ns;
    regression_ratio > (1.0 + threshold)
}

fn bench_with_regression_check(c: &mut Criterion) {
    let baseline = load_baseline_results();
    
    c.bench_function("critical_function", |b| {
        b.iter(|| {
            // Your critical function here
            expensive_operation()
        })
    });
    
    // After benchmarking, check for regressions
    // This would typically be done in a separate step
}

fn expensive_operation() -> u64 {
    (0..10000).map(|x| x * x).sum()
}

criterion_group!(benches, bench_with_regression_check);
criterion_main!(benches);
```

### 3. Load Testing Patterns

```rust
// benches/load_testing.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

// Simulated service under test
struct Service {
    processing_time: Duration,
}

impl Service {
    fn new(processing_time: Duration) -> Self {
        Self { processing_time }
    }
    
    fn process_request(&self, _request: &str) -> String {
        thread::sleep(self.processing_time);
        "response".to_string()
    }
}

fn bench_concurrent_load(c: &mut Criterion) {
    let service = Arc::new(Service::new(Duration::from_millis(10)));
    let mut group = c.benchmark_group("concurrent_load");
    
    for thread_count in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_requests", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter_custom(|iters| {
                    let service = service.clone();
                    let barrier = Arc::new(Barrier::new(thread_count + 1));
                    let start_time = Arc::new(std::sync::Mutex::new(None));
                    let end_time = Arc::new(std::sync::Mutex::new(None));
                    
                    let handles: Vec<_> = (0..thread_count)
                        .map(|_| {
                            let service = service.clone();
                            let barrier = barrier.clone();
                            let start_time = start_time.clone();
                            let end_time = end_time.clone();
                            
                            thread::spawn(move || {
                                barrier.wait();
                                
                                // Record start time (first thread)
                                {
                                    let mut start = start_time.lock().unwrap();
                                    if start.is_none() {
                                        *start = Some(Instant::now());
                                    }
                                }
                                
                                // Perform work
                                for _ in 0..iters {
                                    service.process_request("test request");
                                }
                                
                                // Record end time (last thread)
                                *end_time.lock().unwrap() = Some(Instant::now());
                            })
                        })
                        .collect();
                    
                    // Start all threads
                    barrier.wait();
                    
                    // Wait for completion
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    let start = start_time.lock().unwrap().unwrap();
                    let end = end_time.lock().unwrap().unwrap();
                    end.duration_since(start)
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_concurrent_load);
criterion_main!(benches);
```

---

## Integration with CI/CD

### 1. Automated Performance Testing

```yaml
# .github/workflows/performance.yml
name: Performance Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
        
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: bench-${{ runner.os }}-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run benchmarks
      run: cargo bench -- --output-format json | tee benchmark_results.json
      
    - name: Store benchmark result
      uses: benchmark-action/github-action-benchmark@v1
      with:
        tool: 'cargo'
        output-file-path: benchmark_results.json
        github-token: ${{ secrets.GITHUB_TOKEN }}
        auto-push: true
        alert-threshold: '200%'
        comment-on-alert: true
        fail-on-alert: true
```

### 2. Performance Regression Detection

```bash
#!/bin/bash
# scripts/check_performance.sh

# Run benchmarks and save baseline
cargo bench -- --save-baseline main

# Switch to feature branch
git checkout feature-branch

# Run benchmarks and compare
cargo bench -- --baseline main

# Check if any benchmark regressed by more than 10%
if cargo bench -- --baseline main 2>&1 | grep -q "regressed"; then
    echo "Performance regression detected!"
    exit 1
fi
```

---

## Memory and Resource Monitoring

### 1. Real-time Memory Monitoring

```rust
// Memory monitoring during benchmarks
use criterion::{criterion_group, criterion_main, Criterion};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct MemoryProfiler {
    peak_memory: AtomicUsize,
    current_memory: AtomicUsize,
}

impl MemoryProfiler {
    fn new() -> Self {
        Self {
            peak_memory: AtomicUsize::new(0),
            current_memory: AtomicUsize::new(0),
        }
    }
    
    fn allocate(&self, size: usize) {
        let current = self.current_memory.fetch_add(size, Ordering::SeqCst) + size;
        
        // Update peak if necessary
        let mut peak = self.peak_memory.load(Ordering::SeqCst);
        while peak < current {
            match self.peak_memory.compare_exchange_weak(
                peak,
                current,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => peak = x,
            }
        }
    }
    
    fn deallocate(&self, size: usize) {
        self.current_memory.fetch_sub(size, Ordering::SeqCst);
    }
    
    fn peak_memory(&self) -> usize {
        self.peak_memory.load(Ordering::SeqCst)
    }
    
    fn current_memory(&self) -> usize {
        self.current_memory.load(Ordering::SeqCst)
    }
    
    fn reset(&self) {
        self.peak_memory.store(0, Ordering::SeqCst);
        self.current_memory.store(0, Ordering::SeqCst);
    }
}

static PROFILER: MemoryProfiler = MemoryProfiler {
    peak_memory: AtomicUsize::new(0),
    current_memory: AtomicUsize::new(0),
};

struct ProfiledAllocator;

unsafe impl GlobalAlloc for ProfiledAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            PROFILER.allocate(layout.size());
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        PROFILER.deallocate(layout.size());
    }
}

#[global_allocator]
static GLOBAL: ProfiledAllocator = ProfiledAllocator;

fn bench_memory_intensive_operation(c: &mut Criterion) {
    c.bench_function("memory_intensive", |b| {
        b.iter_custom(|iters| {
            PROFILER.reset();
            let start = std::time::Instant::now();
            
            for _ in 0..iters {
                let data: Vec<u64> = (0..10000).collect();
                criterion::black_box(data);
            }
            
            let duration = start.elapsed();
            let peak_memory = PROFILER.peak_memory();
            
            println!("Peak memory usage: {} bytes", peak_memory);
            duration
        })
    });
}

criterion_group!(benches, bench_memory_intensive_operation);
criterion_main!(benches);
```

---

## Performance Testing Best Practices

### 1. Benchmark Design Principles

```rust
// Good benchmark practices
use criterion::{criterion_group, criterion_main, Criterion, black_box};

// ✅ Good: Isolate the operation being tested
fn bench_string_creation_good(c: &mut Criterion) {
    c.bench_function("string_from_parts", |b| {
        let prefix = "Hello";
        let suffix = "World";
        
        b.iter(|| {
            format!("{} {}", black_box(prefix), black_box(suffix))
        })
    });
}

// ❌ Bad: Including setup in the benchmark
fn bench_string_creation_bad(c: &mut Criterion) {
    c.bench_function("string_from_parts_bad", |b| {
        b.iter(|| {
            let prefix = "Hello";  // This setup is included in timing
            let suffix = "World";
            format!("{} {}", prefix, suffix)
        })
    });
}

// ✅ Good: Use iter_batched for expensive setup
fn bench_with_expensive_setup(c: &mut Criterion) {
    c.bench_function("process_large_data", |b| {
        b.iter_batched(
            || {
                // Expensive setup not included in timing
                (0..100000).collect::<Vec<_>>()
            },
            |data| {
                // Only this is timed
                process_data(black_box(data))
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

// ✅ Good: Prevent compiler optimizations with black_box
fn bench_computation(c: &mut Criterion) {
    c.bench_function("complex_calculation", |b| {
        b.iter(|| {
            let result = expensive_computation(black_box(42));
            black_box(result)  // Prevent optimization of the result
        })
    });
}

fn process_data(data: Vec<i32>) -> i32 {
    data.iter().sum()
}

fn expensive_computation(n: i32) -> i32 {
    (0..n).map(|x| x * x).sum()
}

criterion_group!(
    benches,
    bench_string_creation_good,
    bench_with_expensive_setup,
    bench_computation
);
criterion_main!(benches);
```

### 2. Environment Considerations

```rust
// Environment-aware benchmarking
fn bench_environment_aware(c: &mut Criterion) {
    // Check if running in CI
    let in_ci = std::env::var("CI").is_ok();
    
    let mut group = c.benchmark_group("environment_aware");
    
    if in_ci {
        // Shorter, less accurate benchmarks for CI
        group.sample_size(10);
        group.measurement_time(std::time::Duration::from_secs(1));
    } else {
        // More thorough benchmarks for local development
        group.sample_size(100);
        group.measurement_time(std::time::Duration::from_secs(5));
    }
    
    group.bench_function("computation", |b| {
        b.iter(|| expensive_computation(black_box(100)))
    });
    
    group.finish();
}
```

### 3. Cross-Platform Considerations

```rust
// Platform-specific benchmarks
#[cfg(target_os = "linux")]
fn bench_linux_specific(c: &mut Criterion) {
    c.bench_function("linux_specific_operation", |b| {
        b.iter(|| {
            // Linux-specific code
        })
    });
}

#[cfg(target_os = "windows")]
fn bench_windows_specific(c: &mut Criterion) {
    c.bench_function("windows_specific_operation", |b| {
        b.iter(|| {
            // Windows-specific code
        })
    });
}

// Conditional compilation for criterion groups
#[cfg(target_os = "linux")]
criterion_group!(os_specific_benches, bench_linux_specific);

#[cfg(target_os = "windows")]
criterion_group!(os_specific_benches, bench_windows_specific);

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
criterion_group!(os_specific_benches,);
```

---

## Troubleshooting Performance Issues

### 1. Common Benchmark Problems

```rust
// Debugging benchmark issues
use criterion::{criterion_group, criterion_main, Criterion, black_box};

// Problem: Unstable results
fn debug_unstable_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("debugging");
    
    // Solution: Increase sample size and warm-up time
    group.sample_size(200);
    group.warm_up_time(std::time::Duration::from_secs(3));
    group.measurement_time(std::time::Duration::from_secs(10));
    
    group.bench_function("unstable_operation", |b| {
        b.iter(|| {
            // Operation with high variance
            thread_local! {
                static RNG: std::cell::RefCell<rand::rngs::ThreadRng> = 
                    std::cell::RefCell::new(rand::thread_rng());
            }
            
            RNG.with(|rng| {
                use rand::Rng;
                let mut rng = rng.borrow_mut();
                let work_amount = rng.gen_range(1000..2000);
                (0..work_amount).sum::<usize>()
            })
        })
    });
    
    group.finish();
}

// Problem: Compiler optimizing away the code
fn debug_optimization_issues(c: &mut Criterion) {
    c.bench_function("optimization_issue", |b| {
        b.iter(|| {
            // ❌ Bad: Compiler might optimize this away
            let result = 2 + 2;
            
            // ✅ Good: Prevent optimization
            black_box(result)
        })
    });
}

criterion_group!(benches, debug_unstable_benchmark, debug_optimization_issues);
criterion_main!(benches);
```

### 2. Performance Debugging Tools

```bash
# Profiling with perf (Linux)
cargo bench --bench my_benchmark -- --profile-time=5

# Memory profiling with valgrind
cargo bench --bench my_benchmark
valgrind --tool=massif target/release/deps/my_benchmark-*

# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph --bench my_benchmark

# Memory debugging with AddressSanitizer
RUSTFLAGS="-Z sanitizer=address" cargo bench --target x86_64-unknown-linux-gnu
```

---

## Conclusion

Effective performance testing in Rust requires:

1. **Proper Benchmarking Setup** - Using criterion.rs for statistical rigor
2. **Comprehensive Metrics** - Time, memory, throughput, and scalability
3. **Environment Awareness** - Accounting for CI/CD and platform differences
4. **Regression Detection** - Automated performance monitoring
5. **Profiling Integration** - CPU and memory profiling for optimization

Key principles:
- **Isolate what you're measuring** with proper setup and teardown
- **Prevent compiler optimizations** with black_box
- **Use statistical analysis** for reliable results
- **Monitor continuously** in CI/CD pipelines
- **Profile deeply** when benchmarks reveal issues

With proper performance testing, you can ensure your Rust applications maintain excellent performance characteristics throughout their development lifecycle.