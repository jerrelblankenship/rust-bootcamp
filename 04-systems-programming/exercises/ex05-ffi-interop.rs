// Exercise 05: Foreign Function Interface (FFI) - Fix Broken Code!
//
// BROKEN: This code has 6 FFI-related compilation errors
// Your mission: Fix each error to master C interoperability
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// APPROACH:
// - Fix ONE FFI error at a time following the FIXME comments
// - Compile after each fix: `rustc ex05-ffi-interop.rs`
// - Understand C compatibility requirements
// - Use hints only after trying for 15+ minutes per checkpoint
//
// C# COMPARISON: Like P/Invoke but with explicit memory safety!

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

fn main() {
    println!("=== Exercise 3: Fix C Interop and FFI Problems ===\n");
    
    exercise_3_1();
    exercise_3_2();
    exercise_3_3();
    exercise_3_4();
    exercise_3_5();
    exercise_3_6();
}

// Exercise 3.1: Fix C function declarations
fn exercise_3_1() {
    println!("Exercise 3.1: Fix C function declarations");
    
    // FIXME: These extern declarations have syntax errors
    extern "C" {
        fn strlen(s: *const c_char) -> usize;
        fn strcpy(dest: *mut c_char src: *const c_char) -> *mut c_char;  // COMPILE ERROR: Syntax error!
        fn malloc(size: usize) -> *mut c_void
        fn free(ptr: *mut c_void);  // COMPILE ERROR: Missing semicolon above!
    }
    
    // FIXME: Using C functions without unsafe
    let test_string = b"Hello, World!\0";
    let length = strlen(test_string.as_ptr() as *const c_char);  // COMPILE ERROR: extern fn is unsafe
    println!("String length: {}", length);
    
    // TODO: Create safe wrappers for these C functions
    fn safe_strlen(s: &str) -> usize {
        // TODO: Convert Rust string to C string and call strlen safely
        let c_string = CString::new(s).unwrap();
        unsafe {
            strlen(c_string.as_ptr())
        }
    }
    
    fn safe_malloc(size: usize) -> Option<*mut u8> {
        // TODO: Safely call malloc and handle null return
        unsafe {
            let ptr = malloc(size) as *mut u8;
            if ptr.is_null() {
                None
            } else {
                Some(ptr)
            }
        }
    }
    
    fn safe_free(ptr: *mut u8) {
        // TODO: Safely free memory allocated by malloc
        if !ptr.is_null() {
            unsafe {
                free(ptr as *mut c_void);
            }
        }
    }
    
    // Test the safe wrappers
    let len = safe_strlen("Hello, World!");
    println!("Safe string length: {}", len);
    
    if let Some(ptr) = safe_malloc(100) {
        println!("Allocated 100 bytes at: {:p}", ptr);
        safe_free(ptr);
        println!("Memory freed");
    }
    
    println!("✅ Exercise 3.1 complete\n");
}

// Exercise 3.2: Fix C string conversion errors
fn exercise_3_2() {
    println!("Exercise 3.2: Fix C string conversions");
    
    // FIXME: This C string conversion is broken
    fn broken_string_to_c(s: &str) -> *const c_char {
        let rust_string = s.to_string();
        rust_string.as_ptr() as *const c_char  // COMPILE ERROR: Invalid conversion!
        // PROBLEM: Rust string is not null-terminated, and it gets dropped!
    }
    
    // TODO: Fix the string conversion
    fn safe_string_to_c(s: &str) -> CString {
        // TODO: Create proper null-terminated C string
        CString::new(s).expect("String contains null byte")
    }
    
    // FIXME: This C to Rust conversion is also broken
    unsafe fn broken_c_to_string(c_str: *const c_char) -> String {
        let slice = std::slice::from_raw_parts(c_str as *const u8, ???);  // COMPILE ERROR: Unknown length!
        String::from_utf8_lossy(slice).into_owned()
    }
    
    // TODO: Fix the C to Rust conversion
    unsafe fn safe_c_to_string(c_str: *const c_char) -> Result<String, std::str::Utf8Error> {
        if c_str.is_null() {
            return Ok(String::new());
        }
        
        // TODO: Use CStr to safely convert C string to Rust string
        let c_str_safe = CStr::from_ptr(c_str);
        c_str_safe.to_str().map(|s| s.to_owned())
    }
    
    // TODO: Working with C string arrays
    extern "C" {
        // Simulated C function that takes string array
        fn process_string_array(strings: *const *const c_char, count: c_int) -> c_int;
    }
    
    // TODO: Create safe wrapper for string array processing
    fn call_process_strings(strings: &[&str]) -> i32 {
        // TODO: Convert Rust string slice to C string array
        let c_strings: Vec<CString> = strings
            .iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        
        let c_ptrs: Vec<*const c_char> = c_strings
            .iter()
            .map(|cs| cs.as_ptr())
            .collect();
        
        unsafe {
            // For this exercise, we'll just return the count since we can't link to real C code
            c_ptrs.len() as i32
        }
    }
    
    // Test string conversions
    let rust_str = "Hello, C World!";
    let c_string = safe_string_to_c(rust_str);
    
    unsafe {
        if let Ok(converted_back) = safe_c_to_string(c_string.as_ptr()) {
            println!("Round-trip conversion: {}", converted_back);
        }
    }
    
    let string_array = ["first", "second", "third"];
    let result = call_process_strings(&string_array);
    println!("Processed {} strings", result);
    
    println!("✅ Exercise 3.2 complete\n");
}

// Exercise 3.3: Fix struct layout for C compatibility
fn exercise_3_3() {
    println!("Exercise 3.3: Fix C-compatible structs");
    
    // FIXME: This struct is not C-compatible
    #[derive(Debug)]
    struct Point {  // MISSING: #[repr(C)] attribute!
        x: f64,
        y: f64,
        z: f64,
    }
    
    // FIXME: This struct has layout issues
    #[repr(C)]
    struct Person {
        name: String,        // PROBLEM: String is not C-compatible!
        age: c_int,
        height: f64,
    }
    
    // TODO: Create C-compatible versions
    #[repr(C)]
    #[derive(Debug)]
    struct CPoint {
        // TODO: Add repr(C) to make it C-compatible
        x: f64,
        y: f64,
        z: f64,
    }
    
    #[repr(C)]
    #[derive(Debug)]
    struct CPerson {
        name: *const c_char,  // C string pointer instead of Rust String
        age: c_int,
        height: f64,
    }
    
    impl CPerson {
        // TODO: Safe constructor that handles C string
        fn new(name: &str, age: i32, height: f64) -> (Self, CString) {
            let c_name = CString::new(name).unwrap();
            let person = CPerson {
                name: c_name.as_ptr(),
                age,
                height,
            };
            (person, c_name)  // Return CString to keep it alive
        }
        
        // TODO: Safe method to get name as Rust string
        fn get_name(&self) -> Result<String, std::str::Utf8Error> {
            if self.name.is_null() {
                return Ok(String::new());
            }
            
            unsafe {
                CStr::from_ptr(self.name).to_str().map(|s| s.to_owned())
            }
        }
    }
    
    // TODO: Export functions for C consumption
    #[no_mangle]
    pub extern "C" fn create_point(x: f64, y: f64, z: f64) -> CPoint {
        CPoint { x, y, z }
    }
    
    #[no_mangle]
    pub extern "C" fn point_distance(p1: CPoint, p2: CPoint) -> f64 {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        let dz = p1.z - p2.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    #[no_mangle]
    pub extern "C" fn person_get_age(person: *const CPerson) -> c_int {
        // TODO: Safely dereference the person pointer
        if person.is_null() {
            return -1;
        }
        
        unsafe {
            (*person).age
        }
    }
    
    // Test the C-compatible structs
    let point1 = create_point(1.0, 2.0, 3.0);
    let point2 = create_point(4.0, 5.0, 6.0);
    let distance = point_distance(point1, point2);
    
    println!("Point 1: {:?}", point1);
    println!("Point 2: {:?}", point2);
    println!("Distance: {:.2}", distance);
    
    let (person, _name_holder) = CPerson::new("Alice", 30, 5.6);
    println!("Person age: {}", person_get_age(&person));
    if let Ok(name) = person.get_name() {
        println!("Person name: {}", name);
    }
    
    println!("✅ Exercise 3.3 complete\n");
}

// Exercise 3.4: Fix callback function signatures
fn exercise_3_4() {
    println!("Exercise 3.4: Fix callback functions");
    
    // FIXME: This callback type definition has errors
    type BrokenCallback = fn(value: i32) -> i32;  // MISSING: extern "C"
    
    // TODO: Define correct C-compatible callback type
    type CCallback = extern "C" fn(value: c_int) -> c_int;
    
    // FIXME: This function signature doesn't match C calling convention
    fn process_with_callback(data: &[i32], callback: fn(i32) -> i32) -> i32 {  // WRONG: Not C-compatible
        data.iter().map(|&x| callback(x)).sum()
    }
    
    // TODO: Fix the function to accept C callbacks
    fn safe_process_with_callback(data: &[i32], callback: CCallback) -> i32 {
        data.iter().map(|&x| callback(x)).sum()
    }
    
    // TODO: Export function that accepts callback
    #[no_mangle]
    pub extern "C" fn process_array_with_callback(
        data: *const c_int,
        len: usize,
        callback: CCallback
    ) -> c_int {
        if data.is_null() || len == 0 {
            return -1;
        }
        
        unsafe {
            let slice = std::slice::from_raw_parts(data, len);
            slice.iter().map(|&x| callback(x)).sum()
        }
    }
    
    // TODO: Example callback functions
    extern "C" fn double_value(x: c_int) -> c_int {
        x * 2
    }
    
    extern "C" fn square_value(x: c_int) -> c_int {
        x * x
    }
    
    // Test callback functionality
    let data = [1, 2, 3, 4, 5];
    
    let doubled_sum = safe_process_with_callback(&data, double_value);
    println!("Sum of doubled values: {}", doubled_sum);
    
    let squared_sum = safe_process_with_callback(&data, square_value);
    println!("Sum of squared values: {}", squared_sum);
    
    // Test exported function
    let c_result = process_array_with_callback(
        data.as_ptr(),
        data.len(),
        double_value
    );
    println!("C-compatible function result: {}", c_result);
    
    println!("✅ Exercise 3.4 complete\n");
}

// Exercise 3.5: Fix error handling across FFI boundary
fn exercise_3_5() {
    println!("Exercise 3.5: Fix FFI error handling");
    
    // FIXME: Can't return Rust Result across FFI boundary
    #[no_mangle]
    pub extern "C" fn broken_divide(a: c_int, b: c_int) -> Result<c_int, String> {  // COMPILE ERROR!
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    // TODO: Define C-compatible error codes
    #[repr(C)]
    #[derive(Debug, PartialEq)]
    pub enum CResult {
        Success = 0,
        DivisionByZero = -1,
        InvalidInput = -2,
        OutOfMemory = -3,
        Unknown = -999,
    }
    
    // TODO: Fix function to return C-compatible error
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
    
    // TODO: Create helper for error message strings
    #[no_mangle]
    pub extern "C" fn get_error_message(error: CResult) -> *const c_char {
        let message = match error {
            CResult::Success => "Success",
            CResult::DivisionByZero => "Division by zero",
            CResult::InvalidInput => "Invalid input",
            CResult::OutOfMemory => "Out of memory",
            CResult::Unknown => "Unknown error",
        };
        
        // Return pointer to static string (safe because it's 'static)
        message.as_ptr() as *const c_char
    }
    
    // TODO: Working with optional values across FFI
    #[no_mangle]
    pub extern "C" fn find_in_array(
        data: *const c_int,
        len: usize,
        target: c_int,
        found_index: *mut usize
    ) -> CResult {
        if data.is_null() || found_index.is_null() {
            return CResult::InvalidInput;
        }
        
        unsafe {
            let slice = std::slice::from_raw_parts(data, len);
            
            for (i, &value) in slice.iter().enumerate() {
                if value == target {
                    *found_index = i;
                    return CResult::Success;
                }
            }
            
            // Not found - but this isn't really an error
            CResult::Unknown  // Maybe we need a "NotFound" variant
        }
    }
    
    // Test error handling
    let mut result = 0;
    
    match safe_divide(10, 2, &mut result) {
        CResult::Success => println!("10 / 2 = {}", result),
        error => {
            unsafe {
                let msg = CStr::from_ptr(get_error_message(error));
                println!("Error: {}", msg.to_string_lossy());
            }
        }
    }
    
    match safe_divide(10, 0, &mut result) {
        CResult::Success => println!("10 / 0 = {}", result),
        error => {
            unsafe {
                let msg = CStr::from_ptr(get_error_message(error));
                println!("Error: {}", msg.to_string_lossy());
            }
        }
    }
    
    let data = [1, 2, 3, 4, 5];
    let mut index = 0;
    
    match find_in_array(data.as_ptr(), data.len(), 3, &mut index) {
        CResult::Success => println!("Found value 3 at index {}", index),
        error => println!("Search failed: {:?}", error),
    }
    
    println!("✅ Exercise 3.5 complete\n");
}

// Exercise 3.6: Fix memory ownership across FFI
fn exercise_3_6() {
    println!("Exercise 3.6: Fix memory ownership");
    
    // FIXME: This function doesn't properly manage memory ownership
    #[no_mangle]
    pub extern "C" fn create_string_array(count: usize) -> *mut *mut c_char {
        let mut ptrs = Vec::with_capacity(count);
        
        for i in 0..count {
            let s = format!("String {}", i);
            let c_string = CString::new(s).unwrap();
            ptrs.push(c_string.into_raw());  // Transfer ownership to C
        }
        
        // PROBLEM: Vec will be dropped, but we need to return the pointer!
        ptrs.as_mut_ptr()  // COMPILE ERROR: Returning pointer to dropped Vec
    }
    
    // TODO: Fix memory management
    #[no_mangle]
    pub extern "C" fn create_string_array_safe(count: usize) -> *mut *mut c_char {
        if count == 0 {
            return ptr::null_mut();
        }
        
        let mut ptrs = Vec::with_capacity(count);
        
        for i in 0..count {
            let s = format!("String {}", i);
            let c_string = CString::new(s).unwrap();
            ptrs.push(c_string.into_raw());
        }
        
        // Transfer ownership of the Vec to C
        let ptr = ptrs.as_mut_ptr();
        std::mem::forget(ptrs);  // Don't drop the Vec
        ptr
    }
    
    // TODO: Provide cleanup function
    #[no_mangle]
    pub extern "C" fn free_string_array(ptrs: *mut *mut c_char, count: usize) {
        if ptrs.is_null() || count == 0 {
            return;
        }
        
        unsafe {
            // First, free each individual string
            for i in 0..count {
                let str_ptr = *ptrs.add(i);
                if !str_ptr.is_null() {
                    let _ = CString::from_raw(str_ptr);  // Reconstruct and drop
                }
            }
            
            // Then, free the array of pointers
            let _ = Vec::from_raw_parts(ptrs, count, count);
        }
    }
    
    // TODO: Safe wrapper for creating byte arrays
    #[no_mangle]
    pub extern "C" fn create_byte_array(size: usize, init_value: u8) -> *mut u8 {
        if size == 0 {
            return ptr::null_mut();
        }
        
        let mut vec = vec![init_value; size];
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec);
        ptr
    }
    
    #[no_mangle]
    pub extern "C" fn free_byte_array(ptr: *mut u8, size: usize) {
        if !ptr.is_null() && size > 0 {
            unsafe {
                let _ = Vec::from_raw_parts(ptr, size, size);
            }
        }
    }
    
    // Test memory management
    let string_array = create_string_array_safe(3);
    if !string_array.is_null() {
        println!("Created string array with 3 strings");
        
        unsafe {
            for i in 0..3 {
                let str_ptr = *string_array.add(i);
                if !str_ptr.is_null() {
                    let c_str = CStr::from_ptr(str_ptr);
                    println!("String {}: {}", i, c_str.to_string_lossy());
                }
            }
        }
        
        free_string_array(string_array, 3);
        println!("Freed string array");
    }
    
    let byte_array = create_byte_array(10, 42);
    if !byte_array.is_null() {
        unsafe {
            let slice = std::slice::from_raw_parts(byte_array, 10);
            println!("Byte array: {:?}", slice);
        }
        free_byte_array(byte_array, 10);
        println!("Freed byte array");
    }
    
    println!("✅ Exercise 3.6 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_c_string_conversion() {
        let rust_str = "Hello, World!";
        let c_string = safe_string_to_c(rust_str);
        
        unsafe {
            if let Ok(converted) = safe_c_to_string(c_string.as_ptr()) {
                assert_eq!(converted, rust_str);
            } else {
                panic!("String conversion failed");
            }
        }
    }
    
    #[test]
    fn test_c_compatible_structs() {
        let point = create_point(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
        
        let (person, _holder) = CPerson::new("Test", 25, 5.8);
        assert_eq!(person.age, 25);
        assert_eq!(person.height, 5.8);
    }
    
    #[test]
    fn test_error_handling() {
        let mut result = 0;
        
        assert_eq!(safe_divide(10, 2, &mut result), CResult::Success);
        assert_eq!(result, 5);
        
        assert_eq!(safe_divide(10, 0, &mut result), CResult::DivisionByZero);
        
        assert_eq!(safe_divide(10, 2, ptr::null_mut()), CResult::InvalidInput);
    }
    
    #[test]
    fn test_memory_management() {
        let byte_array = create_byte_array(5, 100);
        assert!(!byte_array.is_null());
        
        unsafe {
            let slice = std::slice::from_raw_parts(byte_array, 5);
            assert_eq!(slice, &[100, 100, 100, 100, 100]);
        }
        
        free_byte_array(byte_array, 5);
    }
}

// COMPILATION CHALLENGES:
// 1. Fix extern function declaration syntax errors
// 2. Add unsafe blocks around all FFI calls
// 3. Add #[repr(C)] to structs for C compatibility
// 4. Use proper C-compatible types (c_int, c_char, etc.)
// 5. Implement proper error handling with C-compatible enums
// 6. Fix memory ownership transfer between Rust and C
//
// LEARNING OBJECTIVES:
// - Master C function declarations and calling conventions
// - Safely convert between Rust and C strings
// - Create C-compatible struct layouts
// - Implement callback functions for C consumption
// - Handle errors across FFI boundaries properly
// - Manage memory ownership between Rust and C
//
// C# COMPARISON:
// C#: [DllImport("lib.dll")] extern static int func(string s);
// Rust: extern "C" { fn func(s: *const c_char) -> c_int; }
//
// C#: Marshal.StringToHGlobalAnsi(str)             // P/Invoke marshaling
// Rust: CString::new(str).unwrap()                 // Explicit C string creation
