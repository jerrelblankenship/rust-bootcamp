// File Processor Library - Fix the Broken Code!
//
// This is the main library file that students need to implement.
// All the core functionality goes here with proper module organization.

// TODO: Declare all the modules that need to be implemented
// FIXME: These modules don't exist yet - you need to create them!
pub mod error;      // COMPILE ERROR: Module not found!
pub mod config;     // COMPILE ERROR: Module not found!
pub mod processor;  // COMPILE ERROR: Module not found!
pub mod reporting;  // COMPILE ERROR: Module not found!
pub mod formats;    // COMPILE ERROR: Module not found!

// Re-export important types for easier use
pub use error::{FileProcessorError, Result};
pub use config::Config;
pub use processor::{FileProcessorEngine, ProcessingOptions, BatchResult};
pub use reporting::{ReportGenerator, ProcessingReport};

// TODO: Define the core types that the CLI expects

// Exercise: Define OutputFormat enum
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    // TODO: Add variants for different output formats
    // HINT: Json, Csv, Text, Summary
}

// Exercise: Define ProcessingOptions struct
#[derive(Debug, Clone)]
pub struct ProcessingOptions {
    // TODO: Add fields for processing configuration
    // HINT: validate_data: bool, convert_encoding: bool, max_file_size: usize, output_format: OutputFormat
}

// Exercise: Define ProcessingResult for individual files
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    // TODO: Add fields to track processing results
    // HINT: source_path: String, record_count: usize, file_size: usize, processing_time_ms: u64
}

// Exercise: Define ProcessingFailure for failed files
#[derive(Debug, Clone)]
pub struct ProcessingFailure {
    // TODO: Add fields to track processing failures
    // HINT: path: String, error: String
}

// Exercise: Define BatchResult for batch processing
#[derive(Debug)]
pub struct BatchResult {
    // TODO: Add fields to track batch processing results
    // HINT: successes: Vec<ProcessingResult>, failures: Vec<ProcessingFailure>
}

impl BatchResult {
    // TODO: Helper method to calculate total records processed
    pub fn total_records_processed(&self) -> usize {
        // TODO: Sum up record_count from all successful results
        todo!("Calculate total records processed")
    }
    
    // TODO: Helper method to calculate success rate
    pub fn success_rate(&self) -> f64 {
        // TODO: Calculate percentage of successful files
        // HINT: successes.len() / (successes.len() + failures.len())
        todo!("Calculate success rate")
    }
}

// Convenience functions for simple use cases
pub fn process_file(filename: &str) -> Result<ProcessingResult> {
    // TODO: Process a single file with default options
    // HINT: Create FileProcessorEngine and ProcessingOptions::default()
    todo!("Implement single file processing")
}

pub fn process_files(filenames: &[String]) -> BatchResult {
    // TODO: Process multiple files with default options
    // HINT: Use FileProcessorEngine::process_batch()
    todo!("Implement batch file processing")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_batch_result_total_records() {
        // TODO: Test BatchResult::total_records_processed()
        // HINT: Create BatchResult with some ProcessingResults and verify count
    }
    
    #[test]
    fn test_batch_result_success_rate() {
        // TODO: Test BatchResult::success_rate()
        // HINT: Test with different combinations of successes and failures
    }
    
    #[test]
    fn test_process_file() {
        // TODO: Test single file processing
        // HINT: Create a temporary file and process it
    }
}

// IMPLEMENTATION CHALLENGES FOR STUDENTS:
//
// 1. Create error.rs with comprehensive error types
// 2. Create config.rs with configuration management
// 3. Create processor.rs with the main processing engine
// 4. Create reporting.rs with report generation
// 5. Create formats/ module with format-specific processors
// 6. Implement proper error handling throughout
// 7. Add comprehensive test coverage
// 8. Handle all edge cases (empty files, invalid formats, etc.)
//
// This modular structure teaches students:
// - How to organize larger Rust projects
// - Module system and visibility controls
// - Error propagation across modules
// - Testing strategies for different components
// - Separation of concerns in application design
