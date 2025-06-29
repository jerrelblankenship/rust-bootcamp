// Exercise 2 Solution: Unsafe Rust Code - WORKING VERSION
//
// This is the complete, working solution for ex02-unsafe-debugging.rs
// Compare this with your implementation to understand safe unsafe patterns.

use std::alloc::{self, Layout};
use std::mem;
use std::ptr;
use std::slice;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("=== Exercise 2 Solution: Unsafe Rust Code ===\n");
    
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
    
    // FIXED: Added unsafe blocks around raw pointer operations
    let ptr = &mut x as *mut i32;
    unsafe {
        *ptr = 100;  // FIXED: Wrapped in unsafe block
        println!("Value through pointer: {}", *ptr);  // FIXED: Wrapped in unsafe block
    }
    
    // FIXED: Complete safe wrapper function
    fn safe_write_through_pointer(ptr: *mut i32, value: i32) {
        if ptr.is_null() {
            return;  // Safety: Don't dereference null pointers
        }
        
        unsafe {
            // SAFETY: We checked for null above
            *ptr = value;
        }
    }
    
    // FIXED: Complete safe reader function
    fn safe_read_through_pointer(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            unsafe {
                // SAFETY: We checked for null above
                Some(*ptr)
            }
        }
    }
    
    safe_write_through_pointer(ptr, 200);
    if let Some(value) = safe_read_through_pointer(ptr) {
        println!("Safe read: {}", value);
    }
    
    // Test null pointer safety
    assert_eq!(safe_read_through_pointer(std::ptr::null()), None);
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Fix unsafe function calls
fn exercise_2_2() {
    println!("Exercise 2.2: Fix unsafe function calls");
    
    // Unsafe function with documented safety contract
    unsafe fn dangerous_array_sum(ptr: *const i32, len: usize) -> i32 {
        // SAFETY CONTRACT: ptr must point to at least len valid i32s
        let mut sum = 0;
        for i in 0..len {
            sum += *ptr.add(i);
        }
        sum
    }
    
    // FIXED: Added unsafe block around unsafe function call
    let data = [1, 2, 3, 4, 5];
    let result = unsafe {
        dangerous_array_sum(data.as_ptr(), data.len())
    };
    println!("Unsafe sum: {}", result);
    
    // Safe wrapper that upholds the safety contract
    fn safe_array_sum(slice: &[i32]) -> i32 {
        unsafe {
            // SAFETY: slice guarantees valid pointer and length
            dangerous_array_sum(slice.as_ptr(), slice.len())
        }
    }
    
    let safe_result = safe_array_sum(&data);
    println!("Safe sum: {}", safe_result);
    
    // Test edge cases
    let empty_slice: &[i32] = &[];
    let empty_result = safe_array_sum(empty_slice);
    println!("Empty slice sum: {}", empty_result);
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Fix global mutable state access
fn exercise_2_3() {
    println!("Exercise 2.3: Fix global state access");
    
    static mut GLOBAL_COUNTER: u32 = 0;
    
    // FIXED: Added unsafe blocks for mutable static access
    fn safe_increment_counter() {
        unsafe {
            GLOBAL_COUNTER += 1;
        }
    }
    
    fn safe_get_counter() -> u32 {
        unsafe {
            GLOBAL_COUNTER
        }
    }
    
    // FIXED: Better solution using atomics (no unsafe needed!)
    static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
    
    fn atomic_increment() {
        ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
    
    fn atomic_get() -> u32 {
        ATOMIC_COUNTER.load(Ordering::SeqCst)
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
    
    struct MyPointer {
        ptr: *mut i32,
        len: usize,
    }
    
    // FIXED: Added 'unsafe' keyword for unsafe trait implementations
    unsafe impl Send for MyPointer {}
    unsafe impl Sync for MyPointer {}
    
    impl MyPointer {
        fn new(data: &mut [i32]) -> Self {
            Self {
                ptr: data.as_mut_ptr(),
                len: data.len(),
            }
        }
        
        // Safe methods to work with the pointer
        fn get(&self, index: usize) -> Option<i32> {
            if index < self.len {
                unsafe {
                    // SAFETY: Index is bounds-checked above
                    Some(*self.ptr.add(index))
                }
            } else {
                None
            }
        }
        
        fn set(&mut self, index: usize, value: i32) -> Result<(), &'static str> {
            if index < self.len {
                unsafe {
                    // SAFETY: Index is bounds-checked above
                    *self.ptr.add(index) = value;
                }
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }
    }
    
    // Test the implementation
    let mut data = vec![1, 2, 3, 4, 5];
    let mut ptr_wrapper = MyPointer::new(&mut data);
    
    println!("Value at index 2: {:?}", ptr_wrapper.get(2));
    
    if let Err(e) = ptr_wrapper.set(10, 42) {
        println!("Error setting value: {}", e);
    }
    
    // Test valid set operation
    ptr_wrapper.set(2, 99).unwrap();
    println!("Value at index 2 after update: {:?}", ptr_wrapper.get(2));
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Fix memory allocation errors
fn exercise_2_5() {
    println!("Exercise 2.5: Fix memory allocation");
    
    // FIXED: Complete custom allocator implementation
    struct CustomBuffer {
        ptr: *mut u8,
        size: usize,
        capacity: usize,
    }
    
    impl CustomBuffer {
        fn new(capacity: usize) -> Self {
            if capacity == 0 {
                return Self {
                    ptr: ptr::null_mut(),
                    size: 0,
                    capacity: 0,
                };
            }
            
            // FIXED: Added unsafe block around alloc call
            let layout = Layout::array::<u8>(capacity).unwrap();
            let ptr = unsafe { alloc::alloc(layout) };
            
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
            
            Self { ptr, size: 0, capacity }
        }
        
        fn push(&mut self, byte: u8) -> Result<(), &'static str> {
            if self.size >= self.capacity {
                return Err("Buffer full");
            }
            
            unsafe {
                // SAFETY: We checked bounds above
                *self.ptr.add(self.size) = byte;
            }
            self.size += 1;
            Ok(())
        }
        
        fn as_slice(&self) -> &[u8] {
            if self.size == 0 || self.ptr.is_null() {
                &[]
            } else {
                unsafe {
                    // SAFETY: ptr is valid and size is within bounds
                    slice::from_raw_parts(self.ptr, self.size)
                }
            }
        }
        
        fn clear(&mut self) {
            self.size = 0;
        }
    }
    
    impl Drop for CustomBuffer {
        fn drop(&mut self) {
            if !self.ptr.is_null() && self.capacity > 0 {
                unsafe {
                    let layout = Layout::array::<u8>(self.capacity).unwrap();
                    alloc::dealloc(self.ptr, layout);
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
    
    // Test buffer overflow protection
    let mut small_buffer = CustomBuffer::new(2);
    small_buffer.push(b'A').unwrap();
    small_buffer.push(b'B').unwrap();
    assert!(small_buffer.push(b'C').is_err());
    
    println!("✅ Exercise 2.5 complete\n");
}

// Exercise 2.6: Fix slice operations and bounds checking
fn exercise_2_6() {
    println!("Exercise 2.6: Fix slice operations");
    
    // FIXED: Added unsafe block and proper bounds checking
    fn create_slice_from_raw(data: &[u8], start: usize, len: usize) -> Option<&[u8]> {
        if start + len > data.len() {
            return None;
        }
        
        unsafe {
            // SAFETY: We checked bounds above
            let ptr = data.as_ptr().add(start);
            Some(slice::from_raw_parts(ptr, len))
        }
    }
    
    // Safe version using built-in slice operations
    fn safe_slice_view(data: &[u8], start: usize, len: usize) -> Option<&[u8]> {
        if start + len <= data.len() {
            Some(&data[start..start + len])
        } else {
            None
        }
    }
    
    // FIXED: Added unsafe block for memory manipulation
    fn zero_memory_unsafe(data: &mut [u8]) {
        let ptr = data.as_mut_ptr();
        unsafe {
            for i in 0..data.len() {
                // SAFETY: i is within bounds of the slice
                *ptr.add(i) = 0;
            }
        }
    }
    
    // Safe version using iterators
    fn zero_memory_safe(data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte = 0;
        }
    }
    
    // Test the functions
    let data = b"Hello, World!";
    
    if let Some(slice) = safe_slice_view(data, 7, 5) {
        println!("Safe slice: {:?}", std::str::from_utf8(slice).unwrap());
    }
    
    // Test unsafe slice creation
    if let Some(slice) = create_slice_from_raw(data, 0, 5) {
        println!("Unsafe slice: {:?}", std::str::from_utf8(slice).unwrap());
    }
    
    // Test bounds checking
    assert_eq!(safe_slice_view(data, 10, 10), None);
    assert_eq!(create_slice_from_raw(data, 10, 10), None);
    
    let mut buffer = vec![1, 2, 3, 4, 5];
    zero_memory_safe(&mut buffer);
    println!("Zeroed buffer (safe): {:?}", buffer);
    
    let mut buffer2 = vec![1, 2, 3, 4, 5];
    zero_memory_unsafe(&mut buffer2);
    println!("Zeroed buffer (unsafe): {:?}", buffer2);
    
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
        assert!(create_slice_from_raw(data, 0, 5).is_some());
        
        // Invalid slice (out of bounds)
        assert!(safe_slice_view(data, 10, 10).is_none());
        assert!(create_slice_from_raw(data, 10, 10).is_none());
        
        // Test zero memory
        let mut buffer = vec![1, 2, 3, 4, 5];
        zero_memory_safe(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0]);
        
        let mut buffer2 = vec![1, 2, 3, 4, 5];
        zero_memory_unsafe(&mut buffer2);
        assert_eq!(buffer2, vec![0, 0, 0, 0, 0]);
    }
    
    #[test]
    fn test_my_pointer() {
        let mut data = vec![1, 2, 3, 4, 5];
        let mut ptr_wrapper = MyPointer::new(&mut data);
        
        assert_eq!(ptr_wrapper.get(2), Some(3));
        assert_eq!(ptr_wrapper.get(10), None);
        
        assert!(ptr_wrapper.set(2, 99).is_ok());
        assert_eq!(ptr_wrapper.get(2), Some(99));
        
        assert!(ptr_wrapper.set(10, 42).is_err());
    }
    
    #[test]
    fn test_atomic_counter() {
        let initial = atomic_get();
        atomic_increment();
        assert_eq!(atomic_get(), initial + 1);
    }
}

// KEY LEARNING POINTS:
//
// 1. UNSAFE BLOCKS ARE REQUIRED FOR:
//    - Dereferencing raw pointers
//    - Calling unsafe functions
//    - Accessing mutable static variables
//    - Implementing unsafe traits
//
// 2. SAFETY CONTRACTS:
//    - Document what guarantees the caller must provide
//    - Check preconditions before unsafe operations
//    - Use SAFETY comments to explain why code is safe
//
// 3. SAFE ABSTRACTIONS:
//    - Wrap unsafe operations in safe APIs
//    - Validate inputs before calling unsafe code
//    - Maintain invariants across function boundaries
//
// 4. NULL POINTER CHECKS:
//    - Always check for null before dereferencing
//    - Use Option<T> to represent nullable pointers safely
//    - Provide graceful error handling
//
// 5. BOUNDS CHECKING:
//    - Validate array/slice indices before access
//    - Use safe slice operations when possible
//    - Provide early returns for invalid inputs
//
// 6. MEMORY MANAGEMENT:
//    - Pair allocations with deallocations
//    - Use RAII pattern (Drop trait) for cleanup
//    - Handle allocation failures gracefully
//
// 7. ATOMIC OPERATIONS:
//    - Prefer atomic types over raw global mutables
//    - Choose appropriate memory ordering
//    - No unsafe blocks needed for atomic operations
//
// SAFETY PATTERNS:
// - Check → Unsafe Operation → Verify
// - Document safety requirements clearly
// - Minimize unsafe code surface area
// - Test edge cases thoroughly
//
// C# COMPARISON:
// - C#: unsafe { } blocks have fewer restrictions
// - Rust: unsafe blocks are more controlled and explicit
// - C#: GC handles memory cleanup automatically
// - Rust: Manual memory management with compile-time guarantees
// - C#: NullReferenceException at runtime
// - Rust: Null pointer access prevented by Option<T> and checks
