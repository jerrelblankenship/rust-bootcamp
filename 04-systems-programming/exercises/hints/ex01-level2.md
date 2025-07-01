# Exercise 01 - Level 2 Hints 🟡

## Specific Solutions for Memory Layout

### Step 1: Struct Syntax Error
```rust
// Problem: Missing comma after field2
struct SimpleStruct {
    field1: u8,
    field2: String  // ERROR: Missing comma!
    field3: i32,
}
```

**Fix**: Add comma after `field2: String`:
```rust
struct SimpleStruct {
    field1: u8,
    field2: String,  // Add comma here
    field3: i32,
}
```

### Step 2: Memory Optimization
The next step will ask you about struct field ordering for optimal memory layout.

**Key concept**: Rust doesn't reorder fields automatically like C# might. Order matters for:
- Memory usage (padding between fields)
- Cache performance
- Alignment requirements

### Step 3: Heap Allocation
You'll work with `Box<T>` for heap allocation.

**C# Comparison**:
```csharp
// C# - reference types automatically on heap
string text = "Hello";
var obj = new MyClass();
```

```rust
// Rust - explicit heap allocation
let text = String::from("Hello");  // Heap-allocated string
let obj = Box::new(MyStruct{});    // Explicitly boxed
```

### Step 4: Zero-Copy Strings
Introduction to `Cow<str>` (Clone on Write) for efficient string handling.

### Memory Layout Tips
- Smaller fields first can reduce padding
- Alignment follows the largest field
- Use `#[repr(C)]` for C-compatible layout

Need the complete solutions? Check Level 3 hints!