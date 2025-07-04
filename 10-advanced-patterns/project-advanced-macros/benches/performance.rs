// ❌ BROKEN: Missing criterion import and setup
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advanced_macros::*;

// ❌ BROKEN: Benchmark function with incorrect signature
fn benchmark_fast_buffer(c: &mut Criterion) {
    c.bench_function("fast_buffer_operations", |b| {
        b.iter(|| {
            let mut buffer = FastBuffer::new(1000);
            
            // ❌ BROKEN: Benchmarking potentially unsafe operations
            for i in 0..1000 {
                buffer.push_safe(i).unwrap();
            }
            
            let mut sum = 0;
            // ❌ BROKEN: Iterator might yield invalid references
            for item in buffer.iter() {
                sum += *item;
            }
            
            // ❌ BROKEN: Using black_box without importing it
            black_box(sum)
        })
    });
}

// ❌ BROKEN: Comparison benchmark with broken safe version
fn benchmark_safe_vs_unsafe(c: &mut Criterion) {
    let mut group = c.benchmark_group("safe_vs_unsafe");
    
    // Safe version using Vec
    group.bench_function("safe_vec", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i);
            }
            
            let sum: i32 = vec.iter().sum();
            black_box(sum)
        })
    });
    
    // ❌ BROKEN: Unsafe version with potential UB
    group.bench_function("unsafe_buffer", |b| {
        b.iter(|| {
            let mut buffer = FastBuffer::new(1000);
            
            // ❌ BROKEN: This might cause undefined behavior
            for i in 0..1000 {
                unsafe {
                    buffer.push_unchecked(i);
                }
            }
            
            let mut sum = 0;
            for i in 0..1000 {
                unsafe {
                    sum += *buffer.get_unchecked(i);
                }
            }
            
            black_box(sum)
        })
    });
    
    group.finish();
}

// ❌ BROKEN: Async benchmark with Pin issues
fn benchmark_async_components(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("async_component_processing", |b| {
        b.to_async(&rt).iter(|| async {
            // ❌ BROKEN: Creating component with broken Pin implementation
            let component = AsyncComponent::new("benchmark".to_string());
            let result = process_async_component(component).await;
            black_box(result)
        })
    });
}

// ❌ BROKEN: Macro expansion benchmark
fn benchmark_macro_performance(c: &mut Criterion) {
    c.bench_function("macro_generated_code", |b| {
        b.iter(|| {
            // ❌ BROKEN: Using broken macro
            let component = generate_component!(TestComponent {
                data: String,
                count: usize,
            });
            
            let test_comp = TestComponent {
                data: "benchmark".to_string(),
                count: 42,
            };
            
            // ❌ BROKEN: Debug formatting might be broken
            let debug_str = format!("{:?}", test_comp);
            black_box(debug_str)
        })
    });
}

// ❌ BROKEN: String operations benchmark with unsafe code
fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    
    group.bench_function("safe_string", |b| {
        b.iter(|| {
            let mut s = String::new();
            for ch in "Hello, World!".chars() {
                s.push(ch);
            }
            black_box(s)
        })
    });
    
    // ❌ BROKEN: Unsafe string with memory issues
    group.bench_function("unsafe_string", |b| {
        b.iter(|| {
            let mut s = UnsafeString::new();
            unsafe {
                for ch in "Hello, World!".chars() {
                    // ❌ BROKEN: This will likely cause UB
                    s.push_char_unchecked(ch);
                }
                let result = s.as_str().to_string();
                black_box(result)
            }
        })
    });
    
    group.finish();
}

// ❌ BROKEN: Missing criterion setup
// criterion_group!(
//     benches,
//     benchmark_fast_buffer,
//     benchmark_safe_vs_unsafe,
//     benchmark_async_components,
//     benchmark_macro_performance,
//     benchmark_string_operations
// );
// criterion_main!(benches);

// ❌ BROKEN: Alternative main function that won't work correctly
fn main() {
    println!("Benchmarks would run here if criterion was properly set up");
    
    // ❌ BROKEN: Manual timing that doesn't account for JIT warmup
    let start = std::time::Instant::now();
    
    let mut buffer = FastBuffer::new(10000);
    for i in 0..10000 {
        buffer.push_safe(i).unwrap();
    }
    
    let mut sum = 0;
    for item in buffer.iter() {
        sum += *item;
    }
    
    let duration = start.elapsed();
    println!("FastBuffer operations took: {:?}", duration);
    println!("Sum: {}", sum);
}