# Lesson 01: Memory Layout and Representation

Welcome to systems programming with Rust! As a C# developer, you're used to the CLR managing memory layout for you. In this lesson, you'll learn how Rust represents data in memory and how to work safely at the systems level.

## 🎯 Learning Objectives

- Understand how Rust lays out data in memory
- Learn about stack vs heap allocation patterns
- Master safe interactions with raw memory
- Compare memory models between C# and Rust
- Work with memory-mapped files and zero-copy operations

## 🧠 Memory Layout Fundamentals

### C# vs Rust Memory Models

#### C# Memory Model
```csharp
// C# - Managed memory with GC
public class Person  // Reference type - allocated on heap
{
    public string Name { get; set; }  // Reference to heap-allocated string
    public int Age { get; set; }      // Value stored inline
}

public struct Point  // Value type - can be stack or heap allocated
{
    public int X { get; set; }
    public int Y { get; set; }
}

// Usage
var person = new Person { Name = "Alice", Age = 30 };  // Heap allocated
var point = new Point { X = 10, Y = 20 };              // Stack allocated
```

#### Rust Memory Model
```rust
// Rust - Explicit control over allocation
#[derive(Debug)]
struct Person {
    name: String,     // String data is heap-allocated, struct holds pointer
    age: u32,         // Stored inline in the struct
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Stack allocation (default)
let person = Person {
    name: String::from("Alice"),  // Heap allocation for string content
    age: 30,                      // Stack allocation
};

let point = Point { x: 10, y: 20 };  // Entirely stack allocated

// Explicit heap allocation
let boxed_person = Box::new(person);  // Person struct now on heap
```

## 📐 Understanding Data Layout

### Memory Layout Visualization

```rust
use std::mem;

fn explore_memory_layout() {
    println!("=== Memory Layout Analysis ===");
    
    // Basic types
    println!("Basic types:");
    println!("u8: {} bytes, align: {}", mem::size_of::<u8>(), mem::align_of::<u8>());
    println!("u32: {} bytes, align: {}", mem::size_of::<u32>(), mem::align_of::<u32>());
    println!("u64: {} bytes, align: {}", mem::size_of::<u64>(), mem::align_of::<u64>());
    println!("usize: {} bytes, align: {}", mem::size_of::<usize>(), mem::align_of::<usize>());
    
    // Strings and vectors
    println!("\nString types:");
    println!("String: {} bytes", mem::size_of::<String>());
    println!("&str: {} bytes", mem::size_of::<&str>());
    println!("Vec<u8>: {} bytes", mem::size_of::<Vec<u8>>());
    
    // Custom structs
    #[derive(Debug)]
    struct Compact {
        a: u8,
        b: u8,
        c: u16,
    }
    
    #[derive(Debug)]
    struct Padded {
        a: u8,
        b: u32,  // This will cause padding
        c: u8,
    }
    
    #[repr(packed)]
    #[derive(Debug)]
    struct Packed {
        a: u8,
        b: u32,
        c: u8,
    }
    
    println!("\nStruct layouts:");
    println!("Compact: {} bytes, align: {}", 
             mem::size_of::<Compact>(), mem::align_of::<Compact>());
    println!("Padded: {} bytes, align: {}", 
             mem::size_of::<Padded>(), mem::align_of::<Padded>());
    println!("Packed: {} bytes, align: {}", 
             mem::size_of::<Packed>(), mem::align_of::<Packed>());
}

fn analyze_struct_padding() {
    #[derive(Debug)]
    struct Example {
        a: u8,    // 1 byte
        // 3 bytes padding here for alignment
        b: u32,   // 4 bytes
        c: u16,   // 2 bytes
        // 2 bytes padding at end for struct alignment
    }
    
    let example = Example { a: 1, b: 2, c: 3 };
    
    // Print addresses to see padding
    unsafe {
        let base = &example as *const Example as usize;
        let a_addr = &example.a as *const u8 as usize;
        let b_addr = &example.b as *const u32 as usize;
        let c_addr = &example.c as *const u16 as usize;
        
        println!("Struct base address: 0x{:x}", base);
        println!("Field a offset: {}", a_addr - base);
        println!("Field b offset: {}", b_addr - base);
        println!("Field c offset: {}", c_addr - base);
        println!("Total struct size: {}", mem::size_of::<Example>());
    }
}
```

### Memory Layout Control

```rust
// Control layout with repr attributes
#[repr(C)]  // C-compatible layout
struct CLayout {
    x: u32,
    y: u16,
}

#[repr(packed)]  // No padding
struct PackedLayout {
    x: u32,
    y: u16,
}

#[repr(align(16))]  // Force 16-byte alignment
struct AlignedLayout {
    x: u32,
}

fn demonstrate_layout_control() {
    use std::mem;
    
    println!("Layout control:");
    println!("Normal: {} bytes, align: {}", 
             mem::size_of::<(u32, u16)>(), mem::align_of::<(u32, u16)>());
    println!("C layout: {} bytes, align: {}", 
             mem::size_of::<CLayout>(), mem::align_of::<CLayout>());
    println!("Packed: {} bytes, align: {}", 
             mem::size_of::<PackedLayout>(), mem::align_of::<PackedLayout>());
    println!("Aligned: {} bytes, align: {}", 
             mem::size_of::<AlignedLayout>(), mem::align_of::<AlignedLayout>());
}
```

## 🎯 Stack vs Heap Allocation

### Understanding Allocation Patterns

```rust
use std::alloc::{alloc, dealloc, Layout};

fn stack_vs_heap_demo() {
    // Stack allocation - automatic, fast, limited size
    let stack_array = [0u8; 1024];  // 1KB on stack
    println!("Stack array address: {:p}", &stack_array);
    
    // Heap allocation via Box - manual control
    let heap_array = Box::new([0u8; 1024]);  // 1KB on heap
    println!("Heap array address: {:p}", &*heap_array);
    
    // Large allocations should use heap
    let large_heap_data = vec![0u8; 1024 * 1024];  // 1MB on heap
    println!("Large data length: {}", large_heap_data.len());
    
    // Demonstrate ownership transfer
    let data = vec![1, 2, 3, 4, 5];
    let boxed_data = data.into_boxed_slice();  // Transfer to heap
    println!("Boxed data: {:?}", boxed_data);
}

// Custom allocator usage (advanced)
fn manual_allocation_example() {
    unsafe {
        // Allocate 100 bytes with 8-byte alignment
        let layout = Layout::from_size_align(100, 8).unwrap();
        let ptr = alloc(layout);
        
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        // Use the memory
        *ptr = 42;
        println!("Allocated memory at: {:p}, value: {}", ptr, *ptr);
        
        // Must deallocate manually
        dealloc(ptr, layout);
    }
}
```

### Memory-Mapped Files

```rust
use std::fs::File;
use std::io::{self, Write};

// Simulate memory mapping (real implementation would use mmap crate)
fn memory_mapped_file_demo() -> io::Result<()> {
    // Create a test file
    let mut file = File::create("test_data.bin")?;
    let test_data = (0..1000u32).collect::<Vec<_>>();
    
    // Write binary data
    for &num in &test_data {
        file.write_all(&num.to_le_bytes())?;
    }
    drop(file);
    
    // Read as memory-mapped (simplified version)
    let file_content = std::fs::read("test_data.bin")?;
    let numbers: &[u32] = unsafe {
        std::slice::from_raw_parts(
            file_content.as_ptr() as *const u32,
            file_content.len() / 4,
        )
    };
    
    println!("Memory-mapped numbers: {} count", numbers.len());
    println!("First 10: {:?}", &numbers[..10]);
    
    // Clean up
    std::fs::remove_file("test_data.bin")?;
    Ok(())
}
```

## 💡 Zero-Copy Operations

### Efficient Data Handling

```rust
use std::borrow::Cow;

// Zero-copy string operations
fn zero_copy_strings() {
    let data = "Hello, World! This is a test string.";
    
    // Borrow if no changes needed
    let processed = process_string(data);
    println!("Processed: {}", processed);
    
    // Only allocate if changes are needed
    let data_with_special = "Hello, World! This needs PROCESSING.";
    let processed_special = process_string(data_with_special);
    println!("Processed special: {}", processed_special);
}

fn process_string(input: &str) -> Cow<str> {
    if input.contains("PROCESSING") {
        // Need to modify - allocate new string
        Cow::Owned(input.replace("PROCESSING", "processing"))
    } else {
        // No changes needed - borrow original
        Cow::Borrowed(input)
    }
}

// Zero-copy slice operations
fn zero_copy_slices() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Create views without copying
    let first_half = &data[..5];
    let second_half = &data[5..];
    
    println!("Original: {:?}", data);
    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);
    
    // Process views independently
    let sum1: i32 = first_half.iter().sum();
    let sum2: i32 = second_half.iter().sum();
    
    println!("Sums: {} + {} = {}", sum1, sum2, sum1 + sum2);
}
```

### Buffer Management

```rust
// Efficient buffer reuse patterns
struct BufferPool {
    buffers: Vec<Vec<u8>>,
    buffer_size: usize,
}

impl BufferPool {
    fn new(buffer_size: usize, initial_count: usize) -> Self {
        let mut buffers = Vec::with_capacity(initial_count);
        for _ in 0..initial_count {
            buffers.push(Vec::with_capacity(buffer_size));
        }
        
        Self { buffers, buffer_size }
    }
    
    fn get_buffer(&mut self) -> Vec<u8> {
        self.buffers.pop().unwrap_or_else(|| {
            Vec::with_capacity(self.buffer_size)
        })
    }
    
    fn return_buffer(&mut self, mut buffer: Vec<u8>) {
        buffer.clear();
        if buffer.capacity() == self.buffer_size {
            self.buffers.push(buffer);
        }
    }
}

fn buffer_pool_demo() {
    let mut pool = BufferPool::new(1024, 5);
    
    // Get buffer from pool
    let mut buffer = pool.get_buffer();
    buffer.extend_from_slice(b"Hello, World!");
    
    println!("Buffer content: {:?}", String::from_utf8_lossy(&buffer));
    
    // Return to pool for reuse
    pool.return_buffer(buffer);
    
    // Get another buffer (might be the same memory)
    let buffer2 = pool.get_buffer();
    println!("Reused buffer capacity: {}", buffer2.capacity());
}
```

## 🔍 Memory Safety in Systems Programming

### Safe Abstractions Over Unsafe Code

```rust
// Safe wrapper around raw memory operations
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl SafeBuffer {
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        if ptr.is_null() {
            panic!("Allocation failed");
        }
        
        Self {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len >= self.capacity {
            return Err("Buffer full");
        }
        
        unsafe {
            *self.ptr.add(self.len) = byte;
        }
        self.len += 1;
        Ok(())
    }
    
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}

// Safe to send between threads (if needed)
unsafe impl Send for SafeBuffer {}

fn safe_buffer_demo() {
    let mut buffer = SafeBuffer::new(10);
    
    for i in 0..5 {
        buffer.push(b'A' + i).unwrap();
    }
    
    println!("Buffer contents: {:?}", 
             String::from_utf8_lossy(buffer.as_slice()));
    println!("Buffer length: {}", buffer.len());
}
```

### Memory Analysis Tools

```rust
// Memory usage analysis
fn analyze_memory_usage() {
    use std::mem;
    
    // Analyze different collection types
    let vec_empty: Vec<u32> = Vec::new();
    let vec_with_capacity: Vec<u32> = Vec::with_capacity(100);
    let vec_with_data: Vec<u32> = (0..100).collect();
    
    println!("Memory analysis:");
    println!("Empty Vec: {} bytes", mem::size_of_val(&vec_empty));
    println!("Vec with capacity: {} bytes + {} heap bytes", 
             mem::size_of_val(&vec_with_capacity),
             vec_with_capacity.capacity() * mem::size_of::<u32>());
    println!("Vec with data: {} bytes + {} heap bytes", 
             mem::size_of_val(&vec_with_data),
             vec_with_data.capacity() * mem::size_of::<u32>());
    
    // String analysis
    let string_empty = String::new();
    let string_with_capacity = String::with_capacity(100);
    let string_with_data = "Hello, World!".to_string();
    
    println!("\nString analysis:");
    println!("Empty String: {} bytes", mem::size_of_val(&string_empty));
    println!("String with capacity: {} bytes + {} heap bytes",
             mem::size_of_val(&string_with_capacity),
             string_with_capacity.capacity());
    println!("String with data: {} bytes + {} heap bytes",
             mem::size_of_val(&string_with_data),
             string_with_data.capacity());
}

// Demonstrate memory efficiency techniques
fn memory_efficiency_demo() {
    // Use Box<[T]> instead of Vec<T> when size is fixed
    let vec_data: Vec<u32> = (0..1000).collect();
    let boxed_slice: Box<[u32]> = vec_data.into_boxed_slice();
    
    println!("Boxed slice len: {}", boxed_slice.len());
    
    // Use Cow for potentially borrowed data
    let borrowed_str = process_maybe_owned("hello");
    let owned_str = process_maybe_owned("HELLO");
    
    println!("Borrowed: {:?}", borrowed_str);
    println!("Owned: {:?}", owned_str);
}

fn process_maybe_owned(input: &str) -> Cow<str> {
    if input.chars().any(|c| c.is_uppercase()) {
        Cow::Owned(input.to_lowercase())
    } else {
        Cow::Borrowed(input)
    }
}
```

## 🎯 Key Takeaways

1. **Explicit Control**: Rust gives you explicit control over memory layout and allocation
2. **Zero-Cost Abstractions**: High-level features don't add runtime overhead
3. **Memory Safety**: Safe abstractions prevent common memory errors
4. **Performance**: Understanding layout enables optimization opportunities
5. **Interoperability**: Control over layout enables C interop

## 💻 Exercises

### Exercise 1: Memory Layout Analysis
```rust
// TODO: Analyze the memory layout of these structs
// Predict the size and alignment before running

#[derive(Debug)]
struct TestStruct1 {
    a: u8,
    b: u32,
    c: u16,
}

#[derive(Debug)]
struct TestStruct2 {
    a: u8,
    b: u16,
    c: u32,
}

fn analyze_layouts() {
    // TODO: Print size, alignment, and field offsets
    // Experiment with different field orderings
    // Try #[repr(packed)] and #[repr(C)]
}
```

### Exercise 2: Buffer Pool Implementation
```rust
// TODO: Implement a thread-safe buffer pool
// Should efficiently reuse allocations
// Include proper error handling

struct ThreadSafeBufferPool {
    // TODO: Add fields
}

impl ThreadSafeBufferPool {
    fn new(buffer_size: usize, max_buffers: usize) -> Self {
        todo!()
    }
    
    fn get_buffer(&self) -> Option<Vec<u8>> {
        todo!()
    }
    
    fn return_buffer(&self, buffer: Vec<u8>) {
        todo!()
    }
}
```

### Exercise 3: Zero-Copy Parser
```rust
// TODO: Implement a zero-copy CSV parser
// Should return views into the original data without allocation

fn parse_csv_zero_copy(input: &str) -> Vec<Vec<&str>> {
    // TODO: Parse CSV without allocating new strings
    // Return references to parts of the input string
    todo!()
}
```

## 📚 Additional Resources

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - The Dark Arts of Unsafe Rust
- [Rust Reference - Type Layout](https://doc.rust-lang.org/reference/type-layout.html)
- [std::mem documentation](https://doc.rust-lang.org/std/mem/)

---

Next: [Unsafe Rust](02-unsafe-rust.md) →
