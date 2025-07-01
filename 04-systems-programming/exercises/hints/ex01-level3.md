# Exercise 01 - Level 3 Hints 🔴

## Complete Solutions for Memory Layout

### Step 1: Struct Syntax Fix
```rust
#[derive(Debug)]
struct SimpleStruct {
    field1: u8,
    field2: String,  // Add comma here
    field3: i32,
}
```

### Step 2: Memory Optimization (Preview)
```rust
// Bad order: causes padding
struct BadLayout {
    flag: bool,     // 1 byte
    id: u64,        // 8 bytes (7 bytes padding before this!)
    count: u16,     // 2 bytes (6 bytes padding after!)
}

// Good order: minimal padding  
struct GoodLayout {
    id: u64,        // 8 bytes
    count: u16,     // 2 bytes
    flag: bool,     // 1 byte (5 bytes padding at end)
}
```

### Step 3: Heap Allocation (Preview)
```rust
// Stack allocation
let stack_data = [1, 2, 3, 4, 5];

// Heap allocation  
let heap_data = Box::new([1, 2, 3, 4, 5]);
let vec_data = vec![1, 2, 3, 4, 5];  // Also heap
```

### Step 4: Zero-Copy Strings (Preview)
```rust
use std::borrow::Cow;

// Efficient string handling
let borrowed: Cow<str> = "static string".into();  // No allocation
let owned: Cow<str> = String::from("dynamic").into();  // Allocated

// Clone only when needed
let modified = if needs_change {
    format!("{} modified", borrowed).into()  // Allocates now
} else {
    borrowed  // Still no allocation
};
```

### Memory Layout Insights

**Memory sizes**:
- `u8`: 1 byte
- `u16`: 2 bytes  
- `u32`: 4 bytes
- `u64`: 8 bytes
- `bool`: 1 byte
- `String`: 24 bytes (pointer + len + capacity)

**Alignment rules**:
- Data must be aligned to its size
- Struct alignment = largest field alignment
- Padding added to maintain alignment

**C# vs Rust Memory**:
- C#: GC manages everything, some layout optimization
- Rust: Manual control, predictable layout, zero-cost abstractions

You now understand the fundamentals of Rust memory layout!