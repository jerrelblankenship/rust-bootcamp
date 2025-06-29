# Lesson 01: Memory Layout and Control

Take control of memory representation and build high-performance, zero-allocation code. Learn how Rust gives you explicit control over memory layout while maintaining safety.

## 🔄 For C# Developers

### From Managed to Manual Memory
```csharp
// C# - CLR controls everything
public class Person  // Reference type - heap allocated
{
    public string Name { get; set; }  // Reference to heap string
    public int Age { get; set; }      // Inline value
}

// No control over layout or allocation
var person = new Person { Name = "Alice", Age = 30 };
```

```rust
// Rust - You control the layout
#[derive(Debug)]
struct Person {
    name: String,     // Heap-allocated string data
    age: u32,         // Inline value, 4 bytes
}

// Stack allocation by default
let person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Explicit heap allocation when needed
let boxed_person = Box::new(person);
```

**Key Insight**: Rust gives you explicit control over where data lives and how it's arranged.

## 📐 Core Concepts

### Struct Padding and Optimization
```rust
// Wasteful layout - padding everywhere
struct Bad {
    a: u8,    // 1 byte + 7 padding
    b: u64,   // 8 bytes
    c: u8,    // 1 byte + 7 padding  
}

// Efficient layout - larger fields first  
struct Good {
    b: u64,   // 8 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte + 6 padding
}
```

### Stack vs Heap Performance
```rust
// Stack - extremely fast
let data = [0u8; 1024];  // 1KB on stack

// Heap - slower but more flexible
let data = vec![0u8; 1024];  // 1KB on heap
```

### Zero-Copy Operations
```rust
use std::borrow::Cow;

// Allocate only when needed
fn process_string(input: &str) -> Cow<str> {
    if input.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(input.to_uppercase())  // Allocate
    } else {
        Cow::Borrowed(input)              // Zero-copy
    }
}
```

## 🎯 Key Takeaways

1. **Explicit Control**: Rust gives you explicit control over memory layout
2. **Zero-Cost Abstractions**: High-level features don't add runtime overhead  
3. **Performance**: Understanding layout enables significant optimizations
4. **Safety**: Memory safety without garbage collection overhead

## 💻 Practice Time!

Ready to fix broken memory layout code? Go to **exercises/ex01-memory-layout.rs** and start debugging!

You'll fix:
- Struct padding and alignment errors
- Stack overflow from large allocations
- Zero-copy operation bugs
- Memory layout optimization challenges

The compiler will guide you through each fix, building your systems programming intuition!

---

Next: [Unsafe Rust](02-unsafe-rust.md) →