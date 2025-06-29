# Lesson 02: Unsafe Rust - When Safety Guarantees Need to be Relaxed

Coming from C#, you understand that sometimes you need to step outside the managed safety guarantees for performance or interoperability. Rust's `unsafe` keyword provides similar escape hatches while maintaining overall memory safety.

## 🎯 Learning Objectives

- Understand when and why to use unsafe Rust
- Master the four unsafe superpowers
- Learn to write safe abstractions over unsafe code
- Compare with C#'s unsafe contexts
- Build low-level, high-performance code safely

## ⚠️ Understanding "Unsafe"

### What Unsafe Doesn't Mean

```rust
// MISCONCEPTION: unsafe means "anything goes"
// REALITY: unsafe means "I take responsibility for upholding safety invariants"

unsafe {
    // This is still checked by the compiler for basic correctness
    // let x: i32 = "hello";  // ❌ Still a compile error!
    
    // Only specific operations are unlocked
    let ptr = 0x1234 as *const i32;
    let value = *ptr;  // ✅ This dereference is now allowed
}
```

### C# Unsafe vs Rust Unsafe

#### C# Unsafe Context
```csharp
unsafe 
{
    int x = 42;
    int* ptr = &x;      // Take address
    *ptr = 100;         // Dereference pointer
    
    // Can do pointer arithmetic
    ptr++;
    *ptr = 200;         // Potential memory corruption!
}
```

#### Rust Unsafe Block
```rust
unsafe {
    let mut x = 42;
    let ptr = &mut x as *mut i32;  // Convert reference to raw pointer
    *ptr = 100;                    // Dereference raw pointer
    
    // Pointer arithmetic requires offset method
    // let ptr2 = ptr.offset(1);   // More explicit than C#
    // *ptr2 = 200;                // Would be UB without proper bounds
}
```

## 🔓 The Four Unsafe Superpowers

### 1. Dereferencing Raw Pointers

```rust
use std::ptr;

fn raw_pointer_examples() {
    // Creating raw pointers is safe
    let mut x = 42;
    let raw_ptr = &mut x as *mut i32;
    let const_ptr = &x as *const i32;
    
    // Dereferencing requires unsafe
    unsafe {
        println!("Value through raw pointer: {}", *raw_ptr);
        *raw_ptr = 100;
        println!("Modified value: {}", *const_ptr);
    }
    
    // Null pointer handling
    let null_ptr = ptr::null::<i32>();
    
    unsafe {
        // Always check for null!
        if !null_ptr.is_null() {
            println!("Value: {}", *null_ptr);
        } else {
            println!("Null pointer detected");
        }
    }
}

// Safe wrapper around unsafe operations
struct SafeArray<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SafeArray<T> {
    fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    fn with_capacity(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        Self { ptr, len: 0, capacity }
    }
    
    fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.len >= self.capacity {
            return Err("Array is full");
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
        Ok(())
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
}

impl<T> Drop for SafeArray<T> {
    fn drop(&mut self) {
        // Clean up all elements
        for i in 0..self.len {
            unsafe {
                ptr::drop_in_place(self.ptr.add(i));
            }
        }
        
        // Deallocate memory
        if !self.ptr.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

### 2. Calling Unsafe Functions

```rust
// Unsafe function definition
unsafe fn dangerous_operation(ptr: *mut i32, len: usize) -> i32 {
    // Contract: ptr must point to at least len valid i32s
    let mut sum = 0;
    for i in 0..len {
        sum += *ptr.add(i);  // Unsafe: dereference raw pointer
    }
    sum
}

// Safe wrapper that upholds the contract
fn safe_sum_array(slice: &[i32]) -> i32 {
    unsafe {
        // Safe because slice guarantees valid pointer and length
        dangerous_operation(slice.as_ptr() as *mut i32, slice.len())
    }
}

// External function declarations (FFI)
extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

fn use_external_functions() {
    unsafe {
        let ptr = malloc(100);
        if !ptr.is_null() {
            // Use the allocated memory
            *ptr = 42;
            println!("Allocated and wrote to C malloc'd memory");
            free(ptr);
        }
    }
}

// Intrinsics for CPU-specific operations
use std::arch::x86_64::*;

#[target_feature(enable = "sse2")]
unsafe fn simd_operations() {
    // SIMD operations for performance
    let a = _mm_set1_epi32(1);
    let b = _mm_set1_epi32(2);
    let result = _mm_add_epi32(a, b);
    
    let mut output = [0i32; 4];
    _mm_store_si128(output.as_mut_ptr() as *mut __m128i, result);
    
    println!("SIMD result: {:?}", output);
}
```

### 3. Accessing Mutable Static Variables

```rust
// Global mutable state requires unsafe to access
static mut GLOBAL_COUNTER: u32 = 0;

fn increment_counter() {
    unsafe {
        GLOBAL_COUNTER += 1;
    }
}

fn get_counter() -> u32 {
    unsafe { GLOBAL_COUNTER }
}

// Better: Use atomic types for thread-safe global state
use std::sync::atomic::{AtomicU32, Ordering};

static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);

fn safe_increment_counter() {
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn safe_get_counter() -> u32 {
    ATOMIC_COUNTER.load(Ordering::SeqCst)
}

// Lazy initialization pattern
use std::sync::Once;

static INIT: Once = Once::new();
static mut EXPENSIVE_RESOURCE: Option<ExpensiveResource> = None;

struct ExpensiveResource {
    data: Vec<u8>,
}

impl ExpensiveResource {
    fn new() -> Self {
        println!("Creating expensive resource...");
        Self {
            data: vec![0; 1024 * 1024], // 1MB allocation
        }
    }
}

fn get_expensive_resource() -> &'static ExpensiveResource {
    unsafe {
        INIT.call_once(|| {
            EXPENSIVE_RESOURCE = Some(ExpensiveResource::new());
        });
        EXPENSIVE_RESOURCE.as_ref().unwrap()
    }
}
```

### 4. Implementing Unsafe Traits

```rust
// Send and Sync are unsafe traits
struct MyPointer(*mut u8);

// We must guarantee this type can be safely sent between threads
unsafe impl Send for MyPointer {}

// We must guarantee this type can be safely shared between threads
unsafe impl Sync for MyPointer {}

// Custom drop implementation for complex cleanup
struct ResourceHandle {
    handle: *mut std::ffi::c_void,
}

impl ResourceHandle {
    fn new() -> Self {
        // Simulate acquiring a system resource
        Self {
            handle: Box::into_raw(Box::new(42)) as *mut std::ffi::c_void,
        }
    }
}

impl Drop for ResourceHandle {
    fn drop(&mut self) {
        unsafe {
            // Convert back to Box to properly deallocate
            let _ = Box::from_raw(self.handle as *mut i32);
            println!("Resource cleaned up");
        }
    }
}

// Unsafe trait for types that can be initialized with all zeros
unsafe trait ZeroInit {
    fn zero_init() -> Self;
}

// Safe for POD types
unsafe impl ZeroInit for u32 {
    fn zero_init() -> Self {
        0
    }
}

unsafe impl ZeroInit for [u8; 32] {
    fn zero_init() -> Self {
        [0; 32]
    }
}

// NOT safe for types with drop glue or non-zero bit patterns
// unsafe impl ZeroInit for String { ... }  // ❌ Would be UB!
```

## 🛡️ Safety Patterns and Best Practices

### Safe Abstractions Over Unsafe Code

```rust
// Example: Safe vector implementation
pub struct SafeVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SafeVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }
        
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
        
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        
        Self { ptr, len: 0, capacity }
    }
    
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            std::ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(std::ptr::read(self.ptr.add(self.len)))
            }
        }
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        
        let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
        let new_ptr = unsafe { std::alloc::alloc(new_layout) as *mut T };
        
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }
        
        // Copy existing elements
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
        }
        
        // Deallocate old memory
        if !self.ptr.is_null() {
            unsafe {
                let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr as *mut u8, old_layout);
            }
        }
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for SafeVec<T> {
    fn drop(&mut self) {
        // Drop all elements
        while let Some(_) = self.pop() {}
        
        // Deallocate memory
        if !self.ptr.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

// Iterator implementation
impl<T> IntoIterator for SafeVec<T> {
    type Item = T;
    type IntoIter = SafeVecIntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        SafeVecIntoIter {
            vec: self,
            index: 0,
        }
    }
}

pub struct SafeVecIntoIter<T> {
    vec: SafeVec<T>,
    index: usize,
}

impl<T> Iterator for SafeVecIntoIter<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = unsafe {
                std::ptr::read(self.vec.ptr.add(self.index))
            };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

### Memory Safety Invariants

```rust
// Demonstrate safety invariants and how to maintain them
use std::slice;

// SAFETY INVARIANT: ptr must point to at least len valid T values
unsafe fn slice_from_raw_parts_demo<T>(ptr: *const T, len: usize) -> &'static [T] {
    // The caller must guarantee:
    // 1. ptr is valid for reads of len * size_of::<T>() bytes
    // 2. ptr is properly aligned for T
    // 3. The memory referenced must not be mutated for the returned lifetime
    // 4. The total size must not overflow isize::MAX
    
    slice::from_raw_parts(ptr, len)
}

// Safe wrapper that upholds invariants
fn safe_slice_view<T>(data: &[T], start: usize, len: usize) -> Option<&[T]> {
    if start + len <= data.len() {
        unsafe {
            // SAFETY: We've verified bounds, ptr comes from valid slice
            Some(slice_from_raw_parts_demo(data.as_ptr().add(start), len))
        }
    } else {
        None
    }
}

// Demonstrate aliasing rules
fn aliasing_example() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // This is safe - immutable references can alias
    let slice1 = &data[0..2];
    let slice2 = &data[2..4];
    println!("Non-overlapping slices: {:?}, {:?}", slice1, slice2);
    
    // This would be UB if done with raw pointers unsafely:
    // let ptr = data.as_mut_ptr();
    // let slice1 = slice::from_raw_parts_mut(ptr, 3);
    // let slice2 = slice::from_raw_parts_mut(ptr.add(2), 3); // OVERLAPPING!
    
    // Safe way to get mutable non-overlapping slices
    let (left, right) = data.split_at_mut(2);
    println!("Split mutable slices: {:?}, {:?}", left, right);
}
```

## 🔗 Interfacing with C Code

### Foreign Function Interface (FFI)

```rust
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

// Declare external C functions
extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
}

// Safe Rust wrapper for C string operations
fn safe_strlen(s: &str) -> usize {
    let c_string = CString::new(s).unwrap();
    unsafe {
        strlen(c_string.as_ptr())
    }
}

fn safe_string_copy(s: &str) -> String {
    let c_string = CString::new(s).unwrap();
    
    unsafe {
        // Allocate memory for copy
        let len = strlen(c_string.as_ptr()) + 1; // +1 for null terminator
        let dest = malloc(len) as *mut c_char;
        
        if dest.is_null() {
            panic!("Memory allocation failed");
        }
        
        // Copy the string
        strcpy(dest, c_string.as_ptr());
        
        // Convert back to Rust string
        let result = CStr::from_ptr(dest).to_string_lossy().into_owned();
        
        // Free the C memory
        free(dest as *mut std::ffi::c_void);
        
        result
    }
}

// Export Rust functions to C
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_process_array(ptr: *mut c_int, len: usize) -> c_int {
    if ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr, len);
        let sum: i32 = slice.iter().sum();
        
        // Double each element
        for element in slice {
            *element *= 2;
        }
        
        sum
    }
}
```

## 🎯 Performance Optimizations with Unsafe

### Zero-Copy String Processing

```rust
use std::str;

// Unsafe but fast string splitting
fn fast_split_whitespace(input: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let bytes = input.as_bytes();
    let mut start = 0;
    
    for i in 0..bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            if start < i {
                // SAFETY: We know start..i is within bounds and on char boundaries
                unsafe {
                    let slice = str::from_utf8_unchecked(&bytes[start..i]);
                    result.push(slice);
                }
            }
            start = i + 1;
        }
    }
    
    // Handle final token
    if start < bytes.len() {
        unsafe {
            let slice = str::from_utf8_unchecked(&bytes[start..]);
            result.push(slice);
        }
    }
    
    result
}

// SIMD-accelerated operations
#[cfg(target_arch = "x86_64")]
mod simd_ops {
    use std::arch::x86_64::*;
    
    // Vectorized array sum (requires SSE2)
    #[target_feature(enable = "sse2")]
    pub unsafe fn sum_i32_simd(slice: &[i32]) -> i32 {
        let mut sum = _mm_setzero_si128();
        let chunks = slice.chunks_exact(4);
        let remainder = chunks.remainder();
        
        for chunk in chunks {
            let values = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
            sum = _mm_add_epi32(sum, values);
        }
        
        // Extract results from SIMD register
        let mut result = [0i32; 4];
        _mm_storeu_si128(result.as_mut_ptr() as *mut __m128i, sum);
        let simd_sum: i32 = result.iter().sum();
        
        // Add remainder
        let remainder_sum: i32 = remainder.iter().sum();
        
        simd_sum + remainder_sum
    }
}

pub fn sum_i32_fast(slice: &[i32]) -> i32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("sse2") {
            return unsafe { simd_ops::sum_i32_simd(slice) };
        }
    }
    
    // Fallback to standard sum
    slice.iter().sum()
}
```

## 🎯 Key Takeaways

1. **Minimal Usage**: Use unsafe sparingly and with clear documentation
2. **Safe Abstractions**: Wrap unsafe code in safe APIs
3. **Invariant Documentation**: Clearly document safety requirements
4. **Testing**: Extensive testing of unsafe code is critical
5. **Performance**: Unsafe enables optimizations impossible in safe Rust

## 💻 Exercises

### Exercise 1: Safe Vector Wrapper
```rust
// TODO: Complete the SafeVec implementation
// Add methods: insert, remove, extend_from_slice
// Ensure all invariants are maintained

impl<T> SafeVec<T> {
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        // TODO: Implement safe insertion
        todo!()
    }
    
    pub fn remove(&mut self, index: usize) -> Option<T> {
        // TODO: Implement safe removal
        todo!()
    }
}
```

### Exercise 2: Lock-Free Data Structure
```rust
// TODO: Implement a lock-free stack using atomic operations
use std::sync::atomic::{AtomicPtr, Ordering};

struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        todo!()
    }
    
    fn push(&self, data: T) {
        todo!()
    }
    
    fn pop(&self) -> Option<T> {
        todo!()
    }
}
```

### Exercise 3: Memory Pool Allocator
```rust
// TODO: Implement a simple memory pool for fixed-size allocations
struct MemoryPool {
    // TODO: Add fields for tracking free blocks
}

impl MemoryPool {
    fn new(block_size: usize, block_count: usize) -> Self {
        todo!()
    }
    
    fn allocate(&mut self) -> Option<*mut u8> {
        todo!()
    }
    
    fn deallocate(&mut self, ptr: *mut u8) {
        todo!()
    }
}
```

## 📚 Additional Resources

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced unsafe Rust guide
- [Miri](https://github.com/rust-lang/miri) - Rust interpreter for detecting UB
- [std::ptr documentation](https://doc.rust-lang.org/std/ptr/) - Raw pointer utilities

---

Next: [Foreign Function Interface](03-ffi.md) →
