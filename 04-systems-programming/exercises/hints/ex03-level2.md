# Exercise 03 - Level 2 Hints 🟡

## Specific Solutions for Manual Memory Management

### Checkpoint 1: Basic Allocation
```rust
// Problem: Allocation and deallocation are unsafe operations
let layout = Layout::new::<i32>();
let ptr = alloc::alloc(layout) as *mut i32;  // ERROR: unsafe!

// Fix: Wrap in unsafe block
unsafe {
    let layout = Layout::new::<i32>();
    let ptr = alloc::alloc(layout) as *mut i32;
    
    if !ptr.is_null() {
        *ptr = 42;
        println!("Value: {}", *ptr);
        alloc::dealloc(ptr as *mut u8, layout);
    }
}
```

### Checkpoint 2: Correct Layout Usage
```rust
// Problem: Using wrong layout for deallocation
let wrong_layout = Layout::new::<u32>();  // 4 bytes
alloc::dealloc(ptr as *mut u8, wrong_layout);  // But allocated u64 (8 bytes)!

// Fix: Use the same layout for alloc and dealloc
let layout = Layout::new::<u64>();
let ptr = alloc::alloc(layout) as *mut u64;
// ... use ptr ...
alloc::dealloc(ptr as *mut u8, layout);  // Same layout!
```

### Checkpoint 3: Prevent Memory Leaks
```rust
// Problem: Memory is allocated but never freed
unsafe {
    let ptr = alloc::alloc(layout) as *mut i32;
    // ... use ptr ...
    // MISSING: alloc::dealloc(ptr as *mut u8, layout);
}

// Fix: Always pair allocation with deallocation
unsafe {
    let ptr = alloc::alloc(layout) as *mut i32;
    if !ptr.is_null() {
        // ... use ptr ...
        alloc::dealloc(ptr as *mut u8, layout);  // Don't forget!
    }
}
```

### Checkpoint 5: RAII Implementation
```rust
impl Drop for ManagedMemory {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                alloc::dealloc(self.ptr, self.layout);
            }
        }
    }
}
```

Need complete implementations? Check Level 3 hints!