# Lesson 02: Unsafe Rust - Safe Abstractions Over Dangerous Operations

Step outside managed safety for performance or interoperability. Learn how Rust's `unsafe` provides escape hatches while maintaining overall memory safety through careful API design.

## 🔄 For C# Developers

### Comparing Unsafe Contexts
```csharp
// C# unsafe - minimal restrictions
unsafe 
{
    int x = 42;
    int* ptr = &x;
    *ptr = 100;        // Easy to cause memory corruption
    ptr++;             // Pointer arithmetic
    *ptr = 200;        // Potential crash!
}
```

```rust
// Rust unsafe - controlled superpowers
unsafe {
    let mut x = 42;
    let ptr = &mut x as *mut i32;
    *ptr = 100;                    // Dereference requires unsafe
    
    // Pointer arithmetic is explicit
    let ptr2 = ptr.offset(1);      // More explicit than C#
    // *ptr2 = 200;                // Would be undefined behavior
}
```

**Key Insight**: Rust `unsafe` doesn't "turn off safety" - it unlocks 4 specific superpowers while keeping all other checks.

## 🔓 The Four Unsafe Superpowers

1. **Dereference raw pointers** - `*ptr`
2. **Call unsafe functions** - Functions marked `unsafe fn`
3. **Access mutable statics** - Global mutable state
4. **Implement unsafe traits** - `Send`, `Sync`, etc.

**Everything else still has safety checks!**

### Safe Abstraction Pattern
```rust
// Unsafe function with safety contract
unsafe fn dangerous_sum(ptr: *const i32, len: usize) -> i32 {
    // CONTRACT: ptr must point to at least len valid i32s
    let mut sum = 0;
    for i in 0..len {
        sum += *ptr.add(i);
    }
    sum
}

// Safe wrapper that upholds the contract
fn safe_sum_slice(slice: &[i32]) -> i32 {
    unsafe {
        // SAFETY: slice guarantees valid pointer and length
        dangerous_sum(slice.as_ptr(), slice.len())
    }
}
```

## 🛡️ Safety Guidelines

### When to Use Unsafe
**✅ Good reasons:**
- Building safe abstractions (like Vec, HashMap)
- FFI with C libraries
- Performance-critical code with proven safety
- Implementing low-level primitives

**❌ Bad reasons:**
- "The borrow checker is annoying"
- "I need a quick hack"
- "It compiles, so it must be fine"

### Documentation Pattern
```rust
unsafe fn my_unsafe_function(ptr: *const u8, len: usize) {
    // SAFETY REQUIREMENTS:
    // - ptr must be valid for reads of len bytes
    // - ptr must be properly aligned
    // - Memory must not be mutated during this call
    // - len must not overflow isize::MAX
    
    // Implementation...
}
```

## 🎯 Key Takeaways

1. **Minimal Usage**: Use unsafe sparingly and document why
2. **Safe Wrappers**: Always wrap unsafe code in safe APIs
3. **Clear Contracts**: Document safety requirements explicitly
4. **Testing**: Unsafe code needs extensive testing

## 💻 Practice Time!

Ready to debug unsafe code? Go to **exercises/ex02-unsafe-debugging.rs** and start fixing!

You'll debug:
- Raw pointer dereference errors
- Unsafe function call mistakes
- Memory safety violations
- Safe abstraction patterns

The compiler will catch your unsafe mistakes and guide you to correct solutions!

---

Next: [Foreign Function Interface](03-ffi.md) →