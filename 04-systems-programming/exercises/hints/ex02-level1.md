# Exercise 02 - Level 1 Hints 🟢

## Basic Unsafe Operations

You're learning to use Rust's basic unsafe operations by wrapping them in `unsafe {}` blocks.

### 🎯 What You're Fixing
Five compilation errors where unsafe operations need to be wrapped:
1. Raw pointer dereferencing
2. Pointer arithmetic  
3. Uninitialized memory handling
4. Transmute operations
5. Slice creation from raw parts

### 💡 Key Concept: The `unsafe` Block

Rust prevents unsafe operations at compile time. When you need them, wrap in `unsafe {}`:

```rust
// This won't compile:
let value = *raw_pointer;

// This will compile:
unsafe {
    let value = *raw_pointer;
}
```

### 🔄 C# Mental Model

Think of it like C#'s unsafe context:

```csharp
// C# unsafe context
unsafe {
    int* ptr = &value;
    int result = *ptr;  // Dereference allowed
}
```

```rust
// Rust unsafe block
unsafe {
    let ptr = &value as *const i32;
    let result = *ptr;  // Dereference allowed
}
```

### 🚨 What Makes These Operations Unsafe?

1. **Raw pointer dereference** - Might point to invalid memory
2. **Pointer arithmetic** - Can go out of bounds
3. **Uninitialized memory** - Contains garbage values
4. **Transmute** - Reinterprets bits unsafely
5. **Raw slice creation** - No bounds validation

### 🎯 Your Approach

1. **Fix ONE checkpoint at a time**
2. **Wrap the unsafe operation** in `unsafe {}`
3. **Understand WHY** it's unsafe
4. **Compile and test** after each fix

### 💭 Questions to Guide You

- Which specific line is causing the compilation error?
- What unsafe operation is being performed?
- How do you wrap just that operation in `unsafe {}`?
- Why does Rust consider this operation dangerous?

Remember: You're not building complex abstractions yet - just learning basic `unsafe {}` syntax!

Need specific code examples? Check Level 2 hints!