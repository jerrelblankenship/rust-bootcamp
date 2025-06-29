# Lesson 03: Foreign Function Interface (FFI)

One of Rust's greatest strengths is its ability to interoperate with existing C libraries and systems. Coming from C#, you understand P/Invoke for calling native code. Rust's FFI provides similar capabilities but with more safety guarantees and zero-cost abstractions.

## 🎯 Learning Objectives

- Master calling C functions from Rust
- Learn to expose Rust functions to C
- Understand safe wrapper patterns for unsafe FFI
- Compare with C# P/Invoke and Interop
- Build practical integrations with system libraries

## 🌉 FFI Fundamentals

### C# P/Invoke vs Rust FFI

#### C# P/Invoke Example
```csharp
using System;
using System.Runtime.InteropServices;

public class NativeMethods
{
    [DllImport("kernel32.dll")]
    public static extern IntPtr GetCurrentProcess();
    
    [DllImport("libc")]
    public static extern int strlen(IntPtr str);
    
    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int MessageBox(IntPtr hWnd, string text, string caption, uint type);
}

// Usage with manual memory management
public void UseNative()
{
    var process = NativeMethods.GetCurrentProcess();
    // Memory management is manual and error-prone
}
```

#### Rust FFI Example
```rust
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// Declare external C functions
extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(format: *const c_char, ...) -> c_int;
}

// Safe wrapper functions
fn safe_strlen(s: &str) -> usize {
    let c_string = CString::new(s).expect("CString::new failed");
    unsafe {
        strlen(c_string.as_ptr())
    }
}

fn safe_printf(message: &str) {
    let c_string = CString::new(message).expect("CString::new failed");
    unsafe {
        printf(c_string.as_ptr());
    }
}

fn ffi_demo() {
    let test_string = "Hello, FFI!";
    let length = safe_strlen(test_string);
    println!("String '{}' has length: {}", test_string, length);
    
    safe_printf("Hello from Rust via C printf!\n");
}
```

## 📝 Working with C Strings

String handling is one of the trickiest parts of FFI:

```rust
use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::raw::c_char;

// Different string types and their use cases
fn string_conversion_examples() {
    // Rust String to C string
    let rust_string = "Hello, World!";
    let c_string = CString::new(rust_string).expect("CString::new failed");
    println!("C string: {:?}", c_string);
    
    // C string back to Rust (from a pointer)
    let c_str_ptr = c_string.as_ptr();
    let back_to_rust = unsafe {
        CStr::from_ptr(c_str_ptr).to_string_lossy()
    };
    println!("Back to Rust: {}", back_to_rust);
    
    // Handling potential null terminators in input
    let potentially_invalid = "Hello\0World"; // Contains null byte
    match CString::new(potentially_invalid) {
        Ok(c_str) => println!("Valid C string: {:?}", c_str),
        Err(e) => println!("Invalid C string: {}", e),
    }
}

// Safe string operations with C functions
extern "C" {
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

fn safe_string_operations() {
    let source = "Hello";
    let append = " World";
    
    // Allocate buffer for destination
    let mut buffer = vec![0u8; 100]; // 100 bytes should be enough
    
    let source_c = CString::new(source).unwrap();
    let append_c = CString::new(append).unwrap();
    
    unsafe {
        // Copy source to buffer
        strcpy(buffer.as_mut_ptr() as *mut c_char, source_c.as_ptr());
        
        // Append to buffer
        strcat(buffer.as_mut_ptr() as *mut c_char, append_c.as_ptr());
        
        // Convert back to Rust string
        let result_ptr = buffer.as_ptr() as *const c_char;
        let result = CStr::from_ptr(result_ptr);
        println!("Result: {}", result.to_string_lossy());
    }
}
```

## 🔧 Calling System Libraries

### Math Library Integration

```rust
use std::os::raw::{c_double, c_int};

// Link with the math library
#[link(name = "m")]
extern "C" {
    fn sin(x: c_double) -> c_double;
    fn cos(x: c_double) -> c_double;
    fn pow(x: c_double, y: c_double) -> c_double;
    fn sqrt(x: c_double) -> c_double;
    fn fabs(x: c_double) -> c_double;
}

// Safe wrappers for math functions
pub mod math {
    use super::*;
    
    pub fn sine(x: f64) -> f64 {
        unsafe { sin(x) }
    }
    
    pub fn cosine(x: f64) -> f64 {
        unsafe { cos(x) }
    }
    
    pub fn power(base: f64, exponent: f64) -> f64 {
        unsafe { pow(base, exponent) }
    }
    
    pub fn square_root(x: f64) -> Option<f64> {
        if x < 0.0 {
            None
        } else {
            Some(unsafe { sqrt(x) })
        }
    }
    
    pub fn absolute(x: f64) -> f64 {
        unsafe { fabs(x) }
    }
}

fn math_library_demo() {
    use std::f64::consts::PI;
    
    println!("Math library demonstration:");
    println!("sin(π/2) = {}", math::sine(PI / 2.0));
    println!("cos(0) = {}", math::cosine(0.0));
    println!("2^3 = {}", math::power(2.0, 3.0));
    
    match math::square_root(16.0) {
        Some(result) => println!("√16 = {}", result),
        None => println!("Cannot take square root of negative number"),
    }
    
    println!("|−5.5| = {}", math::absolute(-5.5));
}
```

### File System Operations

```rust
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};

extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn chmod(pathname: *const c_char, mode: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
}

// Constants for access() function
const F_OK: c_int = 0; // File exists
const R_OK: c_int = 4; // Read permission
const W_OK: c_int = 2; // Write permission
const X_OK: c_int = 1; // Execute permission

pub mod file_ops {
    use super::*;
    
    pub fn file_exists(path: &str) -> bool {
        let c_path = match CString::new(path) {
            Ok(path) => path,
            Err(_) => return false,
        };
        
        unsafe {
            access(c_path.as_ptr(), F_OK) == 0
        }
    }
    
    pub fn can_read(path: &str) -> bool {
        let c_path = match CString::new(path) {
            Ok(path) => path,
            Err(_) => return false,
        };
        
        unsafe {
            access(c_path.as_ptr(), R_OK) == 0
        }
    }
    
    pub fn can_write(path: &str) -> bool {
        let c_path = match CString::new(path) {
            Ok(path) => path,
            Err(_) => return false,
        };
        
        unsafe {
            access(c_path.as_ptr(), W_OK) == 0
        }
    }
    
    pub fn delete_file(path: &str) -> Result<(), std::io::Error> {
        let c_path = CString::new(path)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput, 
                "Invalid path"
            ))?;
        
        let result = unsafe { unlink(c_path.as_ptr()) };
        
        if result == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

fn file_system_demo() {
    // Create a test file
    std::fs::write("test_file.txt", "Hello, FFI!").unwrap();
    
    println!("File operations via FFI:");
    println!("File exists: {}", file_ops::file_exists("test_file.txt"));
    println!("Can read: {}", file_ops::can_read("test_file.txt"));
    println!("Can write: {}", file_ops::can_write("test_file.txt"));
    
    // Clean up
    match file_ops::delete_file("test_file.txt") {
        Ok(()) => println!("File deleted successfully"),
        Err(e) => println!("Failed to delete file: {}", e),
    }
}
```

## 📤 Exposing Rust to C

### Creating C-Compatible Functions

```rust
use std::os::raw::{c_char, c_int, c_double};
use std::ffi::{CStr, CString};
use std::ptr;

// Export Rust functions with C ABI
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_multiply(a: c_double, b: c_double) -> c_double {
    a * b
}

#[no_mangle]
pub extern "C" fn rust_string_length(s: *const c_char) -> c_int {
    if s.is_null() {
        return -1;
    }
    
    unsafe {
        let c_str = CStr::from_ptr(s);
        match c_str.to_str() {
            Ok(rust_str) => rust_str.len() as c_int,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_create_string(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let c_str = CStr::from_ptr(input);
        let rust_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        let processed = format!("Processed: {}", rust_str);
        
        match CString::new(processed) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            // Convert back to CString to properly deallocate
            let _ = CString::from_raw(s);
        }
    }
}

// More complex example: Process an array
#[no_mangle]
pub extern "C" fn rust_process_array(
    input: *const c_int,
    length: usize,
    output: *mut c_int
) -> c_int {
    if input.is_null() || output.is_null() {
        return -1;
    }
    
    unsafe {
        let input_slice = std::slice::from_raw_parts(input, length);
        let output_slice = std::slice::from_raw_parts_mut(output, length);
        
        for (i, &value) in input_slice.iter().enumerate() {
            output_slice[i] = value * 2; // Double each value
        }
    }
    
    0 // Success
}

// Callback function support
type Callback = extern "C" fn(c_int) -> c_int;

#[no_mangle]
pub extern "C" fn rust_map_array(
    input: *const c_int,
    length: usize,
    output: *mut c_int,
    callback: Option<Callback>
) -> c_int {
    let callback = match callback {
        Some(cb) => cb,
        None => return -1,
    };
    
    if input.is_null() || output.is_null() {
        return -1;
    }
    
    unsafe {
        let input_slice = std::slice::from_raw_parts(input, length);
        let output_slice = std::slice::from_raw_parts_mut(output, length);
        
        for (i, &value) in input_slice.iter().enumerate() {
            output_slice[i] = callback(value);
        }
    }
    
    0
}
```

### C Header Generation

```rust
// To generate C headers, you can use cbindgen
// Add to Cargo.toml:
// [build-dependencies]
// cbindgen = "0.20"

// build.rs
/*
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("include/rust_ffi.h");
}
*/

// This would generate a header like:
/*
// rust_ffi.h
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

int32_t rust_add(int32_t a, int32_t b);

double rust_multiply(double a, double b);

int32_t rust_string_length(const char *s);

char *rust_create_string(const char *input);

void rust_free_string(char *s);

int32_t rust_process_array(const int32_t *input, uintptr_t length, int32_t *output);

typedef int32_t (*Callback)(int32_t);

int32_t rust_map_array(const int32_t *input,
                       uintptr_t length,
                       int32_t *output,
                       Callback callback);
*/
```

## 🔗 Working with Complex C Libraries

### SQLite Integration Example

```rust
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

// SQLite C API declarations
#[repr(C)]
pub struct sqlite3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sqlite3_stmt {
    _private: [u8; 0],
}

extern "C" {
    fn sqlite3_open(filename: *const c_char, ppDb: *mut *mut sqlite3) -> c_int;
    fn sqlite3_close(db: *mut sqlite3) -> c_int;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const c_char,
        nByte: c_int,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const c_char,
    ) -> c_int;
    fn sqlite3_step(stmt: *mut sqlite3_stmt) -> c_int;
    fn sqlite3_finalize(stmt: *mut sqlite3_stmt) -> c_int;
    fn sqlite3_column_text(stmt: *mut sqlite3_stmt, iCol: c_int) -> *const c_char;
    fn sqlite3_column_int(stmt: *mut sqlite3_stmt, iCol: c_int) -> c_int;
    fn sqlite3_errmsg(db: *mut sqlite3) -> *const c_char;
}

// SQLite constants
const SQLITE_OK: c_int = 0;
const SQLITE_ROW: c_int = 100;
const SQLITE_DONE: c_int = 101;

// Safe SQLite wrapper
pub struct Database {
    db: *mut sqlite3,
}

impl Database {
    pub fn open(path: &str) -> Result<Self, String> {
        let c_path = CString::new(path).map_err(|_| "Invalid path")?;
        let mut db = ptr::null_mut();
        
        let result = unsafe {
            sqlite3_open(c_path.as_ptr(), &mut db)
        };
        
        if result == SQLITE_OK {
            Ok(Database { db })
        } else {
            let error_msg = unsafe {
                let msg_ptr = sqlite3_errmsg(db);
                CStr::from_ptr(msg_ptr).to_string_lossy().to_string()
            };
            
            if !db.is_null() {
                unsafe { sqlite3_close(db); }
            }
            
            Err(error_msg)
        }
    }
    
    pub fn execute_query(&self, sql: &str) -> Result<Vec<Vec<String>>, String> {
        let c_sql = CString::new(sql).map_err(|_| "Invalid SQL")?;
        let mut stmt = ptr::null_mut();
        
        let result = unsafe {
            sqlite3_prepare_v2(
                self.db,
                c_sql.as_ptr(),
                -1,
                &mut stmt,
                ptr::null_mut(),
            )
        };
        
        if result != SQLITE_OK {
            return Err("Failed to prepare statement".to_string());
        }
        
        let mut rows = Vec::new();
        
        loop {
            let step_result = unsafe { sqlite3_step(stmt) };
            
            if step_result == SQLITE_ROW {
                let mut row = Vec::new();
                
                // For simplicity, assume first two columns are text and int
                unsafe {
                    let text_ptr = sqlite3_column_text(stmt, 0);
                    if !text_ptr.is_null() {
                        let text = CStr::from_ptr(text_ptr as *const c_char)
                            .to_string_lossy()
                            .to_string();
                        row.push(text);
                    }
                    
                    let int_val = sqlite3_column_int(stmt, 1);
                    row.push(int_val.to_string());
                }
                
                rows.push(row);
            } else if step_result == SQLITE_DONE {
                break;
            } else {
                unsafe { sqlite3_finalize(stmt); }
                return Err("Error stepping through results".to_string());
            }
        }
        
        unsafe { sqlite3_finalize(stmt); }
        Ok(rows)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.db.is_null() {
            unsafe {
                sqlite3_close(self.db);
            }
        }
    }
}

// Usage example
fn sqlite_example() {
    match Database::open(":memory:") {
        Ok(db) => {
            println!("Database opened successfully");
            
            // Create table and insert data would go here
            // let _ = db.execute_query("CREATE TABLE test (name TEXT, value INTEGER)");
        }
        Err(e) => {
            println!("Failed to open database: {}", e);
        }
    }
}
```

## 🛡️ Safety Patterns and Best Practices

### RAII for Resource Management

```rust
use std::os::raw::c_void;

extern "C" {
    fn custom_resource_create() -> *mut c_void;
    fn custom_resource_destroy(resource: *mut c_void);
    fn custom_resource_process(resource: *mut c_void, data: *const u8, len: usize) -> i32;
}

// RAII wrapper for C resource
pub struct ManagedResource {
    resource: *mut c_void,
}

impl ManagedResource {
    pub fn new() -> Result<Self, &'static str> {
        let resource = unsafe { custom_resource_create() };
        
        if resource.is_null() {
            Err("Failed to create resource")
        } else {
            Ok(ManagedResource { resource })
        }
    }
    
    pub fn process(&self, data: &[u8]) -> Result<i32, &'static str> {
        let result = unsafe {
            custom_resource_process(self.resource, data.as_ptr(), data.len())
        };
        
        if result >= 0 {
            Ok(result)
        } else {
            Err("Processing failed")
        }
    }
}

impl Drop for ManagedResource {
    fn drop(&mut self) {
        if !self.resource.is_null() {
            unsafe {
                custom_resource_destroy(self.resource);
            }
        }
    }
}

// Safe to send between threads if the underlying C library supports it
unsafe impl Send for ManagedResource {}
```

### Error Handling Patterns

```rust
use std::fmt;

#[derive(Debug)]
pub enum FfiError {
    NullPointer,
    InvalidInput(String),
    CLibraryError(i32),
    StringConversion,
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiError::NullPointer => write!(f, "Null pointer encountered"),
            FfiError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            FfiError::CLibraryError(code) => write!(f, "C library error: {}", code),
            FfiError::StringConversion => write!(f, "String conversion failed"),
        }
    }
}

impl std::error::Error for FfiError {}

// Type alias for convenience
pub type FfiResult<T> = Result<T, FfiError>;

// Safe wrapper with proper error handling
pub fn safe_c_function_wrapper(input: &str) -> FfiResult<String> {
    if input.is_empty() {
        return Err(FfiError::InvalidInput("Input cannot be empty".to_string()));
    }
    
    let c_input = CString::new(input)
        .map_err(|_| FfiError::StringConversion)?;
    
    let result_ptr = unsafe {
        // Assume this C function returns malloc'd string or null
        rust_create_string(c_input.as_ptr())
    };
    
    if result_ptr.is_null() {
        return Err(FfiError::NullPointer);
    }
    
    let result = unsafe {
        let c_str = CStr::from_ptr(result_ptr);
        let rust_string = c_str.to_str()
            .map_err(|_| FfiError::StringConversion)?
            .to_string();
        
        // Free the C string
        rust_free_string(result_ptr);
        
        rust_string
    };
    
    Ok(result)
}
```

## 🎯 Key Takeaways

1. **Safety First**: Always wrap unsafe FFI calls in safe abstractions
2. **Resource Management**: Use RAII patterns for cleanup
3. **Error Handling**: Provide meaningful error types for FFI failures
4. **String Handling**: Be careful with string conversions and lifetimes
5. **Testing**: Thoroughly test FFI code for memory safety

## 💻 Exercises

### Exercise 1: Math Library Wrapper
```rust
// TODO: Create a comprehensive wrapper for math library functions
// Include error handling for domain errors (like sqrt of negative numbers)

extern "C" {
    fn sin(x: f64) -> f64;
    fn log(x: f64) -> f64;
    fn exp(x: f64) -> f64;
}

pub mod safe_math {
    use super::*;
    
    #[derive(Debug)]
    pub enum MathError {
        DomainError,
        RangeError,
    }
    
    pub fn logarithm(x: f64) -> Result<f64, MathError> {
        // TODO: Implement with domain checking
        todo!()
    }
    
    pub fn exponential(x: f64) -> Result<f64, MathError> {
        // TODO: Implement with range checking
        todo!()
    }
}
```

### Exercise 2: File I/O Wrapper
```rust
// TODO: Create safe wrappers for POSIX file operations
extern "C" {
    fn open(pathname: *const i8, flags: i32) -> i32;
    fn read(fd: i32, buf: *mut u8, count: usize) -> isize;
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn close(fd: i32) -> i32;
}

pub struct FileHandle {
    // TODO: Add fields
}

impl FileHandle {
    pub fn open(path: &str, read_only: bool) -> Result<Self, std::io::Error> {
        // TODO: Implement safe file opening
        todo!()
    }
    
    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        // TODO: Implement safe reading
        todo!()
    }
    
    pub fn write(&self, data: &[u8]) -> Result<usize, std::io::Error> {
        // TODO: Implement safe writing
        todo!()
    }
}
```

### Exercise 3: C Callback Integration
```rust
// TODO: Create a system that allows C code to call Rust callbacks
type EventCallback = extern "C" fn(event_type: i32, data: *const u8, len: usize);

extern "C" {
    fn register_callback(callback: EventCallback);
    fn unregister_callback();
    fn trigger_events();
}

pub struct EventSystem {
    // TODO: Add state management
}

impl EventSystem {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn register<F>(&mut self, callback: F) 
    where 
        F: Fn(i32, &[u8]) + Send + 'static 
    {
        // TODO: Bridge Rust closures to C function pointers
        todo!()
    }
}
```

## 📚 Additional Resources

- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) - Comprehensive FFI examples
- [cbindgen](https://github.com/eqrion/cbindgen) - Generate C headers from Rust
- [bindgen](https://docs.rs/bindgen/) - Generate Rust bindings from C headers

---

Next Module: [05 - Concurrency](../../05-concurrency/README.md) →
