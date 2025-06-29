// Exercise 1: Fix Memory Layout Problems
//
// Your task: Make this code compile and run efficiently by fixing
// all the memory layout issues, padding problems, and allocation bugs.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex01-fix-memory-layout.rs && ./ex01-fix-memory-layout` to test
// 4. Optimize memory usage and understand layout through hands-on debugging

use std::mem;

fn main() {
    println!("=== Exercise 1: Fix Memory Layout Problems ===\n");
    
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
    exercise_1_6();
}

// Exercise 1.1: Fix the struct definition compilation error
fn exercise_1_1() {
    println!("Exercise 1.1: Fix struct definition");
    
    // FIXME: This struct won't compile - fix the syntax error
    struct BrokenStruct {
        field1: u8,
        field2: String  // COMPILE ERROR: What's missing?
        field3: i32,    // COMPILE ERROR: Fix the syntax
    }
    
    // TODO: Create an instance of the struct
    let data = BrokenStruct {
        // TODO: Initialize all fields
        // field1: ???,
        // field2: ???,
        // field3: ???,
    };
    
    println!("Struct size: {} bytes", mem::size_of::<BrokenStruct>());
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Fix memory layout optimization
fn exercise_1_2() {
    println!("Exercise 1.2: Optimize struct layout");
    
    // FIXME: This struct is poorly organized and wastes memory
    #[derive(Debug)]
    struct WastefulLayout {
        a: u8,      // 1 byte
        b: u64,     // 8 bytes - will cause padding!
        c: u8,      // 1 byte  
        d: u32,     // 4 bytes
        e: u16,     // 2 bytes
    }
    
    // TODO: Create an optimized version that uses less memory
    #[derive(Debug)]
    struct OptimizedLayout {
        // TODO: Reorder fields to minimize padding
        // HINT: Put larger fields first
    }
    
    println!("Wasteful size: {} bytes", mem::size_of::<WastefulLayout>());
    // println!("Optimized size: {} bytes", mem::size_of::<OptimizedLayout>());  // COMPILE ERROR!
    
    // TODO: Calculate how much memory you saved
    // let savings = ???;
    // println!("Memory saved: {} bytes", savings);
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Fix stack overflow from large allocation
fn exercise_1_3() {
    println!("Exercise 1.3: Fix stack overflow");
    
    // FIXME: This will cause stack overflow - too much data on stack
    fn create_large_data() -> [u8; 10_000_000] {  // 10MB on stack!
        [0; 10_000_000]  // RUNTIME ERROR: Stack overflow!
    }
    
    // TODO: Fix by using heap allocation instead
    fn create_large_data_safely() -> Vec<u8> {
        // TODO: Create 10MB of data on heap instead of stack
        todo!("Create large data on heap")
    }
    
    // Test the fixed version
    let data = create_large_data_safely();
    println!("Created {} bytes on heap", data.len());
    
    // TODO: Compare stack vs heap allocation performance
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
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Fix zero-copy string processing
fn exercise_1_4() {
    println!("Exercise 1.4: Fix string processing efficiency");
    
    // FIXME: This function allocates unnecessarily
    fn process_string_wasteful(input: &str) -> String {
        // This always allocates, even when no processing is needed
        input.to_uppercase()  // INEFFICIENT: Always allocates!
    }
    
    use std::borrow::Cow;
    
    // TODO: Fix to use Cow for zero-copy when possible
    fn process_string_efficient(input: &str) -> Cow<str> {
        // TODO: Only allocate if the string contains lowercase letters
        // If string is already uppercase, return Cow::Borrowed
        // If string needs uppercasing, return Cow::Owned
        todo!("Implement efficient string processing")
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
        println!("Input: '{}' -> Output: '{}' (borrowed: {})", 
                 case, result, matches!(result, Cow::Borrowed(_)));
    }
    
    println!("✅ Exercise 1.4 complete\n");
}

// Exercise 1.5: Fix memory alignment issues
fn exercise_1_5() {
    println!("Exercise 1.5: Fix memory alignment");
    
    // FIXME: This struct has alignment issues for SIMD operations
    #[derive(Debug)]
    struct UnalignedData {
        values: [f32; 8],  // Needs 16-byte alignment for SIMD
    }
    
    // TODO: Create a properly aligned version
    // HINT: Use #[repr(align(16))]
    #[derive(Debug)]
    struct AlignedData {
        // TODO: Apply correct alignment attribute
        values: [f32; 8],
    }
    
    let unaligned = UnalignedData { values: [1.0; 8] };
    // let aligned = AlignedData { values: [1.0; 8] };  // COMPILE ERROR!
    
    println!("Unaligned address: {:p}", &unaligned.values);
    // println!("Aligned address: {:p}", &aligned.values);  // COMPILE ERROR!
    
    // TODO: Verify alignment
    println!("Unaligned alignment: {}", mem::align_of::<UnalignedData>());
    // println!("Aligned alignment: {}", mem::align_of::<AlignedData>());  // COMPILE ERROR!
    
    println!("✅ Exercise 1.5 complete\n");
}

// Exercise 1.6: Fix buffer reuse pattern
fn exercise_1_6() {
    println!("Exercise 1.6: Fix buffer reuse");
    
    // FIXME: This buffer pool has compilation errors
    struct BufferPool {
        buffers: Vec<Vec<u8>>,
        buffer_size: usize,
    }
    
    impl BufferPool {
        fn new(buffer_size: usize, count: usize) -> Self {
            let mut buffers = Vec::new();
            
            // FIXME: This loop has an error
            for i in 0..count {
                buffers.push(Vec::with_capacity(buffer_size);  // COMPILE ERROR: Syntax error!
            }
            
            BufferPool { buffers, buffer_size }  // COMPILE ERROR: Missing Self
        }
        
        fn get_buffer(&mut self) -> Vec<u8> {
            // TODO: Return a buffer from the pool, or create new one if empty
            self.buffers.pop().unwrap_or_else(|| {
                // TODO: Create new buffer with correct capacity
                todo!("Create new buffer")
            })
        }
        
        fn return_buffer(&mut self, buffer: Vec<u8>) {
            // TODO: Clear buffer and return to pool if it has correct capacity
            // HINT: Check buffer.capacity() == self.buffer_size
            todo!("Return buffer to pool")
        }
    }
    
    // Test the buffer pool
    let mut pool = BufferPool::new(1024, 3);
    
    let mut buffer1 = pool.get_buffer();
    buffer1.extend_from_slice(b"Hello, World!");
    
    println!("Buffer1 length: {}", buffer1.len());
    
    pool.return_buffer(buffer1);
    
    let buffer2 = pool.get_buffer();
    println!("Buffer2 capacity: {} (should be reused)", buffer2.capacity());
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Advanced - Fix memory layout analysis
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
    
    // TODO: Analyze and print memory layout information
    fn analyze_layout() {
        println!("ComplexStruct analysis:");
        println!("Size: {} bytes", mem::size_of::<ComplexStruct>());
        println!("Alignment: {} bytes", mem::align_of::<ComplexStruct>());
        
        // TODO: Print sizes of individual field types
        // println!("bool size: {} bytes", mem::size_of::<???>());
        // Add more field analyses...
        
        let instance = ComplexStruct {
            flag: true,
            id: 12345,
            name: String::from("test"),
            data: vec![1, 2, 3, 4, 5],
            optional: Some(Box::new(String::from("optional data"))),
        };
        
        // TODO: Print actual memory addresses to see layout
        unsafe {
            let base = &instance as *const ComplexStruct as usize;
            println!("Struct base address: 0x{:x}", base);
            
            // TODO: Print addresses of each field
            // let flag_addr = &instance.flag as *const bool as usize;
            // println!("flag offset: {}", flag_addr - base);
            // Add more field address calculations...
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
        // TODO: Uncomment when OptimizedLayout is implemented
        // assert!(mem::size_of::<OptimizedLayout>() < mem::size_of::<WastefulLayout>());
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
        
        // TODO: Uncomment when process_string_efficient is implemented
        // let result = process_string_efficient("HELLO");
        // assert!(matches!(result, Cow::Borrowed(_)));
        
        // let result = process_string_efficient("hello");
        // assert!(matches!(result, Cow::Owned(_)));
    }
}

// COMPILATION CHALLENGES:
// 1. Fix struct syntax errors (missing commas, semicolons)
// 2. Implement missing struct definitions
// 3. Fix function return types and implementations
// 4. Apply correct memory alignment attributes
// 5. Implement efficient string processing with Cow
// 6. Create working buffer pool with proper memory management
//
// LEARNING OBJECTIVES:
// - Understand struct memory layout and padding
// - Master stack vs heap allocation decisions
// - Implement zero-copy string processing
// - Control memory alignment for performance
// - Build efficient buffer reuse patterns
// - Debug memory layout issues through addresses
//
// C# COMPARISON:
// C#: class MyClass { public byte A; public long B; }  // CLR controls layout
// Rust: struct MyStruct { a: u8, b: u64 }            // You control layout
//
// C#: string.ToUpper()                                 // Always allocates
// Rust: Cow<str>                                       // Allocate only when needed
