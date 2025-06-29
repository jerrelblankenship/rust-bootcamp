// Exercise 1 Solution: Memory Layout Problems - WORKING VERSION
//
// This is the complete, working solution for ex01-fix-memory-layout.rs
// Compare this with your implementation to understand the concepts better.

use std::mem;
use std::borrow::Cow;

fn main() {
    println!("=== Exercise 1 Solution: Memory Layout Problems ===\n");
    
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
    exercise_1_6();
    exercise_1_7();
}

// Exercise 1.1: Fix the struct definition compilation error
fn exercise_1_1() {
    println!("Exercise 1.1: Fix struct definition");
    
    // FIXED: Added missing comma after String field
    #[derive(Debug)]
    struct FixedStruct {
        field1: u8,
        field2: String,  // Added missing comma
        field3: i32,     // Fixed syntax
    }
    
    // FIXED: Initialize all fields
    let data = FixedStruct {
        field1: 42,
        field2: String::from("Hello, Rust!"),
        field3: 12345,
    };
    
    println!("Struct: {:?}", data);
    println!("Struct size: {} bytes", mem::size_of::<FixedStruct>());
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Fix memory layout optimization
fn exercise_1_2() {
    println!("Exercise 1.2: Optimize struct layout");
    
    // Original wasteful layout
    #[derive(Debug)]
    struct WastefulLayout {
        a: u8,      // 1 byte
        b: u64,     // 8 bytes - causes 7 bytes padding after 'a'
        c: u8,      // 1 byte  
        d: u32,     // 4 bytes - causes 3 bytes padding after 'c'
        e: u16,     // 2 bytes - causes 6 bytes padding at end for alignment
    }
    
    // FIXED: Optimized version - larger fields first
    #[derive(Debug)]
    struct OptimizedLayout {
        b: u64,     // 8 bytes - largest first
        d: u32,     // 4 bytes
        e: u16,     // 2 bytes
        a: u8,      // 1 byte
        c: u8,      // 1 byte - only 6 bytes padding at end instead of 16
    }
    
    let wasteful_size = mem::size_of::<WastefulLayout>();
    let optimized_size = mem::size_of::<OptimizedLayout>();
    
    println!("Wasteful size: {} bytes", wasteful_size);
    println!("Optimized size: {} bytes", optimized_size);
    
    let savings = wasteful_size - optimized_size;
    println!("Memory saved: {} bytes ({:.1}% reduction)", 
             savings, (savings as f64 / wasteful_size as f64) * 100.0);
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Fix stack overflow from large allocation
fn exercise_1_3() {
    println!("Exercise 1.3: Fix stack overflow");
    
    // FIXED: Use heap allocation instead of stack
    fn create_large_data_safely() -> Vec<u8> {
        vec![0; 10_000_000]  // 10MB on heap instead of stack
    }
    
    // Test the fixed version
    let data = create_large_data_safely();
    println!("Created {} bytes on heap", data.len());
    
    // Performance comparison
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _small_stack = [0u8; 1024];  // 1KB stack allocation
    }
    let stack_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _small_heap = vec![0u8; 1024];  // 1KB heap allocation
    }
    let heap_time = start.elapsed();
    
    println!("Stack allocation time: {:?}", stack_time);
    println!("Heap allocation time: {:?}", heap_time);
    println!("Stack is ~{:.1}x faster for small allocations", 
             heap_time.as_nanos() as f64 / stack_time.as_nanos() as f64);
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Fix zero-copy string processing
fn exercise_1_4() {
    println!("Exercise 1.4: Fix string processing efficiency");
    
    // FIXED: Use Cow for zero-copy when possible
    fn process_string_efficient(input: &str) -> Cow<str> {
        if input.chars().any(|c| c.is_lowercase()) {
            // Has lowercase - need to allocate
            Cow::Owned(input.to_uppercase())
        } else {
            // Already uppercase - borrow original
            Cow::Borrowed(input)
        }
    }
    
    // Test cases
    let test_cases = [
        "hello world",      // Should allocate (has lowercase)
        "HELLO WORLD",      // Should NOT allocate (already uppercase)
        "Mixed Case",       // Should allocate (has lowercase)
        "123 NUMBERS",      // Should NOT allocate (no lowercase)
    ];
    
    for case in test_cases {
        let result = process_string_efficient(case);
        let borrowed = matches!(result, Cow::Borrowed(_));
        println!("Input: '{}' -> Output: '{}' (borrowed: {})", 
                 case, result, borrowed);
    }
    
    println!("✅ Exercise 1.4 complete\n");
}

// Exercise 1.5: Fix memory alignment issues
fn exercise_1_5() {
    println!("Exercise 1.5: Fix memory alignment");
    
    // Original unaligned data
    #[derive(Debug)]
    struct UnalignedData {
        values: [f32; 8],  // Default alignment (4 bytes)
    }
    
    // FIXED: Properly aligned for SIMD operations
    #[repr(align(16))]  // Force 16-byte alignment
    #[derive(Debug)]
    struct AlignedData {
        values: [f32; 8],
    }
    
    let unaligned = UnalignedData { values: [1.0; 8] };
    let aligned = AlignedData { values: [1.0; 8] };
    
    println!("Unaligned address: {:p}", &unaligned.values);
    println!("Aligned address: {:p}", &aligned.values);
    
    // Verify alignment
    println!("Unaligned alignment: {}", mem::align_of::<UnalignedData>());
    println!("Aligned alignment: {}", mem::align_of::<AlignedData>());
    
    // Check that aligned address is actually 16-byte aligned
    let aligned_addr = &aligned.values as *const _ as usize;
    println!("Aligned address modulo 16: {}", aligned_addr % 16);
    assert_eq!(aligned_addr % 16, 0, "Should be 16-byte aligned");
    
    println!("✅ Exercise 1.5 complete\n");
}

// Exercise 1.6: Fix buffer reuse pattern
fn exercise_1_6() {
    println!("Exercise 1.6: Fix buffer reuse");
    
    // FIXED: Complete buffer pool implementation
    struct BufferPool {
        buffers: Vec<Vec<u8>>,
        buffer_size: usize,
    }
    
    impl BufferPool {
        fn new(buffer_size: usize, count: usize) -> Self {
            let mut buffers = Vec::new();
            
            // FIXED: Added missing parenthesis and proper loop
            for _i in 0..count {
                buffers.push(Vec::with_capacity(buffer_size));
            }
            
            Self { buffers, buffer_size }  // FIXED: Added Self
        }
        
        fn get_buffer(&mut self) -> Vec<u8> {
            // Return a buffer from the pool, or create new one if empty
            self.buffers.pop().unwrap_or_else(|| {
                Vec::with_capacity(self.buffer_size)
            })
        }
        
        fn return_buffer(&mut self, mut buffer: Vec<u8>) {
            // Clear buffer and return to pool if it has correct capacity
            buffer.clear();
            if buffer.capacity() == self.buffer_size {
                self.buffers.push(buffer);
            }
            // If capacity doesn't match, just drop it
        }
    }
    
    // Test the buffer pool
    let mut pool = BufferPool::new(1024, 3);
    
    let mut buffer1 = pool.get_buffer();
    buffer1.extend_from_slice(b"Hello, World!");
    
    println!("Buffer1 length: {}", buffer1.len());
    println!("Buffer1 capacity: {}", buffer1.capacity());
    
    pool.return_buffer(buffer1);
    
    let buffer2 = pool.get_buffer();
    println!("Buffer2 capacity: {} (should be reused)", buffer2.capacity());
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Advanced - Memory layout analysis
fn exercise_1_7() {
    println!("Exercise 1.7: Memory layout analysis");
    
    #[derive(Debug)]
    struct ComplexStruct {
        flag: bool,
        id: u64,
        name: String,
        data: Vec<u32>,
        optional: Option<Box<String>>,
    }
    
    // Analyze and print memory layout information
    fn analyze_layout() {
        println!("ComplexStruct analysis:");
        println!("Size: {} bytes", mem::size_of::<ComplexStruct>());
        println!("Alignment: {} bytes", mem::align_of::<ComplexStruct>());
        
        // Print sizes of individual field types
        println!("Field type sizes:");
        println!("  bool: {} bytes", mem::size_of::<bool>());
        println!("  u64: {} bytes", mem::size_of::<u64>());
        println!("  String: {} bytes", mem::size_of::<String>());
        println!("  Vec<u32>: {} bytes", mem::size_of::<Vec<u32>>());
        println!("  Option<Box<String>>: {} bytes", mem::size_of::<Option<Box<String>>>());
        
        let instance = ComplexStruct {
            flag: true,
            id: 12345,
            name: String::from("test"),
            data: vec![1, 2, 3, 4, 5],
            optional: Some(Box::new(String::from("optional data"))),
        };
        
        // Print actual memory addresses to see layout
        unsafe {
            let base = &instance as *const ComplexStruct as usize;
            println!("\nField offsets:");
            println!("Struct base address: 0x{:x}", base);
            
            let flag_addr = &instance.flag as *const bool as usize;
            println!("  flag offset: {} bytes", flag_addr - base);
            
            let id_addr = &instance.id as *const u64 as usize;
            println!("  id offset: {} bytes", id_addr - base);
            
            let name_addr = &instance.name as *const String as usize;
            println!("  name offset: {} bytes", name_addr - base);
            
            let data_addr = &instance.data as *const Vec<u32> as usize;
            println!("  data offset: {} bytes", data_addr - base);
            
            let optional_addr = &instance.optional as *const Option<Box<String>> as usize;
            println!("  optional offset: {} bytes", optional_addr - base);
        }
        
        // Show memory usage breakdown
        println!("\nMemory usage breakdown:");
        println!("Stack (struct): {} bytes", mem::size_of::<ComplexStruct>());
        println!("Heap (String content): {} bytes", instance.name.capacity());
        println!("Heap (Vec content): {} bytes", instance.data.capacity() * mem::size_of::<u32>());
        if let Some(ref boxed_string) = instance.optional {
            println!("Heap (Box<String>): {} bytes", mem::size_of::<String>() + boxed_string.capacity());
        }
    }
    
    analyze_layout();
    println!("✅ Exercise 1.7 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_optimized_layout() {
        assert!(mem::size_of::<OptimizedLayout>() < mem::size_of::<WastefulLayout>());
        // Specific sizes might vary by platform, but optimized should always be smaller
    }
    
    #[test]
    fn test_buffer_pool() {
        let mut pool = BufferPool::new(100, 2);
        let buffer = pool.get_buffer();
        assert_eq!(buffer.capacity(), 100);
        
        pool.return_buffer(buffer);
        let reused = pool.get_buffer();
        assert_eq!(reused.capacity(), 100);
    }
    
    #[test]
    fn test_string_efficiency() {
        use std::borrow::Cow;
        
        let result = process_string_efficient("HELLO");
        assert!(matches!(result, Cow::Borrowed(_)));
        
        let result = process_string_efficient("hello");
        assert!(matches!(result, Cow::Owned(_)));
    }
    
    #[test]
    fn test_alignment() {
        assert_eq!(mem::align_of::<AlignedData>(), 16);
        
        let data = AlignedData { values: [1.0; 8] };
        let addr = &data as *const _ as usize;
        assert_eq!(addr % 16, 0, "Should be 16-byte aligned");
    }
}

// KEY LEARNING POINTS:
//
// 1. STRUCT LAYOUT OPTIMIZATION:
//    - Put larger fields first to minimize padding
//    - Understand how alignment affects total struct size
//    - Use #[repr(packed)] carefully (can hurt performance)
//
// 2. STACK VS HEAP ALLOCATION:
//    - Stack is faster but limited in size
//    - Use heap for large data structures
//    - Consider allocation frequency in performance-critical code
//
// 3. ZERO-COPY PATTERNS:
//    - Use Cow<str> to avoid unnecessary string allocations
//    - Borrow when possible, allocate only when needed
//    - Measure the performance impact of different approaches
//
// 4. MEMORY ALIGNMENT:
//    - SIMD operations require specific alignment
//    - Use #[repr(align(N))] to control alignment
//    - Alignment affects cache performance
//
// 5. BUFFER REUSE:
//    - Reuse allocations to reduce GC pressure
//    - Pool pattern for frequent allocations/deallocations
//    - Balance memory usage vs allocation performance
//
// 6. MEMORY LAYOUT ANALYSIS:
//    - Use std::mem::size_of and align_of for analysis
//    - Understand the difference between stack and heap usage
//    - Profile memory usage in real applications
//
// C# COMPARISON:
// - C#: CLR controls layout, GC handles cleanup
// - Rust: Manual control over layout and allocation
// - C#: Reference types always on heap
// - Rust: Choose stack or heap allocation explicitly
// - C#: GC pressure from frequent allocations
// - Rust: Zero-cost abstractions, explicit memory management
