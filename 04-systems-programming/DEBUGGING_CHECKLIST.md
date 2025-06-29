# Module 04 Debugging Checklist - Systems Programming

## 🔍 Systems Programming Debugging Approach

Systems programming involves low-level operations that require careful attention to memory safety and FFI boundaries. Follow this systematic approach:

### **Step 1: Identify Systems Programming Error Type**

**Common systems programming error patterns:**
- [ ] "unsafe operation" → Using unsafe outside unsafe block
- [ ] "raw pointer" → Dereferencing without unsafe
- [ ] "lifetime may not live long enough" → Reference lifetime issues
- [ ] "misaligned pointer dereference" → Memory alignment problems
- [ ] "use of moved value" → Ownership across FFI boundaries
- [ ] "cannot convert" → FFI type mismatches

### **Step 2: C# Unsafe vs Rust Unsafe Mental Model**

**Mental model translation:**
```csharp
// C# unsafe - disables many safety checks
unsafe {
    int x = 42;
    int* ptr = &x;
    *ptr = 100;        // Minimal checking
    ptr++;             // Easy to cause corruption
    *ptr = 200;        // Runtime crash possible
}
```

```rust
// Rust unsafe - surgical precision
unsafe {
    let mut x = 42;
    let ptr = &mut x as *mut i32;
    *ptr = 100;                    // Only dereferencing requires unsafe
    
    let ptr2 = ptr.offset(1);      // Explicit pointer arithmetic
    // *ptr2 = 200;                // Would be undefined behavior
}
// All other safety checks still apply!
```

### **Step 3: Memory Layout Debugging Patterns**

**"misaligned pointer dereference" or padding issues:**
```rust
// Problem: Inefficient struct layout
#[repr(C)]
struct BadLayout {
    a: u8,    // 1 byte + 7 padding
    b: u64,   // 8 bytes  
    c: u8,    // 1 byte + 7 padding
}           // Total: 24 bytes

// Solution: Reorder fields by size
#[repr(C)]
struct GoodLayout {
    b: u64,   // 8 bytes
    a: u8,    // 1 byte
    c: u8,    // 1 byte + 6 padding
}           // Total: 16 bytes

// Debug: Check sizes
fn debug_layout() {
    println!("BadLayout size: {}", std::mem::size_of::<BadLayout>());
    println!("GoodLayout size: {}", std::mem::size_of::<GoodLayout>());
}
```

### **Step 4: Unsafe Code Debugging Patterns**

**"unsafe operation" - dereference outside unsafe block:**
```rust
// Problem:
fn buggy_function() {
    let x = 42;
    let ptr = &x as *const i32;
    let value = *ptr;  // ERROR: dereference requires unsafe
}

// Solution: Wrap dereference in unsafe
fn fixed_function() {
    let x = 42;
    let ptr = &x as *const i32;
    let value = unsafe { *ptr };  // OK: explicit unsafe block
}

// Better: Avoid raw pointers when references work
fn better_function() {
    let x = 42;
    let reference = &x;
    let value = *reference;  // Safe dereferencing
}
```

**Safe abstraction pattern:**
```rust
// Unsafe implementation with safe interface
struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl SafeBuffer {
    fn new(capacity: usize) -> Self {
        let ptr = unsafe {
            std::alloc::alloc(std::alloc::Layout::array::<u8>(capacity).unwrap())
        };
        SafeBuffer { ptr, len: 0, capacity }
    }
    
    // Safe interface - enforces bounds checking
    fn push(&mut self, value: u8) -> Result<(), &'static str> {
        if self.len >= self.capacity {
            return Err("Buffer full");
        }
        
        unsafe {
            *self.ptr.add(self.len) = value;  // SAFETY: bounds checked above
        }
        self.len += 1;
        Ok(())
    }
    
    fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            Some(unsafe { *self.ptr.add(index) })  // SAFETY: bounds checked
        } else {
            None
        }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.ptr, 
                std::alloc::Layout::array::<u8>(self.capacity).unwrap()
            );
        }
    }
}
```

### **Step 5: FFI Debugging Patterns**

**"cannot convert" between C and Rust types:**
```rust
// Problem: Wrong types for C interface
extern "C" {
    fn c_function(s: &str) -> i32;  // ERROR: &str not C-compatible
}

// Solution: Use C-compatible types
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

extern "C" {
    fn c_function(s: *const c_char) -> c_int;
}

// Safe wrapper
fn call_c_function(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let c_string = CString::new(s)?;
    let result = unsafe { c_function(c_string.as_ptr()) };
    Ok(result)
}
```

**C string conversion errors:**
```rust
// Problem: Null bytes in string
let bad_string = "Hello\0World";
let c_string = CString::new(bad_string).unwrap();  // PANIC: null byte

// Solution: Handle null bytes
let bad_string = "Hello\0World";
match CString::new(bad_string) {
    Ok(c_string) => {
        // Use c_string
    }
    Err(e) => {
        eprintln!("String contains null byte: {}", e);
        // Handle error appropriately
    }
}

// Or remove null bytes
let cleaned = bad_string.replace('\0', "");
let c_string = CString::new(cleaned).unwrap();  // Safe
```

## 🚨 Systems Programming Debugging Workflow

### **1. Memory Safety First**
**Before debugging compilation errors, check memory safety:**
- [ ] Are all pointer dereferences in `unsafe` blocks?
- [ ] Are pointer arithmetic operations bounds-checked?
- [ ] Do all allocations have corresponding deallocations?
- [ ] Are C string conversions handling null bytes?

### **2. Unsafe Block Audit**
**For each `unsafe` block, verify:**
- [ ] **Justification**: Why is unsafe needed here?
- [ ] **Safety comment**: Document the safety invariants
- [ ] **Minimal scope**: Unsafe block as small as possible
- [ ] **Safe interface**: Unsafe internals with safe public API

```rust
// Good unsafe block pattern
fn safe_read_array(ptr: *const u8, len: usize) -> Vec<u8> {
    // SAFETY: Caller guarantees ptr is valid for len bytes
    unsafe {
        std::slice::from_raw_parts(ptr, len).to_vec()
    }
    // Safe Vec returned to caller
}
```

### **3. FFI Debugging Steps**
**When C interop fails:**
- [ ] Check type compatibility (`c_int` vs `i32`)
- [ ] Verify string handling (`CString`/`CStr`)
- [ ] Confirm struct layout (`#[repr(C)]`)
- [ ] Test error propagation (C error codes → Rust Results)

### **4. Performance and Layout Verification**
```rust
// Debug memory layout
fn debug_struct_layout<T>() {
    println!("Size: {} bytes", std::mem::size_of::<T>());
    println!("Alignment: {} bytes", std::mem::align_of::<T>());
}

// Test at compile time
const_assert_eq!(std::mem::size_of::<MyStruct>(), 16);
```

## 🎯 Systems Programming Best Practices

### **Safe Unsafe Code Guidelines:**

**1. Minimal Unsafe Blocks**
```rust
// Bad: Large unsafe block
unsafe {
    let ptr = get_raw_pointer();
    let value1 = *ptr;
    let value2 = *ptr.offset(1);
    process_values(value1, value2);
    cleanup_pointer(ptr);
}

// Good: Minimal unsafe operations
let ptr = get_raw_pointer();
let value1 = unsafe { *ptr };
let value2 = unsafe { *ptr.offset(1) };
process_values(value1, value2);
cleanup_pointer(ptr);
```

**2. Safety Documentation**
```rust
unsafe fn read_memory(ptr: *const u8, len: usize) -> Vec<u8> {
    // SAFETY REQUIREMENTS:
    // - ptr must be valid for reads of `len` bytes
    // - ptr must be properly aligned for u8
    // - Memory must not be modified during this call
    // - len must not overflow when added to ptr
    
    std::slice::from_raw_parts(ptr, len).to_vec()
}
```

**3. Error Handling Across FFI**
```rust
// Good: Convert C errors to Rust Result
#[repr(C)]
enum CError {
    Success = 0,
    InvalidInput = -1,
    OutOfMemory = -2,
}

fn call_c_function() -> Result<i32, CError> {
    let result = unsafe { some_c_function() };
    match result {
        0..=i32::MAX => Ok(result),
        -1 => Err(CError::InvalidInput),
        -2 => Err(CError::OutOfMemory),
        _ => Err(CError::InvalidInput),
    }
}
```

## 🔧 Debugging Tools

```bash
# Check for undefined behavior
cargo miri run

# Memory leak detection
valgrind ./target/debug/your_program

# Address sanitizer (nightly)
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run

# Check unsafe code specifically
cargo clippy -- -W clippy::all

# Test with different optimizations
cargo build --release
cargo test --release
```

## 📚 Learning Mindset

**Remember:**
- **Unsafe is a superpower, not a free pass** - Use sparingly and document well
- **Memory layout matters** - Understanding alignment prevents bugs
- **FFI needs careful type matching** - C and Rust have different conventions
- **Test extensively** - Unsafe code needs more testing than safe code

**Mental model shift from C#:**
- In C#: "Unsafe disables many checks, be very careful"
- In Rust: "Unsafe enables specific operations, everything else still checked"

**Questions to ask with unsafe code:**
1. "Why can't I do this with safe Rust?"
2. "What are the safety invariants I must maintain?"
3. "How can I minimize the unsafe surface area?"
4. "What could go wrong if my assumptions are violated?"

---

**Ready for systems programming?** Work through the exercises systematically and build safe, high-performance code!
