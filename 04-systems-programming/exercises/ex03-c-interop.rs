// Exercise 3: C Interoperability (FFI)
//
// This exercise demonstrates how to safely interface with C libraries,
// handle C strings, and create safe wrappers around unsafe FFI calls.

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::raw::{c_char, c_int, c_double, c_void};
use std::ptr;

fn main() {
    println!("=== Exercise 3: C Interoperability (FFI) ===\n");
    
    exercise_3_1();
    exercise_3_2();
    exercise_3_3();
    exercise_3_4();
    exercise_3_5();
}

// Exercise 3.1: Basic C function calls
fn exercise_3_1() {
    println!("Exercise 3.1: Basic C Function Calls");
    
    // Declare external C functions
    extern "C" {
        fn strlen(s: *const c_char) -> usize;
        fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
        fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
        fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    }
    
    // TODO: Create safe wrappers for C string functions
    
    fn safe_strlen(s: &str) -> usize {
        // TODO: Convert Rust string to C string and call strlen
        let c_string = CString::new(s).expect("CString::new failed");
        unsafe {
            strlen(c_string.as_ptr())
        }
    }
    
    fn safe_strcmp(s1: &str, s2: &str) -> i32 {
        // TODO: Compare two Rust strings using C strcmp
        let c_s1 = CString::new(s1).expect("CString::new failed");
        let c_s2 = CString::new(s2).expect("CString::new failed");
        
        unsafe {
            strcmp(c_s1.as_ptr(), c_s2.as_ptr())
        }
    }
    
    fn safe_string_concat(s1: &str, s2: &str) -> String {
        // TODO: Concatenate strings using C functions
        let total_len = s1.len() + s2.len() + 1; // +1 for null terminator
        let mut buffer = vec![0u8; total_len];
        
        let c_s1 = CString::new(s1).expect("CString::new failed");
        let c_s2 = CString::new(s2).expect("CString::new failed");
        
        unsafe {
            strcpy(buffer.as_mut_ptr() as *mut c_char, c_s1.as_ptr());
            strcat(buffer.as_mut_ptr() as *mut c_char, c_s2.as_ptr());
            
            let result_ptr = buffer.as_ptr() as *const c_char;
            CStr::from_ptr(result_ptr).to_string_lossy().into_owned()
        }
    }
    
    // Test the safe wrappers
    let test_str = "Hello, World!";
    println!("Length of '{}': {}", test_str, safe_strlen(test_str));
    
    let str1 = "apple";
    let str2 = "banana";
    let comparison = safe_strcmp(str1, str2);
    println!("'{}' compared to '{}': {}", str1, str2, comparison);
    
    let concatenated = safe_string_concat("Hello, ", "FFI!");
    println!("Concatenated: '{}'", concatenated);
    
    println!("✅ Exercise 3.1 complete\n");
}

// Exercise 3.2: Math library integration
fn exercise_3_2() {
    println!("Exercise 3.2: Math Library Integration");
    
    // Link with the math library
    #[link(name = "m")]
    extern "C" {
        fn sin(x: c_double) -> c_double;
        fn cos(x: c_double) -> c_double;
        fn sqrt(x: c_double) -> c_double;
        fn pow(base: c_double, exp: c_double) -> c_double;
        fn log(x: c_double) -> c_double;
        fn exp(x: c_double) -> c_double;
        fn fabs(x: c_double) -> c_double;
    }
    
    // TODO: Create safe math wrapper module with error handling
    
    #[derive(Debug)]
    pub enum MathError {
        DomainError(String),
        RangeError(String),
    }
    
    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MathError::DomainError(msg) => write!(f, "Domain error: {}", msg),
                MathError::RangeError(msg) => write!(f, "Range error: {}", msg),
            }
        }
    }
    
    impl std::error::Error for MathError {}
    
    pub mod safe_math {
        use super::*;
        
        pub fn sine(x: f64) -> f64 {
            // TODO: Safe wrapper for sin
            unsafe { sin(x) }
        }
        
        pub fn cosine(x: f64) -> f64 {
            // TODO: Safe wrapper for cos
            unsafe { cos(x) }
        }
        
        pub fn square_root(x: f64) -> Result<f64, MathError> {
            // TODO: Domain checking for sqrt
            if x < 0.0 {
                Err(MathError::DomainError("Cannot take square root of negative number".to_string()))
            } else {
                Ok(unsafe { sqrt(x) })
            }
        }
        
        pub fn power(base: f64, exponent: f64) -> Result<f64, MathError> {
            // TODO: Range checking for pow
            let result = unsafe { pow(base, exponent) };
            
            if result.is_infinite() && !base.is_infinite() && !exponent.is_infinite() {
                Err(MathError::RangeError("Result overflow".to_string()))
            } else if result.is_nan() && !base.is_nan() && !exponent.is_nan() {
                Err(MathError::DomainError("Invalid operation".to_string()))
            } else {
                Ok(result)
            }
        }
        
        pub fn natural_log(x: f64) -> Result<f64, MathError> {
            // TODO: Domain checking for log
            if x <= 0.0 {
                Err(MathError::DomainError("Logarithm of non-positive number".to_string()))
            } else {
                Ok(unsafe { log(x) })
            }
        }
        
        pub fn exponential(x: f64) -> Result<f64, MathError> {
            // TODO: Range checking for exp
            let result = unsafe { exp(x) };
            
            if result.is_infinite() && !x.is_infinite() {
                Err(MathError::RangeError("Exponential overflow".to_string()))
            } else {
                Ok(result)
            }
        }
        
        pub fn absolute(x: f64) -> f64 {
            // TODO: Safe wrapper for fabs
            unsafe { fabs(x) }
        }
    }
    
    // Test the math functions
    use std::f64::consts::PI;
    
    println!("sin(π/2) = {}", safe_math::sine(PI / 2.0));
    println!("cos(0) = {}", safe_math::cosine(0.0));
    
    match safe_math::square_root(16.0) {
        Ok(result) => println!("√16 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_math::square_root(-4.0) {
        Ok(result) => println!("√(-4) = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_math::power(2.0, 8.0) {
        Ok(result) => println!("2^8 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_math::natural_log(2.718281828) {
        Ok(result) => println!("ln(e) ≈ {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("|-42.5| = {}", safe_math::absolute(-42.5));
    
    println!("✅ Exercise 3.2 complete\n");
}

// Exercise 3.3: Memory management with C
fn exercise_3_3() {
    println!("Exercise 3.3: Memory Management with C");
    
    extern "C" {
        fn malloc(size: usize) -> *mut c_void;
        fn free(ptr: *mut c_void);
        fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
        fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    }
    
    // TODO: Create RAII wrapper for C malloc/free
    
    pub struct CMalloc {
        ptr: *mut u8,
        size: usize,
    }
    
    impl CMalloc {
        pub fn new(size: usize) -> Result<Self, &'static str> {
            // TODO: Allocate memory with malloc
            let ptr = unsafe { malloc(size) as *mut u8 };
            
            if ptr.is_null() {
                Err("Memory allocation failed")
            } else {
                Ok(CMalloc { ptr, size })
            }
        }
        
        pub fn zero(&mut self) {
            // TODO: Zero the memory using memset
            unsafe {
                memset(self.ptr as *mut c_void, 0, self.size);
            }
        }
        
        pub fn copy_from(&mut self, data: &[u8]) -> Result<(), &'static str> {
            // TODO: Copy data using memcpy
            if data.len() > self.size {
                return Err("Data too large for buffer");
            }
            
            unsafe {
                memcpy(
                    self.ptr as *mut c_void,
                    data.as_ptr() as *const c_void,
                    data.len()
                );
            }
            Ok(())
        }
        
        pub fn as_slice(&self) -> &[u8] {
            // TODO: Return slice view
            unsafe {
                std::slice::from_raw_parts(self.ptr, self.size)
            }
        }
        
        pub fn as_mut_slice(&mut self) -> &mut [u8] {
            // TODO: Return mutable slice view
            unsafe {
                std::slice::from_raw_parts_mut(self.ptr, self.size)
            }
        }
        
        pub fn size(&self) -> usize {
            self.size
        }
    }
    
    impl Drop for CMalloc {
        fn drop(&mut self) {
            // TODO: Free memory
            if !self.ptr.is_null() {
                unsafe {
                    free(self.ptr as *mut c_void);
                }
            }
        }
    }
    
    // Test the C memory wrapper
    let mut buffer = CMalloc::new(64).expect("Failed to allocate memory");
    println!("Allocated {} bytes", buffer.size());
    
    // Zero the buffer
    buffer.zero();
    let slice = buffer.as_slice();
    assert!(slice.iter().all(|&b| b == 0));
    println!("Buffer zeroed successfully");
    
    // Copy data
    let test_data = b"Hello from C malloc!";
    buffer.copy_from(test_data).expect("Failed to copy data");
    
    let data_slice = &buffer.as_slice()[..test_data.len()];
    assert_eq!(data_slice, test_data);
    println!("Data copied: {:?}", std::str::from_utf8(data_slice).unwrap());
    
    // Modify through mutable slice
    let mut_slice = buffer.as_mut_slice();
    mut_slice[0] = b'h'; // Change 'H' to 'h'
    
    println!("Modified data: {:?}", std::str::from_utf8(&mut_slice[..test_data.len()]).unwrap());
    
    println!("✅ Exercise 3.3 complete\n");
}

// Exercise 3.4: Exposing Rust functions to C
fn exercise_3_4() {
    println!("Exercise 3.4: Exposing Rust Functions to C");
    
    // TODO: Create C-compatible functions that can be called from C code
    
    #[no_mangle]
    pub extern "C" fn rust_add_integers(a: c_int, b: c_int) -> c_int {
        // TODO: Simple arithmetic
        a + b
    }
    
    #[no_mangle]
    pub extern "C" fn rust_string_length(s: *const c_char) -> c_int {
        // TODO: Safe string length calculation
        if s.is_null() {
            return -1;
        }
        
        unsafe {
            match CStr::from_ptr(s).to_str() {
                Ok(rust_str) => rust_str.len() as c_int,
                Err(_) => -1,
            }
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_process_array(
        input: *const c_int,
        length: usize,
        output: *mut c_int
    ) -> c_int {
        // TODO: Process array, return sum and double each element
        if input.is_null() || output.is_null() {
            return -1;
        }
        
        unsafe {
            let input_slice = std::slice::from_raw_parts(input, length);
            let output_slice = std::slice::from_raw_parts_mut(output, length);
            
            let mut sum = 0;
            for (i, &value) in input_slice.iter().enumerate() {
                sum += value;
                output_slice[i] = value * 2;
            }
            
            sum
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_create_string(input: *const c_char) -> *mut c_char {
        // TODO: Create new string with processing
        if input.is_null() {
            return ptr::null_mut();
        }
        
        unsafe {
            let c_str = CStr::from_ptr(input);
            let rust_str = match c_str.to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            };
            
            let processed = format!("Processed: {}", rust_str.to_uppercase());
            
            match CString::new(processed) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_free_string(s: *mut c_char) {
        // TODO: Free string created by rust_create_string
        if !s.is_null() {
            unsafe {
                let _ = CString::from_raw(s);
            }
        }
    }
    
    // Callback function support
    type ProcessCallback = extern "C" fn(c_int) -> c_int;
    
    #[no_mangle]
    pub extern "C" fn rust_map_with_callback(
        input: *const c_int,
        length: usize,
        output: *mut c_int,
        callback: Option<ProcessCallback>
    ) -> c_int {
        // TODO: Apply callback to each element
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
        
        0 // Success
    }
    
    // Test the exported functions (simulating C calls)
    
    // Test integer addition
    let result = rust_add_integers(42, 58);
    println!("42 + 58 = {}", result);
    
    // Test string length
    let test_str = CString::new("Hello, FFI!").unwrap();
    let length = rust_string_length(test_str.as_ptr());
    println!("String length: {}", length);
    
    // Test array processing
    let input_array = [1, 2, 3, 4, 5];
    let mut output_array = [0; 5];
    
    let sum = rust_process_array(
        input_array.as_ptr(),
        input_array.len(),
        output_array.as_mut_ptr()
    );
    
    println!("Input: {:?}", input_array);
    println!("Output: {:?}", output_array);
    println!("Sum: {}", sum);
    
    // Test string creation and cleanup
    let input_str = CString::new("hello world").unwrap();
    let processed_ptr = rust_create_string(input_str.as_ptr());
    
    if !processed_ptr.is_null() {
        unsafe {
            let processed_str = CStr::from_ptr(processed_ptr);
            println!("Processed string: {:?}", processed_str.to_string_lossy());
        }
        rust_free_string(processed_ptr);
    }
    
    // Test callback functionality
    extern "C" fn double_value(x: c_int) -> c_int {
        x * 2
    }
    
    extern "C" fn square_value(x: c_int) -> c_int {
        x * x
    }
    
    let callback_input = [1, 2, 3, 4, 5];
    let mut callback_output = [0; 5];
    
    let result = rust_map_with_callback(
        callback_input.as_ptr(),
        callback_input.len(),
        callback_output.as_mut_ptr(),
        Some(square_value)
    );
    
    if result == 0 {
        println!("Callback mapping successful");
        println!("Input: {:?}", callback_input);
        println!("Squared: {:?}", callback_output);
    }
    
    println!("✅ Exercise 3.4 complete\n");
}

// Exercise 3.5: Complex library integration
fn exercise_3_5() {
    println!("Exercise 3.5: Complex Library Integration");
    
    // TODO: Create a more complex integration example
    // This simulates integrating with a hypothetical C graphics library
    
    // Simulate C graphics API
    extern "C" {
        // These would normally be from a real C library
        // For this exercise, we'll implement them in Rust with C ABI
    }
    
    // Define C-compatible structures
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        x: c_double,
        y: c_double,
    }
    
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Color {
        r: c_double,
        g: c_double,
        b: c_double,
        a: c_double,
    }
    
    #[repr(C)]
    pub struct GraphicsContext {
        _private: [u8; 0], // Opaque handle
    }
    
    // Mock C API implementations (normally these would be external)
    #[no_mangle]
    pub extern "C" fn graphics_create_context() -> *mut GraphicsContext {
        // TODO: Simulate context creation
        Box::into_raw(Box::new(GraphicsContext { _private: [] }))
    }
    
    #[no_mangle]
    pub extern "C" fn graphics_destroy_context(ctx: *mut GraphicsContext) {
        // TODO: Simulate context destruction
        if !ctx.is_null() {
            unsafe {
                let _ = Box::from_raw(ctx);
            }
        }
    }
    
    #[no_mangle]
    pub extern "C" fn graphics_draw_line(
        ctx: *mut GraphicsContext,
        start: Point,
        end: Point,
        color: Color
    ) -> c_int {
        // TODO: Simulate drawing
        if ctx.is_null() {
            return -1;
        }
        
        println!("Drawing line from ({:.1}, {:.1}) to ({:.1}, {:.1}) with color RGBA({:.1}, {:.1}, {:.1}, {:.1})",
                 start.x, start.y, end.x, end.y, color.r, color.g, color.b, color.a);
        0
    }
    
    #[no_mangle]
    pub extern "C" fn graphics_draw_circle(
        ctx: *mut GraphicsContext,
        center: Point,
        radius: c_double,
        color: Color
    ) -> c_int {
        // TODO: Simulate circle drawing
        if ctx.is_null() {
            return -1;
        }
        
        println!("Drawing circle at ({:.1}, {:.1}) with radius {:.1} and color RGBA({:.1}, {:.1}, {:.1}, {:.1})",
                 center.x, center.y, radius, color.r, color.g, color.b, color.a);
        0
    }
    
    // Safe Rust wrapper
    pub struct Graphics {
        ctx: *mut GraphicsContext,
    }
    
    impl Graphics {
        pub fn new() -> Result<Self, &'static str> {
            // TODO: Create safe wrapper
            let ctx = graphics_create_context();
            if ctx.is_null() {
                Err("Failed to create graphics context")
            } else {
                Ok(Graphics { ctx })
            }
        }
        
        pub fn draw_line(&self, start: Point, end: Point, color: Color) -> Result<(), &'static str> {
            // TODO: Safe line drawing
            let result = graphics_draw_line(self.ctx, start, end, color);
            if result == 0 {
                Ok(())
            } else {
                Err("Failed to draw line")
            }
        }
        
        pub fn draw_circle(&self, center: Point, radius: f64, color: Color) -> Result<(), &'static str> {
            // TODO: Safe circle drawing
            let result = graphics_draw_circle(self.ctx, center, radius, color);
            if result == 0 {
                Ok(())
            } else {
                Err("Failed to draw circle")
            }
        }
        
        pub fn draw_rectangle(&self, top_left: Point, bottom_right: Point, color: Color) -> Result<(), &'static str> {
            // TODO: Draw rectangle using lines
            let top_right = Point { x: bottom_right.x, y: top_left.y };
            let bottom_left = Point { x: top_left.x, y: bottom_right.y };
            
            self.draw_line(top_left, top_right, color)?;
            self.draw_line(top_right, bottom_right, color)?;
            self.draw_line(bottom_right, bottom_left, color)?;
            self.draw_line(bottom_left, top_left, color)?;
            
            Ok(())
        }
    }
    
    impl Drop for Graphics {
        fn drop(&mut self) {
            graphics_destroy_context(self.ctx);
        }
    }
    
    // Test the graphics wrapper
    let graphics = Graphics::new().expect("Failed to create graphics context");
    
    let red = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    let blue = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    let green = Color { r: 0.0, g: 1.0, b: 0.0, a: 0.5 };
    
    let start = Point { x: 0.0, y: 0.0 };
    let end = Point { x: 100.0, y: 100.0 };
    
    graphics.draw_line(start, end, red).expect("Failed to draw line");
    
    let center = Point { x: 50.0, y: 50.0 };
    graphics.draw_circle(center, 25.0, blue).expect("Failed to draw circle");
    
    let top_left = Point { x: 10.0, y: 10.0 };
    let bottom_right = Point { x: 90.0, y: 90.0 };
    graphics.draw_rectangle(top_left, bottom_right, green).expect("Failed to draw rectangle");
    
    println!("Graphics operations completed successfully");
    println!("✅ Exercise 3.5 complete\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_length() {
        extern "C" {
            fn strlen(s: *const c_char) -> usize;
        }
        
        let test_str = CString::new("Hello").unwrap();
        let length = unsafe { strlen(test_str.as_ptr()) };
        assert_eq!(length, 5);
    }
    
    #[test]
    fn test_math_functions() {
        use exercise_3_2::safe_math;
        
        assert!(safe_math::square_root(4.0).unwrap() - 2.0 < f64::EPSILON);
        assert!(safe_math::square_root(-1.0).is_err());
        assert!(safe_math::natural_log(0.0).is_err());
    }
    
    #[test]
    fn test_c_malloc_wrapper() {
        let mut buffer = CMalloc::new(100).unwrap();
        buffer.zero();
        
        let data = b"test data";
        buffer.copy_from(data).unwrap();
        
        let slice = buffer.as_slice();
        assert_eq!(&slice[..data.len()], data);
    }
    
    #[test]
    fn test_exported_functions() {
        assert_eq!(rust_add_integers(2, 3), 5);
        
        let test_str = CString::new("test").unwrap();
        assert_eq!(rust_string_length(test_str.as_ptr()), 4);
        assert_eq!(rust_string_length(ptr::null()), -1);
    }
    
    #[test]
    fn test_graphics_wrapper() {
        let graphics = Graphics::new().unwrap();
        let color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        let point = Point { x: 0.0, y: 0.0 };
        
        assert!(graphics.draw_circle(point, 10.0, color).is_ok());
    }
}
