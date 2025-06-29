// Exercise 3: Custom Error Types - Fix the Broken Code!
//
// Your task: Make all error types compile and work correctly by fixing
// compilation errors and implementing comprehensive error handling.
//
// INSTRUCTIONS:
// 1. Read each TODO and FIXME comment carefully
// 2. Fix the compilation errors one by one
// 3. Run `rustc ex03-error-types.rs && ./ex03-error-types` to test
// 4. Understand how to design custom error types in Rust

use std::fmt;
use std::error::Error;
use std::num::ParseIntError;
use std::io;

fn main() {
    println!("=== Exercise 3: Custom Error Types (Fix the Code!) ===\n");
    
    // CHALLENGE: Uncomment each section as you implement the error types
    test_validation_errors();
    // test_network_errors();
    // test_database_errors();
    // test_application_errors();
    // test_error_conversion();
    // test_error_chains();
    
    println!("\n🎉 All error handling exercises completed!");
}

fn test_validation_errors() {
    println!("Testing validation errors...");
    
    // Exercise 3.1: User validation
    let test_cases = [
        ("", "valid@email.com", 25),
        ("John", "", 25),
        ("John", "invalid-email", 25),
        ("John", "valid@email.com", 200),
        ("John", "valid@email.com", 25),
    ];
    
    for (name, email, age) in test_cases {
        match validate_user(name, email, age) {  // COMPILE ERROR: Function not implemented!
            Ok(user) => println!("✅ Valid user: {:?}", user),
            Err(e) => println!("❌ Validation error: {}", e),
        }
    }
}

fn test_network_errors() {
    println!("Testing network errors...");
    
    // Simulate different network scenarios
    let responses = [
        simulate_http_request("https://api.example.com/users"),  // COMPILE ERROR!
        simulate_http_request("https://slow-api.example.com/data"),
        simulate_http_request("https://broken-api.example.com/error"),
    ];
    
    for result in responses {
        match result {
            Ok(data) => println!("✅ Request successful: {}", data),
            Err(e) => {
                println!("❌ Request failed: {}", e);
                // TODO: Handle different error types differently
                // HINT: Use match on error variants
            }
        }
    }
}

fn test_database_errors() {
    println!("Testing database errors...");
    
    // Simulate database operations
    let user_ids = [1, 2, 999, 42];
    
    for id in user_ids {
        match get_user_from_database(id) {  // COMPILE ERROR: Function not implemented!
            Ok(user) => println!("✅ Found user: {}", user),
            Err(e) => {
                println!("❌ Database error: {}", e);
                
                // TODO: Implement error recovery based on error type
                // HINT: Different strategies for different errors
                handle_database_error(&e);  // COMPILE ERROR: Function not implemented!
            }
        }
    }
}

fn test_application_errors() {
    println!("Testing application errors...");
    
    // Complex operation that can fail in multiple ways
    let file_operations = [
        "read_config.json",
        "process_data.csv",
        "nonexistent_file.txt",
        "corrupted_data.json",
    ];
    
    for filename in file_operations {
        match process_file(filename) {  // COMPILE ERROR: Function not implemented!
            Ok(result) => println!("✅ File processed: {}", result),
            Err(e) => {
                println!("❌ Processing failed: {}", e);
                print_error_chain(&e);  // COMPILE ERROR: Function not implemented!
            }
        }
    }
}

fn test_error_conversion() {
    println!("Testing error conversion...");
    
    // Test automatic error conversion with ?
    let inputs = ["42", "not_a_number", "123", "456"];
    
    for input in inputs {
        match parse_and_validate(input) {  // COMPILE ERROR: Function not implemented!
            Ok(value) => println!("✅ Parsed and validated: {}", value),
            Err(e) => println!("❌ Error: {}", e),
        }
    }
}

fn test_error_chains() {
    println!("Testing error chains...");
    
    // Test complex operations with multiple failure points
    match complex_operation() {  // COMPILE ERROR: Function not implemented!
        Ok(result) => println!("✅ Complex operation succeeded: {}", result),
        Err(e) => {
            println!("❌ Complex operation failed: {}", e);
            
            // Walk the error chain
            let mut source = e.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("  {} Caused by: {}", "└─".repeat(level), err);
                source = err.source();
                level += 1;
            }
        }
    }
}

// ============================================================================
// TODO: Implement all the error types and functions below
// ============================================================================

// Exercise 3.1: Validation error type
#[derive(Debug)]
enum ValidationError {
    // TODO: Define validation error variants
    // HINT: EmptyName, InvalidEmail, InvalidAge(u32), TooYoung, TooOld
}

// FIXME: Implement Display trait for ValidationError
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Match on self and write appropriate error messages
        // HINT: Use write! macro to format messages
        todo!("Implement Display for ValidationError")
    }
}

// FIXME: Implement Error trait for ValidationError
impl Error for ValidationError {}

// TODO: Define User struct
#[derive(Debug)]
struct User {
    // TODO: Add fields for name, email, age
}

// TODO: Implement user validation function
fn validate_user(name: &str, email: &str, age: u32) -> Result<User, ValidationError> {
    // TODO: Validate each field and return appropriate errors
    // HINT: Check for empty name, valid email format, reasonable age
    todo!("Implement user validation")
}

// Exercise 3.2: Network error type
#[derive(Debug)]
enum NetworkError {
    // TODO: Define network error variants
    // HINT: Timeout, ConnectionFailed(String), HttpError(u16), InvalidUrl(String)
}

// FIXME: Implement Display for NetworkError
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Format network error messages
        todo!("Implement Display for NetworkError")
    }
}

// FIXME: Implement Error trait for NetworkError
impl Error for NetworkError {}

// TODO: Simulate HTTP request function
fn simulate_http_request(url: &str) -> Result<String, NetworkError> {
    // TODO: Simulate different failure scenarios based on URL
    // HINT: Different URLs can trigger different errors
    todo!("Implement HTTP request simulation")
}

// Exercise 3.3: Database error type  
#[derive(Debug)]
enum DatabaseError {
    // TODO: Define database error variants
    // HINT: ConnectionFailed, UserNotFound(u32), QueryFailed(String), PermissionDenied
}

// FIXME: Implement Display for DatabaseError
impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Format database error messages
        todo!("Implement Display for DatabaseError")
    }
}

// FIXME: Implement Error trait for DatabaseError
impl Error for DatabaseError {}

// TODO: Simulate database query
fn get_user_from_database(id: u32) -> Result<String, DatabaseError> {
    // TODO: Simulate database lookup with different failure scenarios
    todo!("Implement database simulation")
}

// TODO: Handle different database errors with recovery strategies
fn handle_database_error(error: &DatabaseError) {
    // TODO: Implement different recovery strategies based on error type
    // HINT: Retry for connection errors, create user for not found, etc.
    todo!("Implement error recovery strategies")
}

// Exercise 3.4: Application-level error that combines multiple error types
#[derive(Debug)]
enum AppError {
    // TODO: Define variants that wrap other error types
    // HINT: Validation(ValidationError), Network(NetworkError), Database(DatabaseError), 
    //       Io(io::Error), Parse(ParseIntError)
}

// FIXME: Implement Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Format application error messages
        // HINT: Delegate to wrapped error's Display implementation
        todo!("Implement Display for AppError")
    }
}

// FIXME: Implement Error trait for AppError
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: Return the source error for error chaining
        // HINT: Match on self and return Some(&wrapped_error) where applicable
        todo!("Implement Error::source for error chaining")
    }
}

// Exercise 3.5: Implement From traits for automatic error conversion
// FIXME: Implement From<ValidationError> for AppError
impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        // TODO: Convert ValidationError to AppError
        todo!("Implement ValidationError -> AppError conversion")
    }
}

// TODO: Implement more From conversions
// HINT: From<NetworkError>, From<DatabaseError>, From<io::Error>, From<ParseIntError>

// TODO: File processing function that uses AppError
fn process_file(filename: &str) -> Result<String, AppError> {
    // TODO: Simulate file processing with multiple failure points
    // HINT: Different filenames trigger different error types
    // Use ? operator to automatically convert errors
    todo!("Implement file processing with multiple error types")
}

// TODO: Function that prints the full error chain
fn print_error_chain(error: &dyn Error) {
    // TODO: Print the error and all its sources
    // HINT: Use error.source() in a loop
    todo!("Implement error chain printing")
}

// Exercise 3.6: Function that combines multiple error-prone operations
fn parse_and_validate(input: &str) -> Result<u32, AppError> {
    // TODO: Parse string to number and validate it's in valid range
    // HINT: Use ? operator for automatic error conversion
    // Parse the string, then validate the result
    todo!("Implement parse and validate")
}

// Exercise 3.7: Complex operation with error chaining
fn complex_operation() -> Result<String, AppError> {
    // TODO: Perform multiple operations that can each fail
    // HINT: Call validate_user, simulate_http_request, get_user_from_database
    // Use ? operator throughout and see how errors propagate
    todo!("Implement complex operation with error chaining")
}

// Helper function for email validation
fn is_valid_email(email: &str) -> bool {
    // Simple email validation for demonstration
    email.contains('@') && email.contains('.') && !email.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation_error_display() {
        // TODO: Uncomment when ValidationError is implemented
        // let error = ValidationError::EmptyName;
        // assert_eq!(format!("{}", error), "Name cannot be empty");
    }
    
    #[test]
    fn test_user_validation() {
        // TODO: Uncomment when validate_user is implemented
        // assert!(validate_user("", "test@example.com", 25).is_err());
        // assert!(validate_user("John", "", 25).is_err());
        // assert!(validate_user("John", "test@example.com", 25).is_ok());
    }
    
    #[test]
    fn test_error_conversion() {
        // TODO: Uncomment when From traits are implemented
        // let validation_err = ValidationError::EmptyName;
        // let app_err: AppError = validation_err.into();
        // assert!(matches!(app_err, AppError::Validation(_)));
    }
    
    #[test]
    fn test_error_chaining() {
        // TODO: Uncomment when Error::source is implemented
        // let validation_err = ValidationError::EmptyName;
        // let app_err = AppError::Validation(validation_err);
        // assert!(app_err.source().is_some());
    }
    
    #[test]
    fn test_parse_and_validate() {
        // TODO: Uncomment when parse_and_validate is implemented
        // assert!(parse_and_validate("42").is_ok());
        // assert!(parse_and_validate("not_a_number").is_err());
    }
}

// COMPILATION CHALLENGES:
// 1. Define custom error enums with appropriate variants
// 2. Implement Display trait for user-friendly error messages
// 3. Implement Error trait for proper error handling
// 4. Create From traits for automatic error conversion
// 5. Use ? operator for error propagation
// 6. Implement error chaining with Error::source()
// 7. Handle multiple error types in a unified way
// 8. Design recovery strategies for different error scenarios
//
// LEARNING OBJECTIVES FOR C# DEVELOPERS:
// - Custom error types vs C# exceptions
// - Error as values vs thrown exceptions
// - Automatic error conversion with From trait
// - Error chaining for debugging (like InnerException)
// - ? operator vs try-catch blocks
// - Comprehensive error handling strategies
// - Zero-cost error handling vs exception overhead
//
// KEY DIFFERENCES FROM C#:
// C#: throw new ValidationException("Invalid input");
// Rust: return Err(ValidationError::InvalidInput);
//
// C#: try { ... } catch (SpecificException e) { ... }
// Rust: match result { Ok(value) => ..., Err(specific_error) => ... }
//
// C#: exception.InnerException
// Rust: error.source()
//
// C#: Different exception types for different errors
// Rust: Different enum variants for different errors
//
// The goal is to make all errors explicit and recoverable at compile time!
