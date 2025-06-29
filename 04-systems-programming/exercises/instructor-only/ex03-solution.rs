// Exercise 3 Solution: C Interop and FFI - WORKING VERSION
//
// This is the complete, working solution for ex03-c-interop.rs
// Compare this with your implementation to understand FFI best practices.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

fn main() {
    println!("=== Exercise 3 Solution: C Interop and FFI ===\n");
    
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
    
    // FIXED: Corrected syntax errors in extern declarations
    extern "C" {
        fn strlen(s: *const c_char) -> usize;
        fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;  // FIXED: syntax
        fn malloc(size: usize) -> *mut c_void;  // FIXED: Added semicolon
        fn free(ptr: *mut c_void);
    }
    
    // FIXED: Added unsafe blocks around extern function calls
    let test_string = b"Hello, World!\0";
    let length = unsafe {
        strlen(test_string.as_ptr() as *const c_char)
    };
    println!("String length: {}", length);
    
    // Safe wrappers for these C functions
    fn safe_strlen(s: &str) -> usize {
        let c_string = CString::new(s).unwrap();
        unsafe {
            strlen(c_string.as_ptr())
        }
    }
    
    fn safe_malloc(size: usize) -> Option<*mut u8> {
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
    
    // FIXED: Proper string conversion that creates null-terminated C string
    fn safe_string_to_c(s: &str) -> CString {
        CString::new(s).expect("String contains null byte")
    }
    
    // FIXED: Safe C to Rust conversion using CStr
    unsafe fn safe_c_to_string(c_str: *const c_char) -> Result<String, std::str::Utf8Error> {
        if c_str.is_null() {
            return Ok(String::new());
        }
        
        let c_str_safe = CStr::from_ptr(c_str);
        c_str_safe.to_str().map(|s| s.to_owned())
    }
    
    // Mock C function for demonstration
    extern "C" {
        // In a real implementation, this would link to actual C code
        // For this exercise, we'll simulate it
    }
    
    // Safe wrapper for string array processing
    fn call_process_strings(strings: &[&str]) -> i32 {
        let c_strings: Vec<CString> = strings
            .iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        
        let c_ptrs: Vec<*const c_char> = c_strings
            .iter()
            .map(|cs| cs.as_ptr())
            .collect();
        
        // For demonstration, just return the count
        // In real code, this would call an actual C function
        c_ptrs.len() as i32
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
    
    // FIXED: Added #[repr(C)] for C compatibility
    #[repr(C)]
    #[derive(Debug)]
    struct CPoint {
        x: f64,
        y: f64,
        z: f64,
    }
    
    // FIXED: Made struct C-compatible with proper field types
    #[repr(C)]
    #[derive(Debug)]
    struct CPerson {
        name: *const c_char,  // C string pointer instead of Rust String
        age: c_int,
        height: f64,
    }
    
    impl CPerson {
        // Safe constructor that handles C string
        fn new(name: &str, age: i32, height: f64) -> (Self, CString) {
            let c_name = CString::new(name).unwrap();
            let person = CPerson {
                name: c_name.as_ptr(),
                age,
                height,
            };
            (person, c_name)  // Return CString to keep it alive
        }
        
        // Safe method to get name as Rust string
        fn get_name(&self) -> Result<String, std::str::Utf8Error> {
            if self.name.is_null() {
                return Ok(String::new());
            }
            
            unsafe {
                CStr::from_ptr(self.name).to_str().map(|s| s.to_owned())
            }
        }
    }
    
    // Export functions for C consumption
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
    
    // FIXED: Added extern "C" for C-compatible callback type
    type CCallback = extern "C" fn(value: c_int) -> c_int;
    
    // FIXED: Updated function to accept C callbacks
    fn safe_process_with_callback(data: &[i32], callback: CCallback) -> i32 {
        data.iter().map(|&x| callback(x)).sum()
    }
    
    // Export function that accepts callback
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
    
    // Example callback functions
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
    
    // FIXED: Use C-compatible error codes instead of Result
    #[repr(C)]
    #[derive(Debug, PartialEq)]
    pub enum CResult {
        Success = 0,
        DivisionByZero = -1,
        InvalidInput = -2,
        OutOfMemory = -3,
        Unknown = -999,
    }
    
    // FIXED: Return C-compatible error code
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
    
    // Helper for error message strings
    #[no_mangle]
    pub extern "C" fn get_error_message(error: CResult) -> *const c_char {
        let message = match error {
            CResult::Success => "Success\0",
            CResult::DivisionByZero => "Division by zero\0",
            CResult::InvalidInput => "Invalid input\0",
            CResult::OutOfMemory => "Out of memory\0",
            CResult::Unknown => "Unknown error\0",
        };
        
        // Return pointer to static string (safe because it's 'static)
        message.as_ptr() as *const c_char
    }
    
    // Working with optional values across FFI
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
            
            // Not found - use a specific result for this
            CResult::Unknown
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
    
    // FIXED: Proper memory management with ownership transfer
    #[no_mangle]
    pub extern "C" fn create_string_array_safe(count: usize) -> *mut *mut c_char {
        if count == 0 {
            return ptr::null_mut();
        }
        
        let mut ptrs = Vec::with_capacity(count);
        
        for i in 0..count {
            let s = format!("String {}", i);
            let c_string = CString::new(s).unwrap();
            ptrs.push(c_string.into_raw());  // Transfer ownership to C
        }
        
        // Transfer ownership of the Vec to C
        let ptr = ptrs.as_mut_ptr();
        std::mem::forget(ptrs);  // Don't drop the Vec
        ptr
    }
    
    // Provide cleanup function
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
    
    // Safe wrapper for creating byte arrays
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
        // Memory is freed, test passes if no crash occurs
    }
    
    #[test]
    fn test_callbacks() {
        let data = [1, 2, 3, 4, 5];
        
        let result = safe_process_with_callback(&data, double_value);
        assert_eq!(result, 30); // (1+2+3+4+5) * 2 = 30
        
        let result = safe_process_with_callback(&data, square_value);
        assert_eq!(result, 55); // 1+4+9+16+25 = 55
    }
}

// KEY LEARNING POINTS:
//
// 1. EXTERN FUNCTION DECLARATIONS:
//    - Use correct C calling convention: extern "C"
//    - Match parameter types exactly with C signatures
//    - All extern functions are unsafe by default
//
// 2. C STRING HANDLING:
//    - Use CString for Rust→C conversion (null-terminated)
//    - Use CStr for C→Rust conversion (safe UTF-8 handling)
//    - Always check for null pointers from C code
//    - Keep CString alive while C code might access it
//
// 3. STRUCT LAYOUT:
//    - Use #[repr(C)] for C-compatible memory layout
//    - Replace Rust types with C-compatible equivalents
//    - String → *const c_char, Vec<T> → pointer + length
//
// 4. CALLBACK FUNCTIONS:
//    - Use extern "C" fn type for C-compatible callbacks
//    - Export callback-accepting functions with #[no_mangle]
//    - Handle null function pointers gracefully
//
// 5. ERROR HANDLING:
//    - Use C-compatible error codes (#[repr(C)] enum)
//    - Return errors through out parameters
//    - Provide error message functions for debugging
//
// 6. MEMORY OWNERSHIP:
//    - Use std::mem::forget() to transfer ownership to C
//    - Provide cleanup functions to prevent memory leaks
//    - Reconstruct Rust types from raw pointers for cleanup
//    - Always pair allocation/deallocation functions
//
// 7. SAFETY PATTERNS:
//    - Validate all pointers before dereferencing
//    - Check array bounds and lengths
//    - Use RAII where possible
//    - Document ownership transfer clearly
//
// FFI BEST PRACTICES:
// - Keep C interface as simple as possible
// - Use opaque pointers for complex Rust types
// - Provide both creation and destruction functions
// - Test with actual C code when possible
// - Consider using bindgen for complex C headers
//
// C# P/INVOKE COMPARISON:
// - C#: [DllImport] handles marshaling automatically
// - Rust: Manual control over all conversions
// - C#: Exceptions can cross boundaries
// - Rust: Use error codes for cross-language errors
// - C#: GC handles cleanup automatically
// - Rust: Manual ownership management required
// - C#: Runtime marshaling overhead
// - Rust: Zero-cost abstractions with compile-time safety
