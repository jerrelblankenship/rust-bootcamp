// Exercise 2: Error Handling - Fix the Terrible Error Messages!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (5 error improvements)
//
// Your task: Transform cryptic error messages into helpful, user-friendly guidance
// This demonstrates proper CLI error handling and user experience design
//
// INSTRUCTIONS:
// 1. Run the program with bad input to see terrible error messages
// 2. Fix ONE error type at a time
// 3. Test: `cargo run --bin ex02-error-handling -- nonexistent.txt`
// 4. Aim for error messages that help users fix their mistakes
//
// LEARNING STRATEGY:
// - Start with the worst error messages (panics, unwraps)
// - Add context and suggestions to each error
// - Think like a user: what would help them succeed?
//
// ERROR IMPROVEMENTS NEEDED:
// [] Replace panics with proper error handling
// [] Add file existence checking with helpful messages
// [] Provide suggestions for common mistakes
// [] Add colored error output for better visibility
// [] Include examples in error messages

use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // TERRIBLE ERROR: Just panics with no helpful message
    if args.len() != 2 {
        panic!("Wrong number of arguments!");  // FIXME: Awful user experience!
    }
    
    let filename = &args[1];
    
    // TERRIBLE ERROR: File operations that crash with cryptic messages
    let content = fs::read_to_string(filename).unwrap();  // FIXME: Crashes with system errors!
    
    // TERRIBLE ERROR: No validation of content
    let lines: Vec<&str> = content.lines().collect();
    let first_line = lines[0];  // FIXME: Panics if file is empty!
    
    // TERRIBLE ERROR: No error handling for parsing
    let number: i32 = first_line.parse().unwrap();  // FIXME: Crashes on non-numbers!
    
    println!("Successfully parsed number: {}", number);
}

// TODO: Create proper error types
// HINT: In C#, you'd create custom exception classes

#[derive(Debug)]
enum CliError {
    // TODO: Define error variants for different failure modes
    // HINT: Similar to different exception types in C#
}

// TODO: Implement proper error handling functions
// HINT: Return Result<T, E> instead of panicking

fn validate_args(args: &[String]) -> Result<String, CliError> {
    // TODO: Check argument count and provide helpful message
    // GOOD ERROR: "Expected exactly 1 argument (filename), got 0. Try: program file.txt"
    // BAD ERROR: "Wrong number of arguments!"
    todo!("Replace panic with helpful validation")
}

fn read_file_safely(filename: &str) -> Result<String, CliError> {
    // TODO: Check if file exists before trying to read
    // GOOD ERROR: "File 'missing.txt' not found. Check the path and try again."
    // BAD ERROR: "No such file or directory (os error 2)"
    
    if !Path::new(filename).exists() {
        // FIXME: Return proper error instead of this placeholder
        return Err(CliError::FileNotFound);  // This won't compile yet!
    }
    
    // TODO: Handle read errors gracefully
    // GOOD ERROR: "Cannot read 'protected.txt': Permission denied. Check file permissions."
    // BAD ERROR: "Permission denied (os error 13)"
    
    todo!("Implement safe file reading")
}

fn parse_first_line(content: &str) -> Result<i32, CliError> {
    // TODO: Handle empty files gracefully
    // GOOD ERROR: "File is empty. Please provide a file with at least one line containing a number."
    
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        // FIXME: Return proper error
        todo!("Handle empty file");
    }
    
    let first_line = lines[0].trim();
    
    // TODO: Provide helpful parsing errors
    // GOOD ERROR: "Cannot parse 'hello' as a number. Expected an integer like 42 or -17."
    // BAD ERROR: "invalid digit found in string"
    
    first_line.parse().map_err(|_| {
        // FIXME: Create helpful parse error
        todo!("Convert parse error to helpful message")
    })
}

// TODO: Add colored error output
// HINT: Use colored crate or manually add ANSI codes

fn print_error(error: &CliError) {
    // TODO: Print errors in red with suggestions
    // Example: 
    // ❌ Error: File 'test.txt' not found
    // 💡 Suggestion: Check the filename and path, then try again
    
    eprintln!("❌ Error: {}", error);  // FIXME: This won't work until CliError implements Display
    
    // TODO: Add contextual suggestions based on error type
    match error {
        // Add helpful suggestions for each error type
        _ => eprintln!("💡 Suggestion: Check the documentation for help"),
    }
}

// TODO: Create a proper main function with error handling
fn run() -> Result<(), CliError> {
    // TODO: Replace the panicky main() with this proper version
    // 1. Validate arguments
    // 2. Read file safely  
    // 3. Parse content safely
    // 4. Return success or error
    
    todo!("Implement safe main logic")
}

// TODO: Update main to use proper error handling
// fn main() {
//     if let Err(error) = run() {
//         print_error(&error);
//         process::exit(1);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_argument_validation() {
        // TODO: Test that argument validation provides helpful messages
        todo!("Test error messages are helpful")
    }
    
    #[test] 
    fn test_file_error_handling() {
        // TODO: Test file errors produce good messages
        todo!("Test file error messages")
    }
    
    #[test]
    fn test_parse_error_handling() {
        // TODO: Test parsing errors are user-friendly
        todo!("Test parse error messages")
    }
}

// ERROR MESSAGE IMPROVEMENT EXAMPLES:
//
// BEFORE (Terrible):
// thread 'main' panicked at 'Wrong number of arguments!', src/main.rs:15:9
//
// AFTER (Helpful):
// ❌ Error: Expected exactly 1 filename, got 0 arguments
// 💡 Try: program myfile.txt
// 📚 Use --help for more information
//
// BEFORE (Terrible):  
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:21:48
//
// AFTER (Helpful):
// ❌ Error: File 'missing.txt' does not exist
// 💡 Check the filename and path:
//   • Make sure the file exists
//   • Check for typos in the filename  
//   • Use absolute path if in different directory
//
// C# COMPARISON:
// C#: try { ... } catch (FileNotFoundException ex) { Console.Error.WriteLine($"File not found: {ex.FileName}"); }
// Rust: fs::read_to_string(path).map_err(|e| format!("Cannot read {}: {}", path, e))
//
// C#: throw new ArgumentException("Invalid argument count", nameof(args));
// Rust: return Err(CliError::InvalidArgs("Expected 1 argument, got 0".to_string()));
//
// 📊 PROGRESS TRACKER:
// Error Type 1 (Arguments): [ ] Improved
// Error Type 2 (File Not Found): [ ] Improved
// Error Type 3 (Permission Denied): [ ] Improved  
// Error Type 4 (Parse Errors): [ ] Improved
// Error Type 5 (Empty Content): [ ] Improved
//
// 🎯 SUCCESS CRITERIA:
// ✅ No more panics or unwraps
// ✅ Every error provides actionable suggestions
// ✅ Colored output for better visibility  
// ✅ Examples included in error messages
// ✅ Users can fix their mistakes easily
//
// 🚀 NEXT CHALLENGE:
// Move to ex03-config-files.rs to handle configuration!