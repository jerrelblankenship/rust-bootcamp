# Memory Visualizer Project

Build a command-line tool that demonstrates Rust's ownership model and helps visualize memory management concepts.

## 🎯 Project Goals

Create a memory visualizer that:
- Demonstrates ownership transfer and borrowing
- Shows memory allocation and deallocation
- Visualizes stack vs heap operations  
- Tracks reference counting with smart pointers
- Compares with C#'s garbage collection model
- Uses all concepts learned in Module 02

## 📋 Requirements

### Core Features
1. **Memory Tracking**: Track allocation and deallocation of values
2. **Ownership Visualization**: Show when ownership transfers occur
3. **Borrowing Demo**: Demonstrate immutable and mutable borrowing
4. **Smart Pointer Examples**: Show Rc, Arc, Box usage patterns
5. **Interactive Mode**: Step through operations to see memory changes

### Stretch Goals
1. **Memory Graph**: ASCII art visualization of memory layout
2. **Performance Comparison**: Compare with simulated GC operations
3. **Leak Detection**: Show how Rust prevents memory leaks
4. **Concurrency Demo**: Arc<Mutex<T>> patterns for shared state

## 🏗️ Project Structure

```
project-memory-visualizer/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point
│   ├── memory_tracker.rs    # Core memory tracking
│   ├── ownership_demo.rs    # Ownership demonstrations
│   ├── borrowing_demo.rs    # Borrowing demonstrations
│   ├── smart_pointers.rs    # Smart pointer examples
│   └── visualizer.rs        # ASCII visualization
└── tests/
    └── integration_tests.rs # Integration tests
```

## 🚀 Implementation Guide

### Step 1: Set Up Project Structure

Create the Cargo.toml with appropriate dependencies:

```toml
[package]
name = "memory-visualizer"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "memory-visualizer"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
colored = "2.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```

### Step 2: Define Core Data Structures

```rust
// src/memory_tracker.rs

#[derive(Debug, Clone)]
pub struct MemoryOperation {
    pub operation: Operation,
    pub location: String,
    pub value_type: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Allocate,
    Deallocate,
    Move,
    Borrow,
    Clone,
}

pub struct MemoryTracker {
    operations: Vec<MemoryOperation>,
    active_allocations: HashMap<String, usize>,
}
```

### Step 3: Implement Ownership Demonstrations

Start with broken code that students need to fix:

```rust
// src/ownership_demo.rs

pub fn demonstrate_moves() {
    println!("=== Ownership Transfer Demo ===");
    
    // TODO: Fix the compilation errors below
    let s1 = String::from("Hello");
    let s2 = s1;  // FIXME: s1 is moved here
    
    // This should print both strings - make it work!
    println!("s1: {}", s1);  // ERROR: Fix this
    println!("s2: {}", s2);
    
    // HINT: Look up the .clone() method
    // QUESTION: What's the difference between move and copy?
}
```

### Step 4: Core Requirements

Your visualizer should handle:

1. **Basic Operations**: Create, move, drop values
2. **Borrowing**: Show immutable and mutable references
3. **Smart Pointers**: Demonstrate Rc, Box, Arc usage
4. **Memory Visualization**: ASCII representation of stack/heap
5. **Interactive Mode**: Step through operations

### Step 5: Example Output

```bash
$ memory-visualizer --demo ownership

=== Ownership Demo ===
Step 1: Creating String "Hello"
  Stack: [s1] -> Heap: ["Hello" (5 bytes)]
  
Step 2: Moving s1 to s2
  Stack: [s2] -> Heap: ["Hello" (5 bytes)]
  Note: s1 is no longer valid
  
Step 3: s2 goes out of scope
  Heap: [deallocated]
  Total memory freed: 5 bytes

$ memory-visualizer --demo borrowing

=== Borrowing Demo ===
Step 1: Creating data
  Stack: [data] -> Heap: [vec![1,2,3] (24 bytes)]
  
Step 2: Creating immutable reference
  Stack: [data, ref1 -> data]
  Multiple readers allowed: ✓
  
Step 3: Creating mutable reference
  Stack: [data, mut_ref -> data]
  Exclusive access: ✓
```

## 🧪 Testing Your Implementation

Create integration tests that verify correct behavior:

```rust
// tests/integration_tests.rs

#[test]
fn test_ownership_transfer() {
    // Test that ownership demo shows correct transfer
}

#[test]
fn test_borrowing_rules() {
    // Test that borrowing demo enforces rules
}

#[test]
fn test_memory_tracking() {
    // Test that memory allocation is tracked correctly
}
```

## 🎯 Learning Objectives

Through this project, you'll master:

1. **Ownership Rules**: How values are transferred and dropped
2. **Borrowing**: Safe references without ownership transfer
3. **Lifetimes**: How Rust ensures references remain valid
4. **Smart Pointers**: When and how to use Box, Rc, Arc
5. **Memory Safety**: How Rust prevents common C++ and C# issues

## 📚 Incremental Development

### Phase 1: Basic Structure (Day 1)
- [ ] Set up project with Cargo.toml
- [ ] Create main.rs with CLI argument parsing
- [ ] Implement basic memory tracking structure
- [ ] Create simple ownership demonstration

### Phase 2: Core Features (Day 2)
- [ ] Implement borrowing demonstrations
- [ ] Add smart pointer examples
- [ ] Create basic ASCII visualization
- [ ] Add interactive mode

### Phase 3: Advanced Features (Day 3)
- [ ] Implement memory graph visualization
- [ ] Add performance comparisons
- [ ] Create comprehensive test suite
- [ ] Polish CLI interface and error handling

### Phase 4: Stretch Goals (Optional)
- [ ] Add concurrency demonstrations
- [ ] Implement leak detection scenarios
- [ ] Create educational mode with explanations
- [ ] Add configuration file support

## 🔄 C# Comparison Mode

Your visualizer should include a mode that compares Rust's approach with C#:

```bash
$ memory-visualizer --compare-csharp

=== Memory Management Comparison ===

C# Approach:
  var obj1 = new MyObject();
  var obj2 = obj1;  // Both reference same object
  obj2.Modify();    // obj1 is also modified
  // GC collects when no references remain

Rust Approach:
  let obj1 = MyStruct::new();
  let obj2 = obj1;  // Ownership MOVES to obj2
  // obj1 is no longer valid
  // obj2 is dropped deterministically when out of scope
```

## 🚀 Running the Project

```bash
# Basic ownership demo
cargo run -- --demo ownership

# Borrowing demonstration
cargo run -- --demo borrowing

# Smart pointers showcase
cargo run -- --demo smart-pointers

# Interactive exploration
cargo run -- --interactive

# Compare with C# model
cargo run -- --compare-csharp

# Full help
cargo run -- --help
```

## 🌟 Extension Ideas

### 1. Web Interface
Create a web-based visualizer using a Rust web framework.

### 2. Performance Benchmarks
Compare actual performance between Rust and simulated GC operations.

### 3. Educational Animations
Step-by-step animations showing memory operations.

### 4. Custom Types
Allow users to define custom structs and see their memory layout.

## 📝 What You'll Build

By the end of this project, you'll have:

1. **Deep Understanding**: How Rust's ownership model works
2. **Practical Tool**: Visualizer for teaching others
3. **Production Patterns**: Real-world ownership and borrowing usage
4. **Testing Skills**: Comprehensive test coverage
5. **CLI Experience**: Professional command-line tool

---

**Ready to start?** Begin with the basic project setup and ownership demonstrations, then build complexity incrementally.

Next Module: [03 - Error Handling](../../03-error-handling/README.md) →
