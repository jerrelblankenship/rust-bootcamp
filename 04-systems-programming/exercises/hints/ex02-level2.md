# Exercise 02 - Level 2 Hints 🟡

## Specific Solutions for Each Checkpoint

### Checkpoint 1: Raw Pointer Dereferencing

**The Error:**
```rust
let value = *ptr;  // COMPILE ERROR: unsafe operation!
```

**The Fix:**
```rust
let value = unsafe { *ptr };  // Wrap dereference in unsafe block
```

**Why it's unsafe:** Raw pointers might be null or point to invalid memory.

### Checkpoint 2: Pointer Arithmetic

**The Error:**
```rust
let third_element = ptr.add(2);  // COMPILE ERROR: unsafe operation!
let value = *third_element;      // COMPILE ERROR: unsafe dereference!
```

**The Fix:**
```rust
unsafe {
    let third_element = ptr.add(2);  // Pointer arithmetic
    let value = *third_element;      // Dereference
    println!("Third element: {}", value);
}
```

**Why it's unsafe:** Pointer arithmetic can go out of bounds.

### Checkpoint 3: Uninitialized Memory

**The Error:**
```rust
uninit = mem::uninitialized();  // COMPILE ERROR: deprecated unsafe!
```

**The Fix:**
```rust
use std::mem::MaybeUninit;

let mut uninit: MaybeUninit<i32> = MaybeUninit::uninit();
unsafe {
    uninit.write(42);  // Initialize the value
}
let value = unsafe { uninit.assume_init() };  // Assume initialized
println!("Initialized value: {}", value);
```

**Why it's safer:** `MaybeUninit` prevents use of uninitialized memory.

### Checkpoint 4: Transmute Operations

**The Error:**
```rust
let number: i32 = mem::transmute(bytes);  // COMPILE ERROR: unsafe!
```

**The Fix:**
```rust
let number: i32 = unsafe { mem::transmute(bytes) };
```

**Why it's unsafe:** Transmute reinterprets memory bits without validation.

### Checkpoint 5: Slice from Raw Parts

**The Error:**
```rust
let slice = std::slice::from_raw_parts(ptr, len);  // COMPILE ERROR: unsafe!
```

**The Fix:**
```rust
let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
```

**Why it's unsafe:** No validation that the pointer and length are valid.

## C# Comparison

In C#, similar operations also require unsafe context:

```csharp
// C# pointer arithmetic
unsafe {
    int* ptr = stackalloc int[5];
    int* third = ptr + 2;  // Pointer arithmetic
    *third = 42;           // Dereference
}
```

```rust
// Rust equivalent
unsafe {
    let ptr = arr.as_ptr();
    let third = ptr.add(2);  // Pointer arithmetic
    *third                   // Dereference (returns value)
}
```

## Key Differences from C#

1. **Explicit unsafe blocks** - Rust requires `unsafe {}` around each operation
2. **No automatic initialization** - Must explicitly handle uninitialized memory
3. **Stricter transmute** - Rust validates size and alignment
4. **No pointer arithmetic operators** - Use `.add()`, `.sub()` methods

Need complete working solutions? Check Level 3 hints!