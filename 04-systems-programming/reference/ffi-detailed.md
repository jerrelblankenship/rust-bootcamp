# Lesson 03: Foreign Function Interface (FFI)

Safely bridge Rust and C code for maximum interoperability. Learn to interface with C libraries from Rust and export Rust functions for C consumption with better type safety than P/Invoke.

## 🏢 Enterprise FFI Use Cases

Enterprise systems often need to integrate with existing C/C++ libraries. Rust's FFI provides safer alternatives to C# P/Invoke:
- **Legacy system integration** - Banking systems with decades-old C code
- **Hardware device drivers** - Industrial control systems and embedded devices
- **High-performance libraries** - OpenSSL, graphics libraries, scientific computing
- **Operating system APIs** - Direct Windows/Linux system calls
- **Database drivers** - Native database connectors without managed overhead
- **Cryptographic modules** - Hardware security modules (HSMs) with C interfaces
- **Real-time protocols** - CAN bus, Modbus, and other industrial protocols

Rust's FFI provides better memory safety guarantees than C# P/Invoke while maintaining similar performance.

## 🔄 For C# Developers

### P/Invoke vs Rust FFI
```csharp
// C# P/Invoke
[DllImport("user32.dll")]
public static extern int MessageBox(IntPtr hWnd, string text, string caption, uint type);

// Usage
MessageBox(IntPtr.Zero, "Hello", "Title", 0);
```

```rust
// Rust FFI
extern "C" {
    fn printf(format: *const std::os::raw::c_char, ...) -> std::os::raw::c_int;
}

// Usage (unsafe required)
unsafe {
    let hello = b"Hello, World!\n\0";
    printf(hello.as_ptr() as *const std::os::raw::c_char);
}
```

**Key Insight**: Rust FFI is more explicit about safety but gives you finer control over data marshaling.

**Enterprise Advantage**: Unlike C# P/Invoke where marshaling errors cause runtime crashes, Rust's FFI makes safety explicit through unsafe blocks and type-safe pointer handling.

## 📞 Essential Patterns

### C String Conversion
```rust
use std::ffi::{CStr, CString};

// Rust to C: Always use CString
let rust_str = "Hello";
let c_string = CString::new(rust_str).unwrap();
unsafe {
    some_c_function(c_string.as_ptr());
}

// C to Rust: Always use CStr
unsafe {
    let c_str = CStr::from_ptr(c_string_ptr);
    let rust_str = c_str.to_string_lossy();
}
```

### C-Compatible Structs
```rust
// C-compatible struct
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Export for C consumption
#[no_mangle]
pub extern "C" fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}
```

### Error Handling Across Boundaries
```rust
// C-compatible error codes
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum CResult {
    Success = 0,
    DivisionByZero = -1,
    InvalidInput = -2,
    Unknown = -999,
}

// Safe function with C-compatible errors
#[no_mangle]
pub extern "C" fn safe_divide(a: c_int, b: c_int, result: *mut c_int) -> CResult {
    if result.is_null() {
        return CResult::InvalidInput;
    }
    
    if b == 0 {
        return CResult::DivisionByZero;
    }
    
    unsafe {
        *result = a / b;
    }
    CResult::Success
}
```

## 🔗 C# Comparison

| C# Feature | Rust Equivalent | Key Difference |
|------------|-----------------|----------------|
| `[DllImport]` | `extern "C"` | More explicit safety |
| `Marshal.StringToHGlobalAnsi` | `CString::new()` | Explicit C string creation |
| `IntPtr` | `*mut c_void` | Type-safe raw pointers |
| Exception marshaling | Error code enums | Explicit error propagation |

## 🎯 Key Takeaways

1. **Type Safety**: Rust FFI is safer than C# P/Invoke by default
2. **Memory Management**: Careful attention to ownership across boundaries
3. **String Handling**: Always use CString/CStr for C strings
4. **Error Propagation**: Design clear error codes for C consumers

## 💻 Practice Time!

Ready to fix FFI integration problems? Go to **exercises/ex03-c-interop.rs** and start debugging!

You'll fix:
- C string conversion errors
- Memory ownership violations across FFI boundaries
- Struct layout compatibility issues
- Callback function signature mismatches

The compiler will help you build robust, safe FFI interfaces!

---

Ready for the **System Monitor Project**? Let's build a real application that demonstrates all these concepts!