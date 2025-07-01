# Exercise 02 - Level 3 Hints 🔴

## Complete Working Solutions

### Checkpoint 1: Raw Pointer Dereferencing
```rust
fn checkpoint_1_raw_pointers() {
    println!("🔧 Checkpoint 1: Fix raw pointer usage");
    
    let x = 42i32;
    let ptr = &x as *const i32;
    
    // FIXED: Wrap dereference in unsafe block
    let value = unsafe { *ptr };
    
    println!("Value through pointer: {}", value);
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [██░░░░░░░░] 20% Complete");
    println!("Now uncomment checkpoint_2_pointer_arithmetic() in main()\n");
}
```

### Checkpoint 2: Pointer Arithmetic
```rust
fn checkpoint_2_pointer_arithmetic() {
    println!("🔧 Checkpoint 2: Fix pointer arithmetic");
    
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    
    // FIXED: Wrap all unsafe operations in single block
    unsafe {
        let third_element = ptr.add(2);  // Pointer arithmetic
        let value = *third_element;      // Dereference
        println!("Third element: {}", value);
    }
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [████░░░░░░] 40% Complete");
    println!("Now uncomment checkpoint_3_uninitialized_memory() in main()\n");
}
```

### Checkpoint 3: Uninitialized Memory
```rust
fn checkpoint_3_uninitialized_memory() {
    println!("🔧 Checkpoint 3: Fix uninitialized memory");
    
    // FIXED: Use MaybeUninit instead of deprecated mem::uninitialized
    use std::mem::MaybeUninit;
    
    let mut uninit: MaybeUninit<i32> = MaybeUninit::uninit();
    unsafe {
        uninit.write(42);  // Initialize with a value
    }
    let value = unsafe { uninit.assume_init() };  // Assume initialized
    
    println!("Initialized value: {}", value);
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [██████░░░░] 60% Complete");
    println!("Now uncomment checkpoint_4_transmute_operations() in main()\n");
}
```

### Checkpoint 4: Transmute Operations
```rust
fn checkpoint_4_transmute_operations() {
    println!("🔧 Checkpoint 4: Fix transmute operations");
    
    let bytes = [0x00, 0x00, 0x00, 0x2A]; // 42 in little-endian
    
    // FIXED: Wrap transmute in unsafe block
    let number: i32 = unsafe { mem::transmute(bytes) };
    
    println!("Transmuted number: {}", number);
    
    println!("✅ Checkpoint 4 complete!");
    println!("Progress: [████████░░] 80% Complete");
    println!("Now uncomment checkpoint_5_slice_from_raw() in main()\n");
}
```

### Checkpoint 5: Slice from Raw Parts
```rust
fn checkpoint_5_slice_from_raw() {
    println!("🔧 Checkpoint 5: Fix slice from raw parts");
    
    let data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();
    
    // FIXED: Wrap slice creation in unsafe block
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    
    println!("Slice from raw parts: {:?}", slice);
    
    println!("✅ Checkpoint 5 complete!");
    println!("Progress: [██████████] 100% Complete");
    println!("🎉 Exercise 2 completed!\n");
}
```

## Complete Main Function
```rust
fn main() {
    println!("=== Exercise 2: Basic Unsafe Operations ===\n");
    
    // Uncomment these ONE AT A TIME as you fix each checkpoint
    checkpoint_1_raw_pointers();
    checkpoint_2_pointer_arithmetic();
    checkpoint_3_uninitialized_memory();
    checkpoint_4_transmute_operations();
    checkpoint_5_slice_from_raw();
    
    println!("All checkpoints complete! 🎉");
    println!("Progress: [██████████] 100% Complete");
}
```

## Key Learning Points

### Why Use `unsafe` Blocks?
1. **Compiler protection** - Prevents accidental unsafe operations
2. **Code audit** - Easy to find all unsafe code sections
3. **Documentation** - Makes unsafe operations explicit
4. **Minimal scope** - Keeps unsafe code contained

### Safety Responsibilities
When you write `unsafe {}`, you promise:
1. **Pointers are valid** - Not null, properly aligned
2. **Memory is allocated** - Not freed or uninitialized
3. **No data races** - No concurrent access violations
4. **Bounds are checked** - No out-of-bounds access

### Best Practices
1. **Minimize unsafe blocks** - Keep them as small as possible
2. **Document safety contracts** - Explain why code is safe
3. **Validate inputs** - Check preconditions before unsafe operations
4. **Use safe abstractions** - Wrap unsafe code in safe APIs

### C# vs Rust Unsafe Comparison

| Aspect | C# | Rust |
|--------|-----|------|
| **Syntax** | `unsafe { }` | `unsafe { }` |
| **Scope** | Method or block level | Expression level |
| **Validation** | Runtime bounds checking | Manual validation required |
| **Memory management** | GC as safety net | Manual responsibility |
| **Pointer arithmetic** | `ptr + offset` | `ptr.add(offset)` |

Congratulations! You now understand how to use basic unsafe operations in Rust. 

**Next:** Move to `ex03-manual-memory.rs` to learn manual memory management!