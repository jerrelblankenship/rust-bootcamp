// Exercise 02: Unsafe Operations - Fix Broken Code!
//
// BROKEN: This code has 5 unsafe-related compilation errors
// Your mission: Fix each error to understand unsafe fundamentals
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 checkpoints to fix)
//
// APPROACH:
// - Fix ONE error at a time following the FIXME comments  
// - Compile after each fix: `rustc ex02-unsafe-operations.rs`
// - Understand WHY each operation is unsafe
// - Use hints only after trying for 15+ minutes per checkpoint
//
// C# COMPARISON: Like using unsafe blocks and pointers, but with
// more explicit safety requirements and no GC safety net!

use std::ptr;
use std::mem;

fn main() {
    println!("=== Exercise 2A: Basic Unsafe Operations ===\n");
    
    // Fix these ONE AT A TIME - uncomment as you go
    checkpoint_1_raw_pointers();
    // checkpoint_2_pointer_arithmetic();
    // checkpoint_3_uninitialized_memory();
    // checkpoint_4_transmute_operations();
    // checkpoint_5_slice_from_raw();
    
    println!("All checkpoints complete! 🎉");
    println!("Progress: [██████████] 100% Complete");
}

// ✅ CHECKPOINT 1: Fix raw pointer dereferencing
fn checkpoint_1_raw_pointers() {
    println!("🔧 Checkpoint 1: Fix raw pointer usage");
    
    let x = 42i32;
    let ptr = &x as *const i32;
    
    // FIXME: This won't compile - dereferencing raw pointer
    // ERROR: dereference of raw pointer is unsafe
    let value = *ptr;  // COMPILE ERROR: unsafe operation!
    
    println!("Value through pointer: {}", value);
    
    println!("✅ Checkpoint 1 complete!");
    println!("Progress: [██░░░░░░░░] 20% Complete");
    println!("Now uncomment checkpoint_2_pointer_arithmetic() in main()\n");
}

// ✅ CHECKPOINT 2: Fix pointer arithmetic
fn checkpoint_2_pointer_arithmetic() {
    println!("🔧 Checkpoint 2: Fix pointer arithmetic");
    
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    
    // FIXME: Pointer arithmetic is unsafe
    // ERROR: call to unsafe function is unsafe
    let third_element = ptr.add(2);  // COMPILE ERROR: unsafe operation!
    let value = *third_element;      // COMPILE ERROR: unsafe dereference!
    
    println!("Third element: {}", value);
    
    println!("✅ Checkpoint 2 complete!");
    println!("Progress: [████░░░░░░] 40% Complete");
    println!("Now uncomment checkpoint_3_uninitialized_memory() in main()\n");
}

// ✅ CHECKPOINT 3: Fix uninitialized memory access
fn checkpoint_3_uninitialized_memory() {
    println!("🔧 Checkpoint 3: Fix uninitialized memory");
    
    // FIXME: Using uninitialized memory is unsafe
    // ERROR: use of possibly-uninitialized variable
    let mut uninit: i32;
    uninit = mem::uninitialized();  // COMPILE ERROR: deprecated unsafe!
    
    println!("Uninitialized value: {}", uninit);
    
    // HINT: Use MaybeUninit<T> for safe uninitialized memory
    
    println!("✅ Checkpoint 3 complete!");
    println!("Progress: [██████░░░░] 60% Complete");
    println!("Now uncomment checkpoint_4_transmute_operations() in main()\n");
}

// ✅ CHECKPOINT 4: Fix transmute operations
fn checkpoint_4_transmute_operations() {
    println!("🔧 Checkpoint 4: Fix transmute operations");
    
    let bytes = [0x00, 0x00, 0x00, 0x2A]; // 42 in little-endian
    
    // FIXME: transmute is unsafe
    // ERROR: call to unsafe function is unsafe  
    let number: i32 = mem::transmute(bytes);  // COMPILE ERROR: unsafe!
    
    println!("Transmuted number: {}", number);
    
    println!("✅ Checkpoint 4 complete!");
    println!("Progress: [████████░░] 80% Complete");
    println!("Now uncomment checkpoint_5_slice_from_raw() in main()\n");
}

// ✅ CHECKPOINT 5: Fix slice from raw parts
fn checkpoint_5_slice_from_raw() {
    println!("🔧 Checkpoint 5: Fix slice from raw parts");
    
    let data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();
    
    // FIXME: Creating slice from raw parts is unsafe
    // ERROR: call to unsafe function is unsafe
    let slice = std::slice::from_raw_parts(ptr, len);  // COMPILE ERROR: unsafe!
    
    println!("Slice from raw parts: {:?}", slice);
    
    println!("✅ Checkpoint 5 complete!");
    println!("Progress: [██████████] 100% Complete");
    println!("🎉 Exercise 2A completed! Move to ex02-unsafe-abstractions.rs\n");
}

/*
COMPILATION CHALLENGES:
1. error[E0133]: dereference of raw pointer is unsafe
2. error[E0133]: call to unsafe function is unsafe  
3. error[E0133]: use of deprecated `std::mem::uninitialized`
4. error[E0133]: call to unsafe function `transmute` is unsafe
5. error[E0133]: call to unsafe function `from_raw_parts` is unsafe

C# COMPARISON:
In C# you might write:
```csharp
unsafe {
    int x = 42;
    int* ptr = &x;
    int value = *ptr;  // Automatic unsafe context
}
```

In Rust, each unsafe operation must be explicitly marked!

SAFETY RULES:
- Raw pointer dereference: Must ensure pointer is valid
- Pointer arithmetic: Must not go out of bounds  
- Uninitialized memory: Must initialize before reading
- Transmute: Must ensure types have same size/alignment
- Raw slices: Must ensure pointer and length are valid

HINTS:
- Level 1: What keyword makes unsafe operations safe?
- Level 2: Wrap unsafe operations in unsafe blocks
- Level 3: Use `unsafe { ... }` around each operation
*/