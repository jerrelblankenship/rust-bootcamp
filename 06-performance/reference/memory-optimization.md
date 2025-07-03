# Memory Optimization

Allocation patterns, capacity management, and memory reuse strategies.

## 🎯 Memory Performance Fundamentals

### The Memory Hierarchy

Modern memory systems have multiple levels with vastly different performance characteristics:

```
CPU Registers:    ~1 cycle     (fastest)
L1 Cache:         ~3 cycles    (32KB typical)
L2 Cache:         ~12 cycles   (256KB typical)  
L3 Cache:         ~40 cycles   (8MB typical)
Main RAM:         ~100 cycles  (8GB+ typical)
SSD:              ~100,000 cycles
HDD:              ~10,000,000 cycles (slowest)
```

**Key Insight**: Cache misses are performance killers. Keep hot data close together!

### Allocation Cost Hierarchy

```rust
// ✅ Fastest: Stack allocation (essentially free)
fn stack_allocation() {
    let array = [0u8; 1024];  // Allocated in registers/stack
    process_data(&array);
}  // Automatically cleaned up

// ✅ Fast: Pre-allocated heap (one allocation)
fn preallocated_heap() {
    let mut vec = Vec::with_capacity(1024);  // Single allocation
    for i in 0..1024 {
        vec.push(i);  // No reallocations needed
    }
}

// ❌ Slow: Growing heap (multiple allocations)
fn growing_heap() {
    let mut vec = Vec::new();  // Starts with capacity 0
    for i in 0..1024 {
        vec.push(i);  // Triggers ~10 reallocations (1->2->4->8->...)
    }
}

// ❌ Slowest: Many small allocations
fn many_allocations() {
    let mut boxes = Vec::new();
    for i in 0..1024 {
        boxes.push(Box::new(i));  // 1024 separate heap allocations!
    }
}
```

## 🏗️ Allocation Patterns

### 1. Pre-allocation Strategy

```rust
// ❌ Bad: Letting Vec grow organically
fn bad_append(items: &[String]) -> Vec<String> {
    let mut result = Vec::new();  // Starts empty
    for item in items {
        result.push(item.clone());  // Multiple reallocations
    }
    result
}

// ✅ Good: Pre-allocate when size is known
fn good_append(items: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(items.len());  // Pre-allocate
    for item in items {
        result.push(item.clone());  // No reallocations
    }
    result
}

// ✅ Better: Use iterator methods when possible
fn best_append(items: &[String]) -> Vec<String> {
    items.iter().cloned().collect()  // Optimal allocation
}
```

### 2. Object Pooling

```rust
use std::collections::VecDeque;

struct ObjectPool<T> {
    objects: VecDeque<T>,
    factory: Box<dyn Fn() -> T>,
}

impl<T> ObjectPool<T> {
    fn new<F>(factory: F) -> Self 
    where 
        F: Fn() -> T + 'static 
    {
        Self {
            objects: VecDeque::new(),
            factory: Box::new(factory),
        }
    }
    
    fn get(&mut self) -> T {
        self.objects.pop_front()
            .unwrap_or_else(|| (self.factory)())
    }
    
    fn return_object(&mut self, obj: T) {
        self.objects.push_back(obj);
    }
}

// Usage
fn process_many_buffers() {
    let mut pool = ObjectPool::new(|| Vec::with_capacity(1024));
    
    for _ in 0..1000 {
        let mut buffer = pool.get();
        buffer.clear();  // Reset but keep capacity
        
        // Use buffer...
        fill_buffer(&mut buffer);
        process_buffer(&buffer);
        
        pool.return_object(buffer);  // Reuse instead of deallocating
    }
}
```

### 3. Arena Allocation

```rust
struct Arena {
    data: Vec<u8>,
    offset: usize,
}

impl Arena {
    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            offset: 0,
        }
    }
    
    fn allocate<T>(&mut self, value: T) -> &mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        
        // Align offset
        self.offset = (self.offset + align - 1) & !(align - 1);
        
        if self.offset + size > self.data.capacity() {
            panic!("Arena out of space");
        }
        
        unsafe {
            let ptr = self.data.as_mut_ptr().add(self.offset) as *mut T;
            ptr.write(value);
            self.offset += size;
            &mut *ptr
        }
    }
    
    fn reset(&mut self) {
        self.offset = 0;  // Reset without deallocating
    }
}

// Usage: Perfect for request/response cycles
fn handle_request() {
    static mut ARENA: Option<Arena> = None;
    
    let arena = unsafe {
        ARENA.get_or_insert_with(|| Arena::new(1024 * 1024))  // 1MB arena
    };
    
    arena.reset();  // Clear previous request data
    
    // Allocate temporary objects for this request
    let temp1 = arena.allocate(ExpensiveStruct::new());
    let temp2 = arena.allocate([0u8; 256]);
    
    // Process request...
    
    // All allocations automatically "freed" on next reset
}
```

## 🧠 Memory Layout Optimization

### Array of Structs vs Struct of Arrays

```rust
// ❌ Array of Structs (AoS) - Poor cache utilization
#[derive(Clone)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3], 
    mass: f32,
    active: bool,  // Poor packing due to alignment
}

fn update_aos(particles: &mut [Particle]) {
    for particle in particles {
        if particle.active {
            // Only need position/velocity, but load entire struct
            for i in 0..3 {
                particle.position[i] += particle.velocity[i];
            }
        }
    }
}

// ✅ Struct of Arrays (SoA) - Better cache utilization
struct ParticleSystem {
    positions: Vec<[f32; 3]>,
    velocities: Vec<[f32; 3]>,
    masses: Vec<f32>,
    active: Vec<bool>,
}

impl ParticleSystem {
    fn update(&mut self) {
        for i in 0..self.positions.len() {
            if self.active[i] {
                // Only load position/velocity arrays - better cache usage
                for j in 0..3 {
                    self.positions[i][j] += self.velocities[i][j];
                }
            }
        }
    }
    
    // Even better: Process in chunks for better cache usage
    fn update_chunked(&mut self) {
        const CHUNK_SIZE: usize = 64;  // Tune for cache line size
        
        for chunk_start in (0..self.positions.len()).step_by(CHUNK_SIZE) {
            let chunk_end = (chunk_start + CHUNK_SIZE).min(self.positions.len());
            
            for i in chunk_start..chunk_end {
                if self.active[i] {
                    for j in 0..3 {
                        self.positions[i][j] += self.velocities[i][j];
                    }
                }
            }
        }
    }
}
```

### Data Alignment and Packing

```rust
// ❌ Poor alignment - wastes memory
#[repr(C)]
struct BadAlignment {
    a: u8,    // 1 byte
    b: u64,   // 8 bytes, but needs 8-byte alignment
    c: u8,    // 1 byte
    d: u32,   // 4 bytes
}
// Total size: 24 bytes (with padding)

// ✅ Good alignment - packed efficiently
#[repr(C)]
struct GoodAlignment {
    b: u64,   // 8 bytes (largest first)
    d: u32,   // 4 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte
}
// Total size: 16 bytes

// ✅ Explicit packing when needed
#[repr(C, packed)]
struct PackedStruct {
    a: u8,
    b: u64,
    c: u8,
}
// Total size: 10 bytes (no padding)

fn analyze_layout() {
    println!("BadAlignment: {} bytes", std::mem::size_of::<BadAlignment>());
    println!("GoodAlignment: {} bytes", std::mem::size_of::<GoodAlignment>());
    println!("PackedStruct: {} bytes", std::mem::size_of::<PackedStruct>());
}
```

## 💾 Memory Reuse Patterns

### Buffer Reuse

```rust
struct BufferManager {
    buffer: Vec<u8>,
}

impl BufferManager {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
    
    fn get_buffer(&mut self, size: usize) -> &mut [u8] {
        // Resize only if needed (preserve capacity when shrinking)
        if self.buffer.len() < size {
            self.buffer.resize(size, 0);
        }
        &mut self.buffer[..size]
    }
    
    fn process_file(&mut self, path: &str) -> std::io::Result<()> {
        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len() as usize;
        
        // Reuse buffer, growing only when necessary
        let buffer = self.get_buffer(file_size);
        std::fs::File::open(path)?.read_exact(buffer)?;
        
        // Process buffer...
        process_data(buffer);
        
        Ok(())
    }
}
```

### String Interning

```rust
use std::collections::HashMap;

struct StringInterner {
    strings: Vec<String>,
    indices: HashMap<String, usize>,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            strings: Vec::new(),
            indices: HashMap::new(),
        }
    }
    
    fn intern(&mut self, s: &str) -> StringId {
        if let Some(&id) = self.indices.get(s) {
            StringId(id)
        } else {
            let id = self.strings.len();
            self.strings.push(s.to_string());
            self.indices.insert(s.to_string(), id);
            StringId(id)
        }
    }
    
    fn resolve(&self, id: StringId) -> &str {
        &self.strings[id.0]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct StringId(usize);

// Usage: Saves memory when same strings appear repeatedly
fn process_tokens(tokens: &[&str]) -> HashMap<StringId, u32> {
    let mut interner = StringInterner::new();
    let mut counts = HashMap::new();
    
    for &token in tokens {
        let id = interner.intern(token);  // Dedup automatically
        *counts.entry(id).or_insert(0) += 1;
    }
    
    counts
}
```

## 🔄 Memory Access Patterns

### Sequential vs Random Access

```rust
use std::time::Instant;

fn benchmark_access_patterns() {
    const SIZE: usize = 10_000_000;
    let data: Vec<i32> = (0..SIZE as i32).collect();
    
    // ✅ Sequential access - cache friendly
    let start = Instant::now();
    let sum: i32 = data.iter().sum();
    let sequential_time = start.elapsed();
    
    // ❌ Random access - cache hostile
    let indices: Vec<usize> = (0..SIZE).rev().collect();  // Reverse order
    let start = Instant::now();
    let sum: i32 = indices.iter().map(|&i| data[i]).sum();
    let random_time = start.elapsed();
    
    println!("Sequential: {:?}, Random: {:?}", sequential_time, random_time);
    println!("Random is {:.1}x slower", random_time.as_nanos() as f64 / sequential_time.as_nanos() as f64);
}
```

### Cache-Friendly Data Structures

```rust
// ❌ Cache-unfriendly: Linked list
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Each node is a separate allocation - terrible cache behavior
fn sum_linked_list(head: &Option<Box<Node<i32>>>) -> i32 {
    let mut sum = 0;
    let mut current = head;
    while let Some(node) = current {
        sum += node.data;  // Cache miss for each node
        current = &node.next;
    }
    sum
}

// ✅ Cache-friendly: Chunked storage
struct ChunkedList<T> {
    chunks: Vec<Vec<T>>,
    chunk_size: usize,
}

impl<T> ChunkedList<T> {
    fn new(chunk_size: usize) -> Self {
        Self {
            chunks: Vec::new(),
            chunk_size,
        }
    }
    
    fn push(&mut self, value: T) {
        if self.chunks.is_empty() || self.chunks.last().unwrap().len() >= self.chunk_size {
            self.chunks.push(Vec::with_capacity(self.chunk_size));
        }
        self.chunks.last_mut().unwrap().push(value);
    }
    
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.chunks.iter().flat_map(|chunk| chunk.iter())
    }
}

// Better cache behavior due to spatial locality within chunks
fn sum_chunked_list(list: &ChunkedList<i32>) -> i32 {
    list.iter().sum()  // Process entire chunks at once
}
```

## 🧪 Memory Usage Measurement

### Custom Allocator for Tracking

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            let current = ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
            PEAK_ALLOCATED.fetch_max(current, Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn get_memory_stats() -> (usize, usize) {
    (
        ALLOCATED.load(Ordering::SeqCst),
        PEAK_ALLOCATED.load(Ordering::SeqCst),
    )
}

// Usage
fn memory_intensive_operation() {
    let (start_mem, _) = get_memory_stats();
    
    let data: Vec<Vec<i32>> = (0..1000)
        .map(|_| vec![0; 1000])
        .collect();
    
    let (current_mem, peak_mem) = get_memory_stats();
    println!("Allocated: {} bytes", current_mem - start_mem);
    println!("Peak: {} bytes", peak_mem);
    
    drop(data);
    
    let (end_mem, _) = get_memory_stats();
    println!("After drop: {} bytes", end_mem - start_mem);
}
```

### Memory Profiling with heaptrack

```bash
# Install heaptrack (Linux)
sudo apt install heaptrack

# Profile your application
heaptrack ./target/release/my_program

# Analyze results
heaptrack_gui heaptrack.my_program.12345.gz
```

## 📊 Memory Optimization Checklist

### Before Optimizing
- [ ] Profile memory usage to identify actual problems
- [ ] Measure allocation frequency and patterns
- [ ] Identify hot paths and data structures
- [ ] Benchmark current performance

### Allocation Optimization
- [ ] Pre-allocate containers when size is known
- [ ] Reuse buffers instead of allocating new ones
- [ ] Use object pools for frequently created/destroyed objects
- [ ] Consider arena allocation for request/response patterns

### Layout Optimization
- [ ] Organize struct fields by size (largest first)
- [ ] Consider struct-of-arrays for data processing
- [ ] Pack related data together for cache efficiency
- [ ] Minimize padding and alignment waste

### Access Pattern Optimization
- [ ] Favor sequential over random memory access
- [ ] Process data in cache-friendly chunks
- [ ] Use iterators for automatic vectorization
- [ ] Consider prefetching for predictable patterns

### Advanced Techniques
- [ ] String interning for duplicate strings
- [ ] Copy-on-write for rarely modified data
- [ ] Memory mapping for large files
- [ ] Custom allocators for specialized use cases

## 🎯 Platform-Specific Considerations

### Linux
- Use `jemalloc` for better allocation performance
- Consider huge pages for large allocations
- Monitor `/proc/meminfo` for system memory pressure

### Windows
- Consider Windows heap APIs for specialized use cases
- Monitor working set and commit charge
- Be aware of memory fragmentation issues

### Embedded Systems
- Stack allocation preferred over heap
- Consider no-std environments
- Be mindful of total memory constraints

## 🎓 Key Takeaways

1. **Allocations are expensive** - Minimize frequency and size
2. **Cache behavior dominates** - Keep related data together
3. **Pre-allocation beats growing** - Size containers appropriately
4. **Reuse beats allocation** - Pool objects when possible
5. **Sequential beats random** - Design for cache-friendly access
6. **Measure, don't guess** - Profile before optimizing

Remember: The fastest memory access is the one that doesn't happen. Sometimes the best optimization is eliminating memory usage entirely!