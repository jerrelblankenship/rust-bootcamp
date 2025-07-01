# Exercise 02 - Level 1 Hints 🟢

## Understanding Unsafe Abstractions

You're building safe APIs on top of unsafe operations - this is advanced Rust! This is like building a safe C# wrapper around unsafe native code.

## Key Questions to Consider

1. **What makes code "unsafe"?** Raw pointers, manual memory management, and unchecked operations

2. **How do we maintain safety?** Through careful API design and invariant checking

3. **In C#, what's similar?**
   ```csharp
   // C# unsafe code
   unsafe {
       int* ptr = stackalloc int[10];
       *ptr = 42;
   }
   ```
   But C# has GC as a safety net. Rust has ownership rules!

## Concepts to Review

- **Raw pointers**: `*mut T` and `*const T`
- **Memory allocation**: Using `std::alloc` for manual memory management
- **Safety invariants**: Rules that must be maintained
- **RAII**: Resource Acquisition Is Initialization

## The SafeVec Challenge

You're implementing a dynamic array similar to `Vec<T>` but from scratch:

1. **Manual memory management** with `alloc` and `dealloc`
2. **Safe public API** that can't be misused
3. **Proper cleanup** in Drop implementation
4. **Growth strategy** for dynamic resizing

## Common Unsafe Operations

- Dereferencing raw pointers
- Calling unsafe functions
- Accessing mutable static variables
- Implementing unsafe traits

## Safety Strategy

1. **Encapsulation**: Keep unsafe code in small, reviewable functions
2. **Invariants**: Document and maintain what must always be true
3. **Testing**: Comprehensive tests for edge cases
4. **Documentation**: Explain safety contracts

Need more specific guidance? Check Level 2 hints after trying for 15+ more minutes!