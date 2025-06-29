// Exercise 1: Memory Layout Problems - IMPROVED SCAFFOLDING
//
// This exercise teaches memory layout through progressive compilation errors.
// Fix ONE error at a time, compile, then move to the next function.
//
// APPROACH: 
// - Each function has exactly ONE main error to fix
// - Compile after each fix: `rustc ex01-memory-layout.rs`
// - Don't move to the next function until current one compiles
// - Use hints only after trying for 15+ minutes

use std::mem;
use std::borrow::Cow;

fn main() {
    println!("=== Exercise 1: Memory Layout Problems (Progressive) ===\n");
    
    // Fix these ONE AT A TIME - uncomment as you go
    step_1_struct_syntax();
    // step_2_memory_optimization();
    // step_3_heap_allocation();
    // step_4_zero_copy_strings();
    // step_5_memory_alignment();
    // step_6_buffer_reuse();
    
    println!("All exercises complete! 🎉");
}

// STEP 1: Fix the struct syntax error (ONE error only)
fn step_1_struct_syntax() {
    println!("Step 1: Fix struct definition");
    
    // FIXME: This struct has a syntax error - fix it!
    #[derive(Debug)]
    struct SimpleStruct {
        field1: u8,
        field2: String  // What's missing here?
        field3: i32,
    }
    
    // Don't modify this - it should work once struct is fixed
    let data = SimpleStruct {
        field1: 42,
        field2: String::from("Hello"),
        field3: 100,
    };
    
    println!("✅ Step 1 complete: {:?}", data);
    println!("Now uncomment step_2_memory_optimization() in main()\n");
}

// STEP 2: Optimize memory layout (ONE concept only)
fn step_2_memory_optimization() {
    println!("Step 2: Optimize struct layout");
    
    // This struct wastes memory due to padding
    #[derive(Debug)]
    struct WastefulStruct {
        a: u8,      // 1 byte
        b: u64,     // 8 bytes - but causes padding!
        c: u8,      // 1 byte
    }
    
    // TODO: Create an optimized version by reordering fields
    // HINT: Put larger fields first
    #[derive(Debug)]
    struct OptimizedStruct {
        // TODO: Reorder these fields to minimize padding
        // a: u8,
        // b: u64, 
        // c: u8,
    }
    
    let wasteful_size = mem::size_of::<WastefulStruct>();
    // let optimized_size = mem::size_of::<OptimizedStruct>();  // Uncomment when ready
    
    println!("Wasteful struct size: {} bytes", wasteful_size);
    // println!("Optimized struct size: {} bytes", optimized_size);
    
    println!("✅ Step 2 complete");
    println!("Now uncomment step_3_heap_allocation() in main()\n");
}

// STEP 3: Fix stack overflow with heap allocation (ONE fix only)
fn step_3_heap_allocation() {
    println!("Step 3: Fix stack overflow");
    
    // FIXME: This will cause stack overflow - too big for stack!
    fn create_large_data() -> [u8; 1_000_000] {  // 1MB on stack
        [0; 1_000_000]
    }
    
    // TODO: Fix by using heap allocation instead
    // HINT: What data structure allocates on the heap?
    
    // Test your fix here
    // let data = create_large_data_fixed();
    // println!("Created {} bytes safely", data.len());
    
    println!("✅ Step 3 complete");
    println!("Now uncomment step_4_zero_copy_strings() in main()\n");
}

// STEP 4: Implement zero-copy string processing (ONE pattern only)
fn step_4_zero_copy_strings() {
    println!("Step 4: Zero-copy string processing");
    
    // TODO: Implement this function using Cow<str>
    // Only allocate if the string contains lowercase letters
    fn process_string_efficiently(input: &str) -> Cow<str> {
        // TODO: Check if string has lowercase letters
        // If yes: return Cow::Owned(input.to_uppercase())
        // If no: return Cow::Borrowed(input)
        todo!("Implement zero-copy string processing")
    }
    
    // Test cases
    let test_cases = ["HELLO", "hello", "Hello World"];
    
    for case in test_cases {
        // let result = process_string_efficiently(case);
        // let borrowed = matches!(result, Cow::Borrowed(_));
        // println!("'{}' -> borrowed: {}", case, borrowed);
    }
    
    println!("✅ Step 4 complete");
    println!("Now uncomment step_5_memory_alignment() in main()\n");
}

// STEP 5: Fix memory alignment for SIMD (ONE attribute only)
fn step_5_memory_alignment() {
    println!("Step 5: Memory alignment");
    
    // Default alignment (might not be optimal for SIMD)
    #[derive(Debug)]
    struct UnalignedData {
        values: [f32; 8],
    }
    
    // TODO: Add proper alignment for SIMD operations
    // HINT: Use #[repr(align(16))]
    #[derive(Debug)]
    struct AlignedData {
        values: [f32; 8],
    }
    
    let unaligned = UnalignedData { values: [1.0; 8] };
    // let aligned = AlignedData { values: [1.0; 8] };
    
    println!("Unaligned alignment: {}", mem::align_of::<UnalignedData>());
    // println!("Aligned alignment: {}", mem::align_of::<AlignedData>());
    
    println!("✅ Step 5 complete");
    println!("Now uncomment step_6_buffer_reuse() in main()\n");
}

// STEP 6: Implement buffer reuse pattern (ONE missing method only)
fn step_6_buffer_reuse() {
    println!("Step 6: Buffer reuse pattern");
    
    struct BufferPool {
        buffers: Vec<Vec<u8>>,
        buffer_size: usize,
    }
    
    impl BufferPool {
        fn new(buffer_size: usize, count: usize) -> Self {
            let mut buffers = Vec::new();
            for _i in 0..count {
                buffers.push(Vec::with_capacity(buffer_size));
            }
            Self { buffers, buffer_size }
        }
        
        fn get_buffer(&mut self) -> Vec<u8> {
            self.buffers.pop().unwrap_or_else(|| {
                Vec::with_capacity(self.buffer_size)
            })
        }
        
        // TODO: Implement this method
        fn return_buffer(&mut self, buffer: Vec<u8>) {
            // TODO: Clear the buffer and return it to the pool
            // Only return buffers with the correct capacity
            todo!("Implement buffer return")
        }
    }
    
    // Test the buffer pool
    let mut pool = BufferPool::new(1024, 2);
    let mut buffer = pool.get_buffer();
    buffer.extend_from_slice(b"Hello, World!");
    
    println!("Buffer length: {}", buffer.len());
    
    // pool.return_buffer(buffer);  // Uncomment when implemented
    
    println!("✅ Step 6 complete");
}

// BONUS: Advanced memory analysis (optional)
#[allow(dead_code)]
fn bonus_memory_analysis() {
    println!("Bonus: Memory layout analysis");
    
    #[derive(Debug)]
    struct ComplexStruct {
        flag: bool,
        id: u64,
        name: String,
        data: Vec<u32>,
    }
    
    let instance = ComplexStruct {
        flag: true,
        id: 12345,
        name: String::from("test"),
        data: vec![1, 2, 3, 4, 5],
    };
    
    println!("Struct size: {} bytes", mem::size_of::<ComplexStruct>());
    println!("Struct alignment: {} bytes", mem::align_of::<ComplexStruct>());
    
    // Print field offsets
    unsafe {
        let base = &instance as *const ComplexStruct as usize;
        let flag_addr = &instance.flag as *const bool as usize;
        let id_addr = &instance.id as *const u64 as usize;
        
        println!("flag offset: {} bytes", flag_addr - base);
        println!("id offset: {} bytes", id_addr - base);
    }
    
    println!("✅ Bonus complete");
}

// COMPILATION HINTS:
// 
// Step 1: Look for missing punctuation in the struct definition
// Step 2: Larger fields should come first (u64 before u8)
// Step 3: Use Vec instead of array for large data
// Step 4: Use Cow::Borrowed for no-change, Cow::Owned for changes
// Step 5: Add #[repr(align(16))] before the struct
// Step 6: Clear the buffer, check capacity, then push to pool
//
// LEARNING GOALS:
// - Fix compilation errors one at a time
// - Understand memory layout optimization
// - Learn stack vs heap allocation trade-offs
// - Practice zero-copy patterns
// - Control memory alignment
// - Implement efficient buffer reuse
//
// SUCCESS CRITERIA:
// - All steps compile without errors
// - You understand why each fix works
// - You can explain the performance implications