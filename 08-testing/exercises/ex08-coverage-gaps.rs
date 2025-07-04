// Exercise 8: Coverage Gaps and Edge Cases - Fix the Missing Test Coverage!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (6 checkpoints to fix)
//
// Your task: Fix missing test coverage and untested edge cases
//
// INSTRUCTIONS:
// 1. Fix ONE coverage gap at a time
// 2. Run tests: `cargo test --bin ex08-coverage-gaps`
// 3. Learn to identify and test edge cases systematically
// 4. Use hints in /hints/ directory if you get stuck for 15+ minutes
//
// LEARNING STRATEGY:
// - Start with checkpoint 1 (missing error case testing)
// - Learn to identify untested code paths
// - Fix boundary condition and edge case testing
//
// COMPLETED CONCEPTS:
// [] Systematic edge case identification
// [] Boundary value testing
// [] Error path and exception testing
// [] Input validation testing
// [] State transition testing
// [] Performance and resource limit testing

use std::collections::HashMap;

// A file processing system with many edge cases
pub struct FileProcessor {
    max_file_size: usize,
    allowed_extensions: Vec<String>,
    processed_files: HashMap<String, ProcessingResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessingResult {
    pub success: bool,
    pub lines_processed: usize,
    pub errors: Vec<String>,
    pub processing_time_ms: u64,
}

impl FileProcessor {
    pub fn new(max_file_size: usize, allowed_extensions: Vec<String>) -> Self {
        Self {
            max_file_size,
            allowed_extensions,
            processed_files: HashMap::new(),
        }
    }

    pub fn process_file(&mut self, filename: &str, content: &str) -> Result<ProcessingResult, String> {
        // Input validation
        if filename.is_empty() {
            return Err("Filename cannot be empty".to_string());
        }

        if content.len() > self.max_file_size {
            return Err(format!("File size {} exceeds maximum {}", content.len(), self.max_file_size));
        }

        // Extension validation
        let extension = self.get_file_extension(filename);
        if let Some(ext) = extension {
            if !self.allowed_extensions.contains(&ext) {
                return Err(format!("File extension '{}' not allowed", ext));
            }
        } else {
            return Err("File must have an extension".to_string());
        }

        // Check if already processed
        if self.processed_files.contains_key(filename) {
            return Err("File already processed".to_string());
        }

        // Process the content
        let result = self.process_content(content);
        self.processed_files.insert(filename.to_string(), result.clone());
        
        Ok(result)
    }

    fn get_file_extension(&self, filename: &str) -> Option<String> {
        if let Some(dot_index) = filename.rfind('.') {
            if dot_index < filename.len() - 1 {
                Some(filename[dot_index + 1..].to_lowercase())
            } else {
                None // Filename ends with dot
            }
        } else {
            None // No dot in filename
        }
    }

    fn process_content(&self, content: &str) -> ProcessingResult {
        let lines: Vec<&str> = content.lines().collect();
        let mut errors = Vec::new();
        let mut processed_lines = 0;

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue; // Skip empty lines
            }

            if line.len() > 200 {
                errors.push(format!("Line {} too long: {} characters", line_num + 1, line.len()));
                continue;
            }

            if line.contains("ERROR") {
                errors.push(format!("Error marker found on line {}", line_num + 1));
                continue;
            }

            processed_lines += 1;
        }

        ProcessingResult {
            success: errors.is_empty(),
            lines_processed: processed_lines,
            errors,
            processing_time_ms: content.len() as u64 / 10, // Simulate processing time
        }
    }

    pub fn get_processing_stats(&self) -> (usize, usize, usize) {
        let total = self.processed_files.len();
        let successful = self.processed_files.values().filter(|r| r.success).count();
        let failed = total - successful;
        (total, successful, failed)
    }

    pub fn clear_processed_files(&mut self) {
        self.processed_files.clear();
    }
}

// BROKEN TEST COVERAGE - MISSING MANY EDGE CASES!

#[cfg(test)]
mod tests {
    use super::*;

    // Exercise 8.1: Fix missing error case testing
    #[test]
    fn test_process_file_happy_path_only() {
        let mut processor = FileProcessor::new(1000, vec!["txt".to_string(), "md".to_string()]);
        
        // FIXME: Only testing the happy path!
        let content = "Line 1\nLine 2\nLine 3";
        let result = processor.process_file("test.txt", content);
        
        assert!(result.is_ok());
        let processing_result = result.unwrap();
        assert!(processing_result.success);
        assert_eq!(processing_result.lines_processed, 3);
        
        // FIXME: What about error cases?
        // - Empty filename
        // - Content too large
        // - Invalid file extension
        // - File already processed
        // - No file extension
        // - File ending with dot
    }
    
    // ✅ CHECKPOINT 1: Add comprehensive error case testing

    // Exercise 8.2: Fix missing boundary value testing
    #[test]
    fn test_file_size_boundary() {
        let mut processor = FileProcessor::new(10, vec!["txt".to_string()]);
        
        // FIXME: Not testing boundary values!
        let content = "Hello";  // 5 bytes - well within limit
        let result = processor.process_file("test.txt", content);
        assert!(result.is_ok());
        
        // FIXME: Missing boundary tests:
        // - Exactly at limit (10 bytes)
        // - One byte over limit (11 bytes)
        // - One byte under limit (9 bytes)
        // - Zero bytes (empty content)
        // - Exactly 1 byte
        // - Maximum possible size
    }
    
    // ✅ CHECKPOINT 2: Add systematic boundary value testing

    // Exercise 8.3: Fix missing input validation edge cases
    #[test]
    fn test_filename_validation_incomplete() {
        let mut processor = FileProcessor::new(1000, vec!["txt".to_string()]);
        
        // FIXME: Only testing one invalid case!
        let result = processor.process_file("", "content");
        assert!(result.is_err());
        
        // FIXME: Missing filename edge cases:
        // - Filename with only extension (.txt)
        // - Filename ending with dot (test.)
        // - Filename with multiple dots (test.backup.txt)
        // - Filename with no extension (test)
        // - Filename with uppercase extension (test.TXT)
        // - Very long filename
        // - Filename with special characters
        // - Unicode characters in filename
    }
    
    // ✅ CHECKPOINT 3: Test all filename and extension edge cases

    // Exercise 8.4: Fix missing content processing edge cases
    #[test]
    fn test_content_processing_basic() {
        let mut processor = FileProcessor::new(1000, vec!["txt".to_string()]);
        
        // FIXME: Only testing basic content!
        let content = "Normal line 1\nNormal line 2";
        let result = processor.process_file("test.txt", content);
        assert!(result.is_ok());
        
        // FIXME: Missing content edge cases:
        // - Empty content
        // - Only whitespace
        // - Only newlines
        // - Very long lines (>200 chars)
        // - Lines with "ERROR" marker
        // - Mixed content (some good, some bad lines)
        // - Content with various line endings (\n, \r\n, \r)
        // - Unicode content
        // - Binary content (non-UTF8)
    }
    
    // ✅ CHECKPOINT 4: Test all content processing edge cases

    // Exercise 8.5: Fix missing state transition testing
    #[test]
    fn test_processor_state_simple() {
        let mut processor = FileProcessor::new(1000, vec!["txt".to_string()]);
        
        // FIXME: Not testing state transitions thoroughly!
        processor.process_file("file1.txt", "content1").unwrap();
        let (total, successful, failed) = processor.get_processing_stats();
        assert_eq!(total, 1);
        assert_eq!(successful, 1);
        
        // FIXME: Missing state transition tests:
        // - Processing multiple files
        // - Mix of successful and failed processing
        // - Processing same file twice (should fail)
        // - Clear processed files and reprocess
        // - State after various error conditions
        // - Memory usage with many processed files
    }
    
    // ✅ CHECKPOINT 5: Test state transitions and edge conditions

    // Exercise 8.6: Fix missing error combination and stress testing
    #[test]
    fn test_single_error_condition() {
        let mut processor = FileProcessor::new(1000, vec!["txt".to_string()]);
        
        // FIXME: Only testing one error at a time!
        let result = processor.process_file("test.pdf", "content");  // Wrong extension
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not allowed"));
        
        // FIXME: Missing combination and stress tests:
        // - Multiple error conditions at once
        // - Large number of files processed
        // - Very large allowed extension lists
        // - Stress test with maximum file sizes
        // - Memory leaks with repeated processing
        // - Performance with many processed files
        // - Concurrent access patterns (if applicable)
        // - Resource exhaustion scenarios
    }
    
    // ✅ CHECKPOINT 6: Add comprehensive error combinations and stress tests

    // Helper function to create test content of specific size
    fn create_content_of_size(size: usize) -> String {
        "x".repeat(size)
    }

    // Helper function to create long line content
    fn create_long_line_content(line_length: usize) -> String {
        "x".repeat(line_length)
    }

    // FIXME: These helpers are defined but not used in comprehensive tests!
    // They should be used to test the missing edge cases above.
}

// COMPILATION CHALLENGES:
// 1. Add comprehensive error case testing for all error paths
// 2. Implement systematic boundary value testing
// 3. Test all filename and extension validation edge cases
// 4. Add content processing edge case testing
// 5. Test state transitions and processor lifecycle
// 6. Add stress testing and error combination scenarios
//
// LEARNING OBJECTIVES:
// - Systematic identification of untested code paths
// - Boundary value analysis and edge case testing
// - Input validation testing strategies
// - State transition and lifecycle testing
// - Error combination and stress testing approaches
// - Test coverage analysis and gap identification
//
// C# COMPARISON:
// C#: Code coverage tools like dotCover, OpenCover
// Rust: tarpaulin, kcov for coverage analysis
//
// C#: [TestCase] attributes for parameterized tests
// Rust: Parameterized tests with loops or macros
//
// C#: Assert.Throws<Exception>() for error testing
// Rust: assert!(result.is_err()) and error message checking
//
// C#: Theory and InlineData for boundary testing
// Rust: Property-based testing or manual boundary tests
//
// 📊 PROGRESS TRACKER - Update as you complete each checkpoint:
// Checkpoint 1 (Error case testing): [ ] Complete
// Checkpoint 2 (Boundary values): [ ] Complete
// Checkpoint 3 (Input validation): [ ] Complete
// Checkpoint 4 (Content processing): [ ] Complete
// Checkpoint 5 (State transitions): [ ] Complete
// Checkpoint 6 (Stress testing): [ ] Complete
//
// 🎯 WHEN YOU FINISH:
// All checkpoints complete? Congratulations! You've mastered:
// ✅ Systematic edge case identification and testing
// ✅ Comprehensive boundary value analysis
// ✅ Input validation and error path testing
// ✅ State transition and lifecycle testing
// ✅ Stress testing and error combination scenarios
// ✅ Test coverage analysis and gap remediation
//
// 🚀 Ready for the final challenge?
// Move on to project-testing-suite to build a comprehensive testing framework!
// Or check your work with: `cargo test --bin ex08-coverage-gaps`