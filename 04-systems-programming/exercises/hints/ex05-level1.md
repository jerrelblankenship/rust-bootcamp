# Exercise 05 - Level 1 Hints 🟢

## Understanding Foreign Function Interface (FFI)

You're bridging Rust and C code - the foundation of systems integration! This is like P/Invoke in C# but with more control over memory safety.

## Key Questions to Consider

1. **Why do we need FFI?** Integrate with existing C libraries and system APIs
2. **What are the challenges?** Different memory models, calling conventions, error handling
3. **How is this like C# P/Invoke?**
   ```csharp
   [DllImport("kernel32.dll")]
   public static extern IntPtr GetCurrentProcess();
   ```
   But Rust gives you more control over safety!

## Core Concepts

- **extern "C"**: C calling convention
- **#[repr(C)]**: C-compatible struct layout
- **String conversion**: Rust String ↔ C char*
- **Memory ownership**: Who allocates and frees?

## Common FFI Patterns

### Calling C from Rust
```rust
extern "C" {
    fn c_function(param: i32) -> i32;
}
```

### Exposing Rust to C
```rust
#[no_mangle]
pub extern "C" fn rust_function(param: i32) -> i32 {
    // Implementation
}
```

### String Handling
Use `CString` and `CStr` for safe string conversion

### Error Handling
Convert C error codes to Rust Results

## Safety Considerations

1. **Null pointers**: Always check before dereferencing
2. **String lifetime**: Ensure C doesn't outlive Rust strings
3. **Memory ownership**: Clear contracts about who owns what
4. **Error propagation**: Handle C errors gracefully

Need specific FFI patterns? Check Level 2 hints!