// Exercise 2: Fix Unsafe Rust Code
//
// Your task: Debug and fix unsafe Rust code by resolving compilation errors
// and implementing safe abstractions over dangerous operations.
//
// INSTRUCTIONS:
// 1. Read each FIXME comment and understand the safety issue
// 2. Fix compilation errors while maintaining memory safety
// 3. Run `rustc ex02-unsafe-debugging.rs && ./ex02-unsafe-debugging` to test
// 4. Learn to write safe wrappers around unsafe operations

use std::alloc::{self, Layout};
use std::mem;
use std::ptr;
use std::slice;

fn main() {
    println!("=== Exercise 2: Fix Unsafe Rust Code ===\n");
    
    exercise_2_1();
    exercise_2_2();
    exercise_2_3();
    exercise_2_4();
    exercise_2_5();
    exercise_2_6();
}

// Exercise 2.1: Fix raw pointer dereference errors
fn exercise_2_1() {
    println!("Exercise 2.1: Fix pointer dereference");
    
    let mut x = 42;
    
    // FIXME: This won't compile - raw pointer dereference needs unsafe
    let ptr = &mut x as *mut i32;
    *ptr = 100;  // COMPILE ERROR: Cannot dereference raw pointer outside unsafe
    
    println!("Value through pointer: {}", *ptr);  // COMPILE ERROR: Same issue
    
    // TODO: Create a safe wrapper function
    fn safe_write_through_pointer(ptr: *mut i32, value: i32) {
        // TODO: Add safety checks and unsafe block
        if ptr.is_null() {
            return;  // Safety: Don't dereference null pointers
        }
        
        unsafe {
            // TODO: Write the value through the pointer
        }
    }
    
    // TODO: Create a safe reader function
    fn safe_read_through_pointer(ptr: *const i32) -> Option<i32> {
        // TODO: Implement safe reading with null check
        todo!("Implement safe pointer reading")
    }
    
    safe_write_through_pointer(ptr, 200);
    if let Some(value) = safe_read_through_pointer(ptr) {
        println!("Safe read: {}", value);
    }
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Fix unsafe function calls
fn exercise_2_2() {
    println!("Exercise 2.2: Fix unsafe function calls");
    
    // FIXME: This unsafe function has issues
    unsafe fn dangerous_array_sum(ptr: *const i32, len: usize) -> i32 {
        // SAFETY CONTRACT: ptr must point to at least len valid i32s
        let mut sum = 0;
        for i in 0..len {
            sum += *ptr.add(i);  // This could be undefined behavior!
        }
        sum
    }
    
    // FIXME: Calling unsafe function without unsafe block
    let data = [1, 2, 3, 4, 5];
    let result = dangerous_array_sum(data.as_ptr(), data.len());  // COMPILE ERROR!
    println!("Unsafe sum: {}", result);
    
    // TODO: Create a safe wrapper that upholds the safety contract
    fn safe_array_sum(slice: &[i32]) -> i32 {
        // TODO: Call the unsafe function safely
        // HINT: slice guarantees valid pointer and length
        unsafe {
            dangerous_array_sum(slice.as_ptr(), slice.len())
        }
    }
    
    let safe_result = safe_array_sum(&data);
    println!("Safe sum: {}", safe_result);
    
    // TODO: Test edge cases
    let empty_slice: &[i32] = &[];
    let empty_result = safe_array_sum(empty_slice);
    println!("Empty slice sum: {}", empty_result);
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Fix global mutable state access
fn exercise_2_3() {
    println!("Exercise 2.3: Fix global state access");
    
    // FIXME: Accessing mutable static requires unsafe
    static mut GLOBAL_COUNTER: u32 = 0;
    
    fn increment_counter() {
        GLOBAL_COUNTER += 1;  // COMPILE ERROR: Mutable static access needs unsafe
    }
    
    fn get_counter() -> u32 {
        GLOBAL_COUNTER  // COMPILE ERROR: Same issue
    }
    
    // TODO: Fix the functions to use unsafe properly
    fn safe_increment_counter() {
        unsafe {
            // TODO: Safely increment the global counter
            // WARNING: This is still not thread-safe!
        }
    }
    
    fn safe_get_counter() -> u32 {
        unsafe {
            // TODO: Safely read the global counter
            todo!("Read global counter")
        }
    }
    
    // TODO: Implement a better solution using atomics
    use std::sync::atomic::{AtomicU32, Ordering};
    
    static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
    
    fn atomic_increment() {
        // TODO: Use atomic operation (no unsafe needed!)
        todo!("Implement atomic increment")
    }
    
    fn atomic_get() -> u32 {
        // TODO: Use atomic load operation
        todo!("Implement atomic get")
    }
    
    // Test both approaches
    safe_increment_counter();
    safe_increment_counter();
    println!("Global counter: {}", safe_get_counter());
    
    atomic_increment();
    atomic_increment();
    println!("Atomic counter: {}", atomic_get());
    
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Fix unsafe trait implementation
fn exercise_2_4() {
    println!("Exercise 2.4: Fix unsafe trait implementations");
    
    // FIXME: This struct needs Send/Sync implementation for threading
    struct MyPointer {
        ptr: *mut i32,
        len: usize,
    }
    
    // FIXME: These trait implementations are missing 'unsafe'
    impl Send for MyPointer {}  // COMPILE ERROR: Send is unsafe trait
    impl Sync for MyPointer {}  // COMPILE ERROR: Sync is unsafe trait
    
    impl MyPointer {
        fn new(data: &mut [i32]) -> Self {
            Self {
                ptr: data.as_mut_ptr(),
                len: data.len(),
            }
        }
        
        // TODO: Add safe methods to work with the pointer
        fn get(&self, index: usize) -> Option<i32> {
            if index < self.len {
                unsafe {
                    // TODO: Safely read from pointer
                    todo!("Read from pointer at index")
                }
            } else {
                None
            }
        }
        
        fn set(&mut self, index: usize, value: i32) -> Result<(), &'static str> {
            // TODO: Safely write to pointer
            todo!("Write to pointer at index")
        }
    }
    
    // Test the implementation
    let mut data = vec![1, 2, 3, 4, 5];
    let mut ptr_wrapper = MyPointer::new(&mut data);
    
    println!("Value at index 2: {:?}", ptr_wrapper.get(2));
    
    if let Err(e) = ptr_wrapper.set(10, 42) {
        println!("Error setting value: {}", e);
    }
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Fix memory allocation errors
fn exercise_2_5() {
    println!("Exercise 2.5: Fix memory allocation");
    
    // FIXME: This custom allocator has multiple issues
    struct CustomBuffer {
        ptr: *mut u8,
        size: usize,
        capacity: usize,
    }
    
    impl CustomBuffer {
        fn new(capacity: usize) -> Self {
            // FIXME: This allocation code won't compile
            let layout = Layout::array::<u8>(capacity).unwrap();
            let ptr = alloc::alloc(layout);  // COMPILE ERROR: alloc is unsafe
            
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            
            CustomBuffer { ptr, size: 0, capacity }  // COMPILE ERROR: Missing Self
        }
        
        fn push(&mut self, byte: u8) -> Result<(), &'static str> {
            if self.size >= self.capacity {
                return Err("Buffer full");
            }
            
            // FIXME: Writing to raw pointer needs unsafe
            *self.ptr.add(self.size) = byte;  // COMPILE ERROR!
            self.size += 1;
            Ok(())
        }
        
        fn as_slice(&self) -> &[u8] {
            // FIXME: slice::from_raw_parts is unsafe
            slice::from_raw_parts(self.ptr, self.size)  // COMPILE ERROR!
        }
        
        // TODO: Implement proper cleanup
        fn clear(&mut self) {
            // TODO: Reset size to 0 (no deallocation needed)
            todo!("Reset buffer size")
        }
    }
    
    impl Drop for CustomBuffer {
        fn drop(&mut self) {
            if !self.ptr.is_null() && self.capacity > 0 {
                unsafe {
                    let layout = Layout::array::<u8>(self.capacity).unwrap();
                    // FIXME: dealloc is also unsafe
                    alloc::dealloc(self.ptr, layout);  // COMPILE ERROR!
                }
            }
        }
    }
    
    // Test the buffer
    let mut buffer = CustomBuffer::new(10);
    
    for i in 0..5 {
        buffer.push(b'A' + i).unwrap();
    }
    
    let data = buffer.as_slice();
    println!("Buffer contents: {:?}", std::str::from_utf8(data).unwrap());
    
    buffer.clear();
    println!("Buffer size after clear: {}", buffer.as_slice().len());
    
    println!("✅ Exercise 2.5 complete\n");
}

// Exercise 2.6: Fix slice operations and bounds checking
fn exercise_2_6() {
    println!("Exercise 2.6: Fix slice operations");
    
    // FIXME: This function has unsafe operations that need fixing
    fn create_slice_from_raw(data: &[u8], start: usize, len: usize) -> &[u8] {
        // TODO: Add bounds checking
        if start + len > data.len() {
            panic!("Slice bounds exceeded");
        }
        
        // FIXME: from_raw_parts is unsafe
        let ptr = data.as_ptr().add(start);
        slice::from_raw_parts(ptr, len)  // COMPILE ERROR!
    }
    
    // TODO: Create a safe version
    fn safe_slice_view(data: &[u8], start: usize, len: usize) -> Option<&[u8]> {
        // TODO: Use safe Rust slice operations instead of raw pointers
        if start + len <= data.len() {
            Some(&data[start..start + len])
        } else {
            None
        }
    }
    
    // FIXME: This function manipulates memory unsafely
    fn zero_memory_unsafe(data: &mut [u8]) {
        let ptr = data.as_mut_ptr();
        // TODO: Add unsafe block and proper implementation
        for i in 0..data.len() {
            *ptr.add(i) = 0;  // COMPILE ERROR: Unsafe pointer operation
        }
    }
    
    // TODO: Create safe version
    fn zero_memory_safe(data: &mut [u8]) {
        // TODO: Use safe iterator instead of raw pointers
        todo!("Zero memory safely")
    }
    
    // Test the functions
    let data = b"Hello, World!";
    
    if let Some(slice) = safe_slice_view(data, 7, 5) {
        println!("Safe slice: {:?}", std::str::from_utf8(slice).unwrap());
    }
    
    let mut buffer = vec![1, 2, 3, 4, 5];
    zero_memory_safe(&mut buffer);
    println!("Zeroed buffer: {:?}", buffer);
    
    println!("✅ Exercise 2.6 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_pointer_operations() {
        let mut x = 42;
        let ptr = &mut x as *mut i32;
        
        safe_write_through_pointer(ptr, 100);
        assert_eq!(safe_read_through_pointer(ptr), Some(100));
        
        // Test null pointer safety
        assert_eq!(safe_read_through_pointer(std::ptr::null()), None);
    }
    
    #[test]
    fn test_custom_buffer() {
        let mut buffer = CustomBuffer::new(5);
        
        for i in 0..5 {
            buffer.push(b'A' + i).unwrap();
        }
        
        assert_eq!(buffer.as_slice().len(), 5);
        assert!(buffer.push(b'F').is_err()); // Should be full
        
        buffer.clear();
        assert_eq!(buffer.as_slice().len(), 0);
    }
    
    #[test]
    fn test_safe_slice_operations() {
        let data = b"Hello, World!";
        
        // Valid slice
        assert!(safe_slice_view(data, 0, 5).is_some());
        
        // Invalid slice (out of bounds)
        assert!(safe_slice_view(data, 10, 10).is_none());
        
        // Test zero memory
        let mut buffer = vec![1, 2, 3, 4, 5];
        zero_memory_safe(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0]);
    }
}

// COMPILATION CHALLENGES:
// 1. Add unsafe blocks around raw pointer operations
// 2. Fix unsafe function calls by adding proper unsafe contexts
// 3. Implement unsafe trait implementations correctly
// 4. Add proper safety checks before unsafe operations
// 5. Use atomic types instead of raw global mutables
// 6. Implement safe wrappers around all unsafe code
//
// LEARNING OBJECTIVES:
// - Understand when and why unsafe is required
// - Learn to write safe abstractions over unsafe operations
// - Master pointer safety and null checking
// - Implement proper memory allocation and cleanup
// - Use atomic types for thread-safe global state
// - Debug unsafe code compilation errors systematically
//
// C# COMPARISON:
// C#: unsafe { int* p = &x; *p = 10; }           // Minimal restrictions
// Rust: unsafe { let p = &mut x as *mut i32; *p = 10; }  // More explicit control
//
// C#: static int counter;                         // Thread-unsafe by default
// Rust: static COUNTER: AtomicI32                // Thread-safe primitives
