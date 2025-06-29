// Exercise 1: Memory Layout Exploration
//
// This exercise helps you understand how Rust lays out data in memory
// and how to analyze memory usage patterns.

use std::mem;

fn main() {
    println!("=== Exercise 1: Memory Layout Exploration ===\n");
    
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
}

// Exercise 1.1: Basic type analysis
fn exercise_1_1() {
    println!("Exercise 1.1: Basic type analysis");
    
    // TODO: Complete the analysis for these types
    // Print size, alignment, and analyze the results
    
    println!("Primitive types:");
    print_type_info::<u8>("u8");
    print_type_info::<u16>("u16");
    print_type_info::<u32>("u32");
    print_type_info::<u64>("u64");
    print_type_info::<usize>("usize");
    print_type_info::<f32>("f32");
    print_type_info::<f64>("f64");
    
    println!("\nCompound types:");
    print_type_info::<(u8, u32)>("(u8, u32)");
    print_type_info::<(u32, u8)>("(u32, u8)");
    print_type_info::<[u8; 4]>("[u8; 4]");
    print_type_info::<[u32; 4]>("[u32; 4]");
    
    // TODO: Explain why (u8, u32) and (u32, u8) have different sizes
    // Your answer here:
    
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Struct layout optimization
fn exercise_1_2() {
    println!("Exercise 1.2: Struct layout optimization");
    
    // TODO: Define three versions of a struct with the same fields
    // but different ordering to see how it affects size
    
    #[derive(Debug)]
    struct Unoptimized {
        // TODO: Define fields in a suboptimal order
        // Include: u8, u64, u16, u8, u32
    }
    
    #[derive(Debug)]
    struct Optimized {
        // TODO: Reorder fields for optimal packing
        // Same fields as above, but reordered
    }
    
    #[repr(packed)]
    #[derive(Debug)]
    struct Packed {
        // TODO: Same fields as Unoptimized but packed
    }
    
    // TODO: Compare sizes and explain the differences
    print_type_info::<Unoptimized>("Unoptimized");
    print_type_info::<Optimized>("Optimized");
    print_type_info::<Packed>("Packed");
    
    // TODO: Calculate padding for each struct
    // Your analysis here:
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Memory address analysis
fn exercise_1_3() {
    println!("Exercise 1.3: Memory address analysis");
    
    #[derive(Debug)]
    struct TestStruct {
        a: u8,
        b: u32,
        c: u16,
        d: u64,
    }
    
    let test_instance = TestStruct {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };
    
    // TODO: Print the memory addresses of each field
    // Calculate and verify the padding between fields
    // Use unsafe code to examine memory layout
    
    unsafe {
        let base_addr = &test_instance as *const TestStruct as usize;
        println!("Struct base address: 0x{:x}", base_addr);
        
        // TODO: Print field addresses and calculate offsets
        // let field_a_addr = &test_instance.a as *const u8 as usize;
        // Your code here...
        
        // TODO: Verify your understanding by examining raw memory
        // Use std::slice::from_raw_parts to view the bytes
    }
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Stack vs heap allocation
fn exercise_1_4() {
    println!("Exercise 1.4: Stack vs heap allocation");
    
    // TODO: Create the same data on stack and heap
    // Compare addresses and analyze allocation patterns
    
    // Stack allocation
    let stack_data = [1u32; 1000]; // 4KB on stack
    
    // Heap allocation
    let heap_data = vec![1u32; 1000]; // 4KB on heap
    
    // TODO: Print addresses and analyze the difference
    println!("Stack data address: {:p}", &stack_data);
    println!("Heap data address: {:p}", heap_data.as_ptr());
    println!("Heap Vec address: {:p}", &heap_data);
    
    // TODO: Demonstrate what happens with large allocations
    // Try creating a very large array on stack vs heap
    // (Be careful with stack size - use smaller arrays for testing)
    
    // TODO: Measure allocation performance difference
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _stack_alloc = [0u8; 1024];
    }
    let stack_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _heap_alloc = vec![0u8; 1024];
    }
    let heap_time = start.elapsed();
    
    println!("Stack allocation time: {:?}", stack_time);
    println!("Heap allocation time: {:?}", heap_time);
    
    println!("✅ Exercise 1.4 complete\n");
}

// Exercise 1.5: Zero-copy operations
fn exercise_1_5() {
    println!("Exercise 1.5: Zero-copy operations");
    
    let data = "The quick brown fox jumps over the lazy dog. ".repeat(100);
    
    // TODO: Implement functions that demonstrate zero-copy principles
    // 1. String processing without allocation when possible
    // 2. Slice operations that create views
    // 3. Cow (Clone on Write) usage
    
    fn count_words_copying(input: &str) -> (String, usize) {
        // TODO: Process string with unnecessary copying
        todo!()
    }
    
    fn count_words_zero_copy(input: &str) -> (&str, usize) {
        // TODO: Process string without copying
        todo!()
    }
    
    use std::borrow::Cow;
    
    fn process_string_cow(input: &str) -> Cow<str> {
        // TODO: Return borrowed if no changes needed, owned if modified
        // Uppercase the string only if it contains lowercase letters
        todo!()
    }
    
    // TODO: Benchmark the different approaches
    use std::time::Instant;
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = count_words_copying(&data);
    }
    let copying_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = count_words_zero_copy(&data);
    }
    let zero_copy_time = start.elapsed();
    
    println!("Copying approach: {:?}", copying_time);
    println!("Zero-copy approach: {:?}", zero_copy_time);
    
    // TODO: Test Cow behavior
    let test_cases = ["hello world", "HELLO WORLD"];
    for case in test_cases {
        let result = process_string_cow(case);
        println!("Input: '{}' -> Output: '{}' (borrowed: {})", 
                 case, result, matches!(result, Cow::Borrowed(_)));
    }
    
    println!("✅ Exercise 1.5 complete\n");
}

// Helper function
fn print_type_info<T>(name: &str) {
    println!("{}: {} bytes, alignment: {} bytes", 
             name, mem::size_of::<T>(), mem::align_of::<T>());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_sizes() {
        // Basic sanity checks
        assert_eq!(mem::size_of::<u8>(), 1);
        assert_eq!(mem::size_of::<u32>(), 4);
        assert_eq!(mem::size_of::<u64>(), 8);
    }
    
    #[test]
    fn test_struct_layout() {
        #[repr(C)]
        struct TestStruct {
            a: u8,
            b: u32,
        }
        
        // C representation should have predictable layout
        assert!(mem::size_of::<TestStruct>() >= 5); // At least the sum of fields
    }
}
