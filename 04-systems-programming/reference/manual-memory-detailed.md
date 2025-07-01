# Manual Memory Management in Rust

## 🎯 Supporting Exercise: ex03-manual-memory.rs

This reference guide covers manual memory allocation and deallocation patterns you'll need for exercise 03.

## 🧠 Memory Management Fundamentals

### Stack vs Heap (Rust vs C#)

| Memory Type | Rust | C# |
|-------------|------|-----|
| **Stack** | Automatic, fast, limited size | Automatic, fast, limited size |
| **Heap** | Manual via `Box`, `Vec`, etc. | Automatic via GC |
| **Cleanup** | Deterministic (RAII) | Non-deterministic (GC) |

### Why Manual Memory Management?
- **Performance**: No GC pauses, predictable timing
- **Control**: Precise memory layout for systems programming
- **Interop**: C library compatibility
- **Embedded**: Resource-constrained environments

## 🔧 The `std::alloc` Module

### Basic Allocation API
```rust
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

unsafe fn allocate_buffer(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).unwrap();
    let ptr = alloc(layout);
    
    if ptr.is_null() {
        panic!("Allocation failed");
    }
    
    ptr
}

unsafe fn deallocate_buffer(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 1).unwrap();
    dealloc(ptr, layout);
}
```

### Memory Layout Requirements
```rust
use std::alloc::Layout;

// Single value allocation
let layout = Layout::new::<u64>(); // 8 bytes, 8-byte aligned

// Array allocation
let layout = Layout::array::<u32>(10).unwrap(); // 40 bytes, 4-byte aligned

// Custom alignment
let layout = Layout::from_size_align(1024, 64).unwrap(); // 1KB, 64-byte aligned
```

## 🏗️ Building Safe Abstractions

### RAII Pattern Implementation
```rust
struct ManagedBuffer {
    ptr: *mut u8,
    size: usize,
    layout: Layout,
}

impl ManagedBuffer {
    fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 1).unwrap();
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        Self { ptr, size, layout }
    }
    
    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}

impl Drop for ManagedBuffer {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr, self.layout);
        }
    }
}
```

### Custom Allocator Implementation
```rust
use std::alloc::{GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator {
    allocated: AtomicUsize,
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = std::alloc::System.alloc(layout);
        if !ptr.is_null() {
            self.allocated.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        std::alloc::System.dealloc(ptr, layout);
        self.allocated.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator {
    allocated: AtomicUsize::new(0),
};
```

## 🔄 Memory Pool Patterns

### Simple Pool Implementation
```rust
struct MemoryPool {
    blocks: Vec<*mut u8>,
    block_size: usize,
    layout: Layout,
}

impl MemoryPool {
    fn new(block_size: usize, initial_blocks: usize) -> Self {
        let layout = Layout::from_size_align(block_size, 8).unwrap();
        let mut blocks = Vec::new();
        
        for _ in 0..initial_blocks {
            unsafe {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    blocks.push(ptr);
                }
            }
        }
        
        Self { blocks, block_size, layout }
    }
    
    fn allocate(&mut self) -> Option<*mut u8> {
        self.blocks.pop().or_else(|| {
            // Allocate new block if pool is empty
            unsafe {
                let ptr = alloc(self.layout);
                if ptr.is_null() { None } else { Some(ptr) }
            }
        })
    }
    
    fn deallocate(&mut self, ptr: *mut u8) {
        // Return to pool instead of freeing
        self.blocks.push(ptr);
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        // Free all remaining blocks
        for ptr in self.blocks.drain(..) {
            unsafe {
                dealloc(ptr, self.layout);
            }
        }
    }
}
```

## 🛡️ Memory Safety Patterns

### MaybeUninit for Safe Initialization
```rust
use std::mem::MaybeUninit;

// Safe way to work with uninitialized memory
fn create_initialized_array<T: Clone>(value: T, count: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(count);
    
    for _ in 0..count {
        vec.push(value.clone());
    }
    
    vec
}

// More efficient for large arrays
fn create_array_uninit<T: Copy>(value: T, count: usize) -> Vec<T> {
    let mut vec: Vec<MaybeUninit<T>> = Vec::with_capacity(count);
    
    for _ in 0..count {
        vec.push(MaybeUninit::new(value));
    }
    
    // Safe because we initialized all elements
    unsafe {
        std::mem::transmute(vec)
    }
}
```

### Reference Counting for Manual Management
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct RefCountedBuffer {
    data: *mut u8,
    size: usize,
    ref_count: AtomicUsize,
    layout: Layout,
}

impl RefCountedBuffer {
    fn new(size: usize) -> Arc<Self> {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let data = unsafe { alloc(layout) };
        
        if data.is_null() {
            panic!("Allocation failed");
        }
        
        Arc::new(Self {
            data,
            size,
            ref_count: AtomicUsize::new(1),
            layout,
        })
    }
    
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.size) }
    }
}

impl Drop for RefCountedBuffer {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data, self.layout);
        }
    }
}
```

## ⚠️ Common Pitfalls and Solutions

### 1. Memory Leaks
```rust
// BAD: Allocation without corresponding deallocation
unsafe fn leak_memory() {
    let layout = Layout::new::<u64>();
    let ptr = alloc(layout);
    // Missing: dealloc(ptr, layout);
}

// GOOD: RAII wrapper ensures cleanup
struct SafeBuffer(*mut u8, Layout);

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe { dealloc(self.0, self.1); }
    }
}
```

### 2. Use After Free
```rust
// BAD: Using pointer after deallocation
unsafe fn use_after_free() {
    let layout = Layout::new::<u64>();
    let ptr = alloc(layout);
    dealloc(ptr, layout);
    *(ptr as *mut u64) = 42; // UNDEFINED BEHAVIOR!
}

// GOOD: Nullify pointer after deallocation
unsafe fn safe_dealloc(ptr: &mut *mut u8, layout: Layout) {
    if !ptr.is_null() {
        dealloc(*ptr, layout);
        *ptr = std::ptr::null_mut();
    }
}
```

### 3. Double Free
```rust
// BAD: Deallocating the same pointer twice
unsafe fn double_free(ptr: *mut u8, layout: Layout) {
    dealloc(ptr, layout);
    dealloc(ptr, layout); // UNDEFINED BEHAVIOR!
}

// GOOD: Use wrapper that prevents double-free
struct OncePtr(*mut u8, Layout);

impl OncePtr {
    unsafe fn free(&mut self) {
        if !self.0.is_null() {
            dealloc(self.0, self.1);
            self.0 = std::ptr::null_mut();
        }
    }
}
```

## 🔍 Debugging Memory Issues

### Memory Leak Detection
```rust
#[cfg(debug_assertions)]
static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(debug_assertions)]
fn track_allocation(size: usize) {
    ALLOCATION_COUNT.fetch_add(size, Ordering::Relaxed);
    println!("Allocated {} bytes, total: {}", 
             size, ALLOCATION_COUNT.load(Ordering::Relaxed));
}

#[cfg(debug_assertions)]
fn track_deallocation(size: usize) {
    ALLOCATION_COUNT.fetch_sub(size, Ordering::Relaxed);
    println!("Deallocated {} bytes, total: {}", 
             size, ALLOCATION_COUNT.load(Ordering::Relaxed));
}
```

### Valgrind Integration
```rust
// Add to Cargo.toml for debugging builds
#[cfg(feature = "valgrind")]
extern crate valgrind_request;

#[cfg(feature = "valgrind")]
fn mark_memory_undefined(ptr: *mut u8, len: usize) {
    unsafe {
        valgrind_request::make_mem_undefined(ptr, len);
    }
}
```

## 🚀 Performance Optimization

### Alignment for SIMD
```rust
// Allocate 32-byte aligned buffer for AVX operations
unsafe fn allocate_simd_buffer(elements: usize) -> *mut f32 {
    let size = elements * std::mem::size_of::<f32>();
    let layout = Layout::from_size_align(size, 32).unwrap();
    alloc(layout) as *mut f32
}
```

### Zero-Copy Patterns
```rust
// Share memory between threads without copying
use std::sync::Arc;

struct SharedBuffer {
    data: Arc<Vec<u8>>,
}

impl SharedBuffer {
    fn new(size: usize) -> Self {
        Self {
            data: Arc::new(vec![0; size]),
        }
    }
    
    fn clone_handle(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
```

## 💡 Tips for C# Developers

### Key Differences from C# Memory Management
1. **No Garbage Collector** - Manual cleanup required
2. **Deterministic Destruction** - Objects cleaned up immediately when dropped
3. **Stack vs Heap Control** - You choose where objects live
4. **No Memory Pressure** - No GC pressure or pause times

### Migration Strategies
1. **Start with RAII** - Wrap raw allocations in safe types
2. **Use existing abstractions** - `Box`, `Vec`, `String` handle most cases
3. **Pool frequently allocated objects** - Reduce allocation overhead
4. **Profile memory usage** - Use tools like `heaptrack` or `massif`

## 📚 Related Topics
- See `unsafe-rust-detailed.md` for unsafe code patterns
- See `memory-layout-detailed.md` for memory representation
- Exercise ex03-manual-memory.rs applies these concepts