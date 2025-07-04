// Exercise 06: Unsafe Undefined - Unsafe Code and Undefined Behavior
//
// Learning Objectives:
// - Understand unsafe code and its responsibilities
// - Debug undefined behavior issues
// - Learn memory safety contract violations
// - Compare with C# unsafe code and pointer operations
//
// C# Analogy: Like unsafe code blocks with pointers, but with stricter rules
// and more responsibility for maintaining memory safety invariants.
//
// Your Mission: Fix the unsafe code to eliminate undefined behavior.
// Each checkpoint deals with different unsafe operations and their contracts.

use std::ptr;
use std::slice;
use std::alloc::{alloc, dealloc, Layout};

// ❌ CHECKPOINT 1: Raw Pointer Dereferencing Issues
// This unsafe code has undefined behavior with raw pointers
// C# equivalent: Unsafe pointer dereferencing without proper bounds checking
unsafe fn process_raw_pointer(ptr: *const i32) -> i32 {
    // ❌ This dereference is potentially undefined behavior
    *ptr
}

unsafe fn modify_through_pointer(ptr: *mut i32, value: i32) {
    // ❌ This write is potentially undefined behavior
    *ptr = value;
}

// ❌ CHECKPOINT 2: Uninitialized Memory Issues
// This code uses uninitialized memory unsafely
// C# equivalent: Using uninitialized memory in unsafe context
unsafe fn create_uninitialized_array<T>(size: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    // ❌ This is undefined behavior - setting length without initialization
    vec.set_len(size);
    vec
}

unsafe fn read_uninitialized() -> [i32; 10] {
    // ❌ This creates uninitialized memory
    let mut array: [i32; 10] = std::mem::uninitialized();
    array
}

// ❌ CHECKPOINT 3: Aliasing and Mutable Reference Issues
// This code violates aliasing rules with unsafe pointers
// C# equivalent: Multiple mutable references to the same memory
unsafe fn create_aliased_references(data: &mut [i32]) -> (&mut i32, &mut i32) {
    let ptr = data.as_mut_ptr();
    // ❌ This creates aliased mutable references
    let ref1 = &mut *ptr;
    let ref2 = &mut *ptr;
    (ref1, ref2)
}

unsafe fn violate_aliasing_rules(slice: &mut [i32]) {
    let ptr = slice.as_mut_ptr();
    // ❌ This violates aliasing by creating overlapping mutable references
    let first_half = slice::from_raw_parts_mut(ptr, slice.len() / 2);
    let second_half = slice::from_raw_parts_mut(ptr, slice.len());
    
    first_half[0] = 42;
    second_half[0] = 24; // Undefined behavior due to aliasing
}

// ❌ CHECKPOINT 4: Manual Memory Management Issues
// This code has memory safety violations in manual allocation
// C# equivalent: Manual memory management with Marshal.AllocHGlobal
unsafe fn allocate_and_leak() -> *mut i32 {
    let layout = Layout::new::<i32>();
    let ptr = alloc(layout) as *mut i32;
    
    // ❌ This write might be to invalid memory
    *ptr = 42;
    
    // ❌ Memory is never freed (leak)
    ptr
}

unsafe fn double_free_error() {
    let layout = Layout::new::<i32>();
    let ptr = alloc(layout) as *mut i32;
    
    // ❌ This deallocates the same memory twice
    dealloc(ptr as *mut u8, layout);
    dealloc(ptr as *mut u8, layout); // Undefined behavior
}

unsafe fn use_after_free() {
    let layout = Layout::new::<i32>();
    let ptr = alloc(layout) as *mut i32;
    *ptr = 42;
    
    dealloc(ptr as *mut u8, layout);
    
    // ❌ This is use-after-free - undefined behavior
    println!("Value: {}", *ptr);
}

// ❌ CHECKPOINT 5: Transmute and Type Casting Issues
// This code misuses transmute and type casting
// C# equivalent: Unsafe casting between incompatible types
unsafe fn bad_transmute_example() {
    let x = 42u64;
    // ❌ This transmute is invalid - different sizes
    let y: u32 = std::mem::transmute(x);
    println!("Transmuted: {}", y);
}

unsafe fn invalid_slice_creation() {
    let number = 42u64;
    let ptr = &number as *const u64 as *const u8;
    // ❌ This creates a slice with invalid lifetime and bounds
    let slice = slice::from_raw_parts(ptr, 100);
    println!("Slice: {:?}", slice);
}

// ❌ CHECKPOINT 6: Lifetime Extension Issues
// This code extends lifetimes unsafely
// C# equivalent: Keeping references to freed memory
unsafe fn extend_lifetime_unsafely<'a>(data: &str) -> &'a str {
    let local = data.to_string();
    // ❌ This extends lifetime beyond the local string's scope
    std::mem::transmute(local.as_str())
}

unsafe fn return_dangling_reference() -> &'static str {
    let local = "temporary".to_string();
    // ❌ This returns a reference to local memory
    std::mem::transmute(local.as_str())
}

// ❌ CHECKPOINT 7: Unsafe Trait Implementation Issues
// This unsafe trait implementation violates its contract
// C# equivalent: Implementing unsafe interfaces incorrectly
unsafe trait UnsafeMarker {
    // This trait promises certain safety invariants
    fn is_safe_to_use(&self) -> bool;
}

struct UnsafeStruct {
    ptr: *mut i32,
}

// ❌ This implementation doesn't maintain safety invariants
unsafe impl UnsafeMarker for UnsafeStruct {
    fn is_safe_to_use(&self) -> bool {
        // ❌ This doesn't actually check if the pointer is valid
        true
    }
}

// ❌ CHECKPOINT 8: Unsafe Abstraction Boundary Issues
// This safe function wraps unsafe code incorrectly
// C# equivalent: Public safe methods that don't properly validate unsafe operations
fn safe_wrapper_with_bugs(data: &[i32], index: usize) -> i32 {
    unsafe {
        // ❌ This doesn't validate the bounds before unsafe access
        let ptr = data.as_ptr().add(index);
        *ptr
    }
}

fn safe_slice_creation(ptr: *const i32, len: usize) -> &'static [i32] {
    unsafe {
        // ❌ This creates a slice without validating the pointer or lifetime
        slice::from_raw_parts(ptr, len)
    }
}

// Helper functions for testing
fn create_test_data() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

fn print_separator(title: &str) {
    println!("\n=== {} ===", title);
}

fn main() {
    println!("=== Unsafe Undefined Exercise ===");
    
    // Test Checkpoint 1: Raw pointer dereferencing
    print_separator("Raw Pointer Issues");
    let value = 42;
    unsafe {
        let result = process_raw_pointer(&value);
        println!("Raw pointer result: {}", result);
        
        let mut mutable_value = 10;
        modify_through_pointer(&mut mutable_value, 20);
        println!("Modified value: {}", mutable_value);
    }
    
    // Test Checkpoint 2: Uninitialized memory
    print_separator("Uninitialized Memory");
    unsafe {
        // These will cause undefined behavior
        let uninitialized_vec: Vec<i32> = create_uninitialized_array(5);
        println!("Uninitialized vec length: {}", uninitialized_vec.len());
        
        // This will also cause undefined behavior
        let uninitialized_array = read_uninitialized();
        println!("Uninitialized array: {:?}", &uninitialized_array[0..3]);
    }
    
    // Test Checkpoint 3: Aliasing violations
    print_separator("Aliasing Issues");
    let mut data = create_test_data();
    unsafe {
        let (ref1, ref2) = create_aliased_references(&mut data);
        // This will cause undefined behavior
        *ref1 = 100;
        *ref2 = 200;
        println!("Aliased references: {} {}", ref1, ref2);
        
        violate_aliasing_rules(&mut data);
    }
    
    // Test Checkpoint 4: Manual memory management
    print_separator("Memory Management");
    unsafe {
        let leaked_ptr = allocate_and_leak();
        println!("Leaked pointer: {:?}", leaked_ptr);
        
        // These will cause undefined behavior
        double_free_error();
        use_after_free();
    }
    
    // Test Checkpoint 5: Transmute issues
    print_separator("Transmute Issues");
    unsafe {
        bad_transmute_example();
        invalid_slice_creation();
    }
    
    // Test Checkpoint 6: Lifetime extension
    print_separator("Lifetime Issues");
    unsafe {
        let extended = extend_lifetime_unsafely("test");
        println!("Extended lifetime: {}", extended);
        
        let dangling = return_dangling_reference();
        println!("Dangling reference: {}", dangling);
    }
    
    // Test Checkpoint 7: Unsafe trait implementations
    print_separator("Unsafe Trait Issues");
    let unsafe_struct = UnsafeStruct {
        ptr: std::ptr::null_mut(),
    };
    unsafe {
        println!("Is safe to use: {}", unsafe_struct.is_safe_to_use());
    }
    
    // Test Checkpoint 8: Safe wrapper issues
    print_separator("Safe Wrapper Issues");
    let test_data = create_test_data();
    let result = safe_wrapper_with_bugs(&test_data, 10); // Out of bounds
    println!("Safe wrapper result: {}", result);
    
    let slice = safe_slice_creation(std::ptr::null(), 10);
    println!("Safe slice creation: {:?}", slice);
    
    println!("🎉 Unsafe concepts demonstrated (with undefined behavior)!");
}

// C# Comparison Notes:
//
// 1. Unsafe code in Rust is stricter than C# unsafe blocks
// 2. Raw pointers are like C# pointers but with no GC protection
// 3. Manual memory management is like Marshal.AllocHGlobal operations
// 4. Transmute is like unsafe casting but more dangerous
// 5. Lifetime extension is like keeping references to freed memory
// 6. Unsafe traits are like unsafe interfaces with stricter contracts
// 7. Safe wrappers must validate all unsafe operations
// 8. Undefined behavior is more strictly defined than in C#

// Key Differences from C#:
// - No garbage collector to protect against use-after-free
// - Stricter aliasing rules than C# references
// - More explicit about undefined behavior
// - Better tools for detecting unsafe issues
// - More responsibility on the programmer for safety