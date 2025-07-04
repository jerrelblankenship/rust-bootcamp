# Exercise 06 - Level 2 Hints: Unsafe Undefined

## 🎯 Focus Areas

You should now understand that unsafe code requires maintaining safety invariants manually. Let's fix the specific undefined behavior issues.

## 🔧 Specific Issues to Fix

### Issue 1: Null Pointer Dereferences
```rust
// ❌ Wrong - no null check
unsafe fn process_raw_pointer(ptr: *const i32) -> i32 {
    *ptr  // Undefined behavior if ptr is null
}

// ✅ Correct - check for null
unsafe fn process_raw_pointer(ptr: *const i32) -> i32 {
    if ptr.is_null() {
        panic!("Null pointer dereference");
    }
    *ptr
}
```

### Issue 2: Uninitialized Memory
```rust
// ❌ Wrong - using uninitialized memory
let mut array: [i32; 10] = std::mem::uninitialized();

// ✅ Correct - use MaybeUninit
let mut array: [MaybeUninit<i32>; 10] = MaybeUninit::uninit_array();
// Initialize before use
for i in 0..10 {
    array[i] = MaybeUninit::new(i as i32);
}
```

### Issue 3: Aliasing Violations
```rust
// ❌ Wrong - multiple mutable references to same memory
let ref1 = &mut *ptr;
let ref2 = &mut *ptr;

// ✅ Correct - use split_at_mut or ensure non-overlapping
let (left, right) = slice.split_at_mut(slice.len() / 2);
```

### Issue 4: Memory Management
```rust
// ❌ Wrong - double free
dealloc(ptr as *mut u8, layout);
dealloc(ptr as *mut u8, layout);

// ✅ Correct - track allocation state
let mut ptr = alloc(layout) as *mut i32;
if !ptr.is_null() {
    *ptr = 42;
    dealloc(ptr as *mut u8, layout);
    ptr = std::ptr::null_mut();  // Prevent reuse
}
```

## 🔍 C# Comparison

```csharp
// C# unsafe code has similar issues
unsafe {
    int* ptr = null;
    *ptr = 42;  // Access violation
    
    // Proper null checking
    if (ptr != null) {
        *ptr = 42;
    }
}
```

## 🎮 Implementation Strategy

1. **Add null checks** - Always validate pointers
2. **Use MaybeUninit** - Replace deprecated uninitialized
3. **Fix aliasing** - Ensure exclusive mutable access
4. **Track memory state** - Prevent double free and use-after-free
5. **Validate transmute** - Ensure size and alignment compatibility

## 🔧 Code Patterns to Apply

### Pattern 1: Safe Pointer Usage
```rust
unsafe fn safe_pointer_deref(ptr: *const i32) -> Option<i32> {
    if ptr.is_null() {
        None
    } else {
        Some(*ptr)
    }
}
```

### Pattern 2: Proper Uninitialized Memory
```rust
use std::mem::MaybeUninit;

unsafe fn create_initialized_array<T: Clone>(size: usize, value: T) -> Vec<T> {
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(value.clone());
    }
    vec
}
```

### Pattern 3: Safe Abstraction Boundaries
```rust
fn safe_wrapper_fixed(data: &[i32], index: usize) -> Option<i32> {
    if index < data.len() {
        unsafe {
            let ptr = data.as_ptr().add(index);
            Some(*ptr)
        }
    } else {
        None
    }
}
```

## ⏰ Time Check

Spent 30 minutes total? If you're still struggling with specific unsafe patterns, move to Level 3 for complete solutions.

**Hint**: Focus on the `create_uninitialized_array` function - it needs to use `MaybeUninit` and proper initialization!