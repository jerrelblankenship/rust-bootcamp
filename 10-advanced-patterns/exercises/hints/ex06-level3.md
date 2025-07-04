# Exercise 06 - Level 3 Hints: Unsafe Undefined

## 🎯 Complete Solutions

Here are the fixes for each checkpoint:

## 🔧 Checkpoint 1: Raw Pointer Safety

```rust
unsafe fn process_raw_pointer(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() {
        None
    } else {
        Some(*ptr)
    }
}

unsafe fn modify_through_pointer(ptr: *mut i32, value: i32) -> bool {
    if ptr.is_null() {
        false
    } else {
        *ptr = value;
        true
    }
}
```

## 🔧 Checkpoint 2: Uninitialized Memory Fix

```rust
use std::mem::MaybeUninit;

unsafe fn create_uninitialized_array<T: Clone>(size: usize, default_value: T) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(default_value.clone());
    }
    vec
}

// Alternative: Using MaybeUninit for more control
unsafe fn create_array_with_uninit<T>(size: usize) -> Vec<MaybeUninit<T>> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(MaybeUninit::uninit());
    }
    vec
}

fn read_initialized() -> [i32; 10] {
    // Safe initialization
    [0; 10]
}
```

## 🔧 Checkpoint 3: Aliasing Fixes

```rust
// Fix aliasing by not creating overlapping mutable references
unsafe fn create_non_aliased_references(data: &mut [i32]) -> Option<(&mut i32, &mut i32)> {
    if data.len() < 2 {
        None
    } else {
        let ptr = data.as_mut_ptr();
        let ref1 = &mut *ptr;
        let ref2 = &mut *ptr.add(1);  // Point to different memory
        Some((ref1, ref2))
    }
}

unsafe fn safe_slice_split(slice: &mut [i32]) {
    if slice.len() >= 2 {
        let (first_half, second_half) = slice.split_at_mut(slice.len() / 2);
        first_half[0] = 42;
        second_half[0] = 24;  // Safe - no aliasing
    }
}
```

## 🔧 Checkpoint 4: Memory Management Fix

```rust
use std::alloc::{alloc, dealloc, Layout};

unsafe fn allocate_safely() -> *mut i32 {
    let layout = Layout::new::<i32>();
    let ptr = alloc(layout) as *mut i32;
    
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    
    *ptr = 42;
    ptr
}

unsafe fn proper_deallocation(ptr: *mut i32) {
    if !ptr.is_null() {
        let layout = Layout::new::<i32>();
        dealloc(ptr as *mut u8, layout);
    }
}

// RAII wrapper for safe memory management
struct SafeAlloc {
    ptr: *mut i32,
    layout: Layout,
}

impl SafeAlloc {
    unsafe fn new(value: i32) -> Self {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        *ptr = value;
        Self { ptr, layout }
    }
    
    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }
}

impl Drop for SafeAlloc {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                dealloc(self.ptr as *mut u8, self.layout);
            }
        }
    }
}
```

## 🔧 Checkpoint 5: Transmute Safety

```rust
// Fix transmute size mismatch
unsafe fn safe_transmute_example() {
    let x = 42u64;
    // Convert to array of bytes instead
    let bytes: [u8; 8] = std::mem::transmute(x);
    println!("Bytes: {:?}", bytes);
}

unsafe fn safe_slice_creation() {
    let numbers = [1u32, 2, 3, 4, 5];
    let ptr = numbers.as_ptr() as *const u8;
    let slice = slice::from_raw_parts(ptr, numbers.len() * 4);
    println!("Slice length: {}", slice.len());
}
```

## 🔧 Checkpoint 6: Lifetime Safety

```rust
// Fix lifetime extension
fn safe_string_processing(data: &str) -> String {
    // Don't extend lifetime, return owned data
    data.to_string()
}

fn return_owned_reference() -> String {
    let local = "temporary".to_string();
    local  // Return owned, not borrowed
}
```

## 🔧 Checkpoint 7: Unsafe Trait Implementation

```rust
unsafe trait UnsafeMarker {
    fn is_safe_to_use(&self) -> bool;
}

struct SafeStruct {
    ptr: *mut i32,
    valid: bool,
}

impl SafeStruct {
    unsafe fn new(value: i32) -> Self {
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        *ptr = value;
        Self { ptr, valid: true }
    }
}

unsafe impl UnsafeMarker for SafeStruct {
    fn is_safe_to_use(&self) -> bool {
        self.valid && !self.ptr.is_null()
    }
}

impl Drop for SafeStruct {
    fn drop(&mut self) {
        if self.valid && !self.ptr.is_null() {
            unsafe {
                let layout = Layout::new::<i32>();
                dealloc(self.ptr as *mut u8, layout);
            }
            self.valid = false;
        }
    }
}
```

## 🔧 Checkpoint 8: Safe Wrapper Implementation

```rust
fn safe_wrapper_fixed(data: &[i32], index: usize) -> Option<i32> {
    if index < data.len() {
        unsafe {
            let ptr = data.as_ptr().add(index);
            Some(*ptr)
        }
    } else {
        None
    }
}

fn safe_slice_creation_fixed(ptr: *const i32, len: usize) -> Option<&'static [i32]> {
    if ptr.is_null() || len == 0 {
        None
    } else {
        unsafe {
            // This is still unsafe - we can't guarantee the lifetime
            // In practice, you'd need additional validation
            Some(slice::from_raw_parts(ptr, len))
        }
    }
}

// Better: Use a lifetime parameter
unsafe fn create_slice_with_lifetime<'a>(ptr: *const i32, len: usize) -> Option<&'a [i32]> {
    if ptr.is_null() || len == 0 {
        None
    } else {
        Some(slice::from_raw_parts(ptr, len))
    }
}
```

## 🎮 Testing Your Solution

```rust
fn main() {
    // Test safe pointer usage
    let value = 42;
    unsafe {
        if let Some(result) = process_raw_pointer(&value) {
            println!("Safe pointer result: {}", result);
        }
    }
    
    // Test safe memory management
    unsafe {
        let ptr = allocate_safely();
        println!("Allocated value: {}", *ptr);
        proper_deallocation(ptr);
    }
    
    // Test RAII wrapper
    unsafe {
        let safe_alloc = SafeAlloc::new(100);
        println!("RAII value: {}", safe_alloc.get());
        // Automatically freed when dropped
    }
    
    // Test safe wrapper
    let data = [1, 2, 3, 4, 5];
    if let Some(value) = safe_wrapper_fixed(&data, 2) {
        println!("Safe wrapper result: {}", value);
    }
}
```

## 🔍 Key Takeaways

1. **Always validate pointers** - Check for null before dereferencing
2. **Use MaybeUninit** - Replace deprecated uninitialized
3. **Avoid aliasing** - Ensure exclusive mutable access
4. **Track memory state** - Prevent double free and use-after-free
5. **Create safe abstractions** - Wrap unsafe code in safe interfaces
6. **Use RAII patterns** - Automatic cleanup prevents leaks

## 🎯 Verification

Your code should now be free of undefined behavior. Use tools like Miri to verify:
```bash
cargo +nightly miri run
```

The unsafe code should maintain all safety invariants while providing the performance benefits of low-level operations!