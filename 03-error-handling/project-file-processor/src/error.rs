// Error handling module - Fix the Broken Code!
//
// This module defines all the error types used throughout the file processor.
// Students need to implement comprehensive error handling with proper Display,
// Error trait implementations, and error conversion.

use std::fmt;
use std::error::Error;
use std::io;
use serde_json;

// Exercise: Define the main error type for the file processor
#[derive(Debug)]
pub enum FileProcessorError {
    // TODO: Add error variants for different failure scenarios
    // HINT: Consider these scenarios:
    // - File not found
    // - I/O errors (reading/writing files)
    // - JSON parsing errors
    // - CSV parsing errors
    // - Validation errors
    // - Configuration errors
    // - Processing failures
    // - Size limit exceeded
    
    // FIXME: Add variants here
    // Example structure:
    // FileNotFound { path: String },
    // IoError(io::Error),
    // JsonError(serde_json::Error),
    // ProcessingFailed { path: String, reason: String },
    // ...
}

// FIXME: Implement Display trait for user-friendly error messages
impl fmt::Display for FileProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Match on self and write appropriate error messages
        // HINT: Use write! macro to format messages
        // EXAMPLE: 
        // match self {
        //     FileProcessorError::FileNotFound { path } => {
        //         write!(f, "File not found: {}", path)
        //     }
        //     FileProcessorError::IoError(e) => {
        //         write!(f, "I/O error: {}", e)
        //     }
        //     ...
        // }
        todo!("Implement Display for FileProcessorError")
    }
}

// FIXME: Implement Error trait for proper error handling
impl Error for FileProcessorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // TODO: Return the source error for error chaining
        // HINT: For wrapped errors (like IoError(io::Error)), return Some(&inner_error)
        // HINT: For custom errors, return None
        todo!("Implement Error::source for error chaining")
    }
}

// Exercise: Implement automatic error conversion with From traits
// FIXME: Convert io::Error to FileProcessorError
impl From<io::Error> for FileProcessorError {
    fn from(err: io::Error) -> Self {
        // TODO: Convert io::Error to appropriate FileProcessorError variant
        // HINT: Use IoError variant or map to more specific errors
        todo!("Convert io::Error to FileProcessorError")
    }
}

// TODO: Implement more From conversions
// HINT: From<serde_json::Error>, From<csv::Error>, From<String>
// EXAMPLE:
// impl From<serde_json::Error> for FileProcessorError {
//     fn from(err: serde_json::Error) -> Self {
//         FileProcessorError::JsonError(err)
//     }
// }

// Exercise: Define a Result type alias for convenience
pub type Result<T> = std::result::Result<T, FileProcessorError>;

// Exercise: Error context helpers for better debugging
impl FileProcessorError {
    // TODO: Helper to create file not found errors
    pub fn file_not_found(path: &str) -> Self {
        // TODO: Create FileNotFound variant with path
        todo!("Create file not found error")
    }
    
    // TODO: Helper to create processing failure errors
    pub fn processing_failed(path: &str, reason: &str) -> Self {
        // TODO: Create ProcessingFailed variant with path and reason
        todo!("Create processing failed error")
    }
    
    // TODO: Helper to create validation errors
    pub fn validation_error(message: &str) -> Self {
        // TODO: Create ValidationError variant with message
        todo!("Create validation error")
    }
    
    // TODO: Helper to create configuration errors
    pub fn config_error(message: &str) -> Self {
        // TODO: Create ConfigError variant with message
        todo!("Create configuration error")
    }
    
    // TODO: Helper to check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        // TODO: Determine which errors might be recoverable with retry
        // HINT: Network timeouts might be recoverable, file not found might not be
        todo!("Determine if error is recoverable")
    }
    
    // TODO: Helper to get error category for metrics/logging
    pub fn category(&self) -> ErrorCategory {
        // TODO: Categorize errors for better handling
        todo!("Categorize error type")
    }
}

// Exercise: Error categories for better error handling
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCategory {
    // TODO: Define error categories
    // HINT: FileSystem, Network, Parsing, Validation, Configuration, Processing
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        // TODO: Test that error Display implementations work correctly
        // HINT: Create different error variants and check their display strings
    }
    
    #[test]
    fn test_error_conversion() {
        // TODO: Test automatic error conversion with From traits
        // HINT: Create underlying errors and convert them to FileProcessorError
    }
    
    #[test]
    fn test_error_chain() {
        // TODO: Test error chaining with source() method
        // HINT: Create errors that wrap other errors and verify the chain
    }
    
    #[test]
    fn test_error_helpers() {
        // TODO: Test error helper methods
        // HINT: Test file_not_found(), processing_failed(), etc.
    }
    
    #[test]
    fn test_error_categories() {
        // TODO: Test error categorization
        // HINT: Verify that different errors return correct categories
    }
    
    #[test]
    fn test_recoverable_errors() {
        // TODO: Test is_recoverable() logic
        // HINT: Verify which errors are considered recoverable
    }
}

// COMPILATION CHALLENGES:
// 1. Define comprehensive error variants covering all failure scenarios
// 2. Implement Display trait with clear, user-friendly messages  
// 3. Implement Error trait with proper source chaining
// 4. Create From implementations for automatic error conversion
// 5. Add helper methods for common error creation patterns
// 6. Categorize errors for better handling and metrics
// 7. Design error types that aid in debugging and recovery
//
// ERROR DESIGN PRINCIPLES FOR STUDENTS:
// - Make errors specific enough to be actionable
// - Include context (file paths, operation details) in errors
// - Chain errors to preserve the full error context
// - Design errors that help with debugging and monitoring
// - Consider which errors might be recoverable vs permanent
// - Use consistent error messages and formatting
//
// C# COMPARISON:
// C#: throw new FileNotFoundException($"File not found: {path}");
// Rust: return Err(FileProcessorError::file_not_found(path));
//
// C#: try { ... } catch (IOException e) { ... }
// Rust: match result { Ok(value) => ..., Err(FileProcessorError::IoError(e)) => ... }
//
// This approach makes all errors explicit and forces handling at compile time!
