# Exercise 2 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working Result<T, E> implementations. Here's the full solution with all exercises implemented.

## 📝 Complete ex02-result-chain.rs Implementation

```rust
// Exercise 2: Result Chaining and Error Propagation - Complete Solutions
//
// This file demonstrates comprehensive Result handling patterns

use std::fs;
use std::num::ParseIntError;

fn main() {
    println!("=== Exercise 2: Result Chaining (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_2_1();
    exercise_2_2();
    exercise_2_3();
    exercise_2_4();
    exercise_2_5();
    exercise_2_6();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 2.1: Basic Result handling with ? operator - SOLVED
fn exercise_2_1() {
    println!("Exercise 2.1: Basic Result handling with ?");
    
    // SOLUTION: Parse string and double it using ? operator
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        let number = s.parse::<i32>()?;  // ? operator extracts value or returns error
        Ok(number * 2)                   // Wrap success in Ok()
    }
    
    // Test cases
    let test_inputs = ["42", "100", "abc", "-15", ""];
    
    for input in test_inputs {
        match parse_and_double(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> Error: {}", input, e),
        }
    }
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Chaining multiple Results - SOLVED
fn exercise_2_2() {
    println!("Exercise 2.2: Chaining multiple Results");
    
    // SOLUTION: Parse all strings and sum them, fail on first error
    fn calculate_sum_from_strings(nums: &[&str]) -> Result<i32, ParseIntError> {
        let mut sum = 0;
        
        for num_str in nums {
            let number = num_str.parse::<i32>()?;  // Early return on any parse error
            sum += number;
        }
        
        Ok(sum)
    }
    
    // Test cases
    let good_input = vec!["10", "20", "30"];
    let bad_input = vec!["10", "abc", "30"];
    let empty_input = vec![];
    
    match calculate_sum_from_strings(&good_input) {
        Ok(sum) => println!("Sum of {:?}: {}", good_input, sum),
        Err(e) => println!("Error with {:?}: {}", good_input, e),
    }
    
    match calculate_sum_from_strings(&bad_input) {
        Ok(sum) => println!("Sum of {:?}: {}", bad_input, sum),
        Err(e) => println!("Error with {:?}: {}", bad_input, e),
    }
    
    match calculate_sum_from_strings(&empty_input) {
        Ok(sum) => println!("Sum of empty input: {}", sum),
        Err(e) => println!("Error with empty input: {}", e),
    }
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Custom error types with conversions - SOLVED
fn exercise_2_3() {
    println!("Exercise 2.3: Custom error types with conversions");
    
    // SOLUTION: Define comprehensive error type
    #[derive(Debug)]
    enum MathError {
        ParseError(ParseIntError),  // Wrap parsing errors
        DivisionByZero,            // Domain-specific error
        NegativeResult,            // Business logic error
    }
    
    // SOLUTION: Automatic conversion from ParseIntError
    impl From<ParseIntError> for MathError {
        fn from(err: ParseIntError) -> Self {
            MathError::ParseError(err)
        }
    }
    
    // SOLUTION: User-friendly error display
    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MathError::ParseError(e) => write!(f, "Failed to parse number: {}", e),
                MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
                MathError::NegativeResult => write!(f, "Operation resulted in negative value"),
            }
        }
    }
    
    // SOLUTION: Proper error trait implementation
    impl std::error::Error for MathError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                MathError::ParseError(e) => Some(e),  // Chain to original error
                _ => None,
            }
        }
    }
    
    // SOLUTION: Safe division with comprehensive error handling
    fn safe_divide_strings(a_str: &str, b_str: &str) -> Result<f64, MathError> {
        let a = a_str.parse::<i32>()?;  // Auto-converts ParseIntError to MathError
        let b = b_str.parse::<i32>()?;  // Auto-converts ParseIntError to MathError
        
        if b == 0 {
            return Err(MathError::DivisionByZero);
        }
        
        let result = a as f64 / b as f64;
        if result < 0.0 {
            return Err(MathError::NegativeResult);
        }
        
        Ok(result)
    }
    
    // Test cases
    let test_cases = [
        ("10", "2"),   // Should work
        ("15", "3"),   // Should work
        ("20", "0"),   // Division by zero
        ("abc", "5"),  // Parse error
        ("8", "def"),  // Parse error
        ("-10", "2"),  // Negative result
    ];
    
    for (a, b) in test_cases {
        match safe_divide_strings(a, b) {
            Ok(result) => println!("{} / {} = {:.2}", a, b, result),
            Err(e) => println!("{} / {} -> Error: {}", a, b, e),
        }
    }
    
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Working with file operations - SOLVED
fn exercise_2_4() {
    println!("Exercise 2.4: Working with file operations");
    
    // First, let's create a test file
    let test_content = "42\n17\n88\n";
    fs::write("test_numbers.txt", test_content).expect("Failed to create test file");
    
    // SOLUTION: Comprehensive file processing error type
    #[derive(Debug)]
    enum FileProcessingError {
        IoError(std::io::Error),        // Wrap I/O errors
        ParseError(ParseIntError),      // Wrap parsing errors
        EmptyFile,                      // Domain-specific error
    }
    
    // SOLUTION: Automatic error conversions
    impl From<std::io::Error> for FileProcessingError {
        fn from(err: std::io::Error) -> Self {
            FileProcessingError::IoError(err)
        }
    }
    
    impl From<ParseIntError> for FileProcessingError {
        fn from(err: ParseIntError) -> Self {
            FileProcessingError::ParseError(err)
        }
    }
    
    // SOLUTION: Display implementation
    impl std::fmt::Display for FileProcessingError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FileProcessingError::IoError(e) => write!(f, "File I/O error: {}", e),
                FileProcessingError::ParseError(e) => write!(f, "Number parsing error: {}", e),
                FileProcessingError::EmptyFile => write!(f, "File is empty or contains no valid data"),
            }
        }
    }
    
    // SOLUTION: Error trait with source chaining
    impl std::error::Error for FileProcessingError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                FileProcessingError::IoError(e) => Some(e),
                FileProcessingError::ParseError(e) => Some(e),
                FileProcessingError::EmptyFile => None,
            }
        }
    }
    
    // SOLUTION: File processing with error propagation
    fn sum_numbers_from_file(path: &str) -> Result<i32, FileProcessingError> {
        let content = fs::read_to_string(path)?;  // I/O error auto-converted
        
        if content.trim().is_empty() {
            return Err(FileProcessingError::EmptyFile);
        }
        
        let mut sum = 0;
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                let number = line.parse::<i32>()?;  // Parse error auto-converted
                sum += number;
            }
        }
        
        Ok(sum)
    }
    
    // Test with our created file
    match sum_numbers_from_file("test_numbers.txt") {
        Ok(sum) => println!("Sum from file: {}", sum),
        Err(e) => println!("Error reading file: {}", e),
    }
    
    // Test with non-existent file
    match sum_numbers_from_file("nonexistent.txt") {
        Ok(sum) => println!("Sum from nonexistent file: {}", sum),
        Err(e) => println!("Error reading nonexistent file: {}", e),
    }
    
    // Clean up
    let _ = fs::remove_file("test_numbers.txt");
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Result combinators - SOLVED
fn exercise_2_5() {
    println!("Exercise 2.5: Result combinators");
    
    // SOLUTION: Chain Result operations using combinators
    fn process_user_input(input: &str) -> Result<String, String> {
        input
            .parse::<i32>()                                    // Result<i32, ParseIntError>
            .map_err(|e| format!("Parse error: {}", e))        // Convert error to String
            .and_then(|num| {                                  // Chain validation
                if num <= 0 {
                    Err("Number must be positive".to_string())
                } else {
                    Ok(num)
                }
            })
            .map(|num| num * num)                              // Square the number
            .map(|squared| format!("Square of input: {}", squared))  // Format result
    }
    
    let test_inputs = ["25", "-10", "abc", "0", "100"];
    
    for input in test_inputs {
        match process_user_input(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' -> Error: {}", input, e),
        }
    }
    
    println!("✅ Exercise 2.5 complete\n");
}

// Exercise 2.6: Collecting Results - SOLVED
fn exercise_2_6() {
    println!("Exercise 2.6: Collecting Results");
    
    // SOLUTION: All-or-nothing parsing (fail on first error)
    fn try_parse_all(strings: &[&str]) -> Result<Vec<i32>, ParseIntError> {
        strings
            .iter()
            .map(|s| s.parse::<i32>())           // Iterator<Result<i32, ParseIntError>>
            .collect()                           // Result<Vec<i32>, ParseIntError>
    }
    
    // SOLUTION: Separate successes and failures
    fn parse_with_partition(strings: &[&str]) -> (Vec<i32>, Vec<ParseIntError>) {
        let results: Vec<Result<i32, ParseIntError>> = strings
            .iter()
            .map(|s| s.parse::<i32>())
            .collect();
        
        let mut successes = Vec::new();
        let mut failures = Vec::new();
        
        for result in results {
            match result {
                Ok(num) => successes.push(num),
                Err(e) => failures.push(e),
            }
        }
        
        (successes, failures)
    }
    
    // SOLUTION: Keep only valid numbers (ignore errors)
    fn sum_valid_numbers(strings: &[&str]) -> i32 {
        strings
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())  // Convert Result to Option, keep Some
            .sum()
    }
    
    let mixed_input = vec!["10", "abc", "20", "def", "30"];
    
    // Test try_parse_all (should fail on first error)
    match try_parse_all(&mixed_input) {
        Ok(numbers) => println!("All parsed: {:?}", numbers),
        Err(e) => println!("Parse failed: {}", e),
    }
    
    // Test parse_with_partition
    let (successes, failures) = parse_with_partition(&mixed_input);
    println!("Successes: {:?}", successes);
    println!("Failures: {} errors", failures.len());
    
    // Test sum_valid_numbers
    let sum = sum_valid_numbers(&mixed_input);
    println!("Sum of valid numbers: {}", sum);
    
    println!("✅ Exercise 2.6 complete\n");
}

// Demonstration of advanced Result patterns
fn demonstrate_advanced_patterns() {
    println!("=== Advanced Result Patterns ===");
    
    // Pattern 1: Error context building
    #[derive(Debug)]
    enum ContextError {
        ParseFailed { input: String, source: ParseIntError },
        ValidationFailed { value: i32, reason: String },
    }
    
    impl std::fmt::Display for ContextError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ContextError::ParseFailed { input, source } => {
                    write!(f, "Failed to parse '{}': {}", input, source)
                }
                ContextError::ValidationFailed { value, reason } => {
                    write!(f, "Validation failed for {}: {}", value, reason)
                }
            }
        }
    }
    
    impl std::error::Error for ContextError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                ContextError::ParseFailed { source, .. } => Some(source),
                ContextError::ValidationFailed { .. } => None,
            }
        }
    }
    
    fn parse_with_context(input: &str) -> Result<i32, ContextError> {
        let number = input.parse::<i32>()
            .map_err(|e| ContextError::ParseFailed {
                input: input.to_string(),
                source: e,
            })?;
        
        if number < 0 {
            return Err(ContextError::ValidationFailed {
                value: number,
                reason: "must be non-negative".to_string(),
            });
        }
        
        Ok(number)
    }
    
    // Pattern 2: Fallback chains
    fn load_config() -> Result<String, &'static str> {
        load_from_env()
            .or_else(|_| load_from_file())
            .or_else(|_| load_from_defaults())
    }
    
    fn load_from_env() -> Result<String, &'static str> {
        Err("No environment config")
    }
    
    fn load_from_file() -> Result<String, &'static str> {
        Err("No file config")
    }
    
    fn load_from_defaults() -> Result<String, &'static str> {
        Ok("Default configuration".to_string())
    }
    
    // Pattern 3: Result transformation chains
    fn process_number_chain(input: &str) -> Result<String, Box<dyn std::error::Error>> {
        input
            .parse::<f64>()?                              // Parse to float
            .sqrt()                                       // Take square root (always succeeds for positive)
            .round() as i32                               // Round to integer
            .checked_mul(2)                               // Safe multiplication
            .ok_or("Multiplication overflow")?            // Convert Option to Result
            .to_string()                                  // Convert to string
            .parse::<i32>()?                              // Parse back (for demonstration)
            .checked_add(1)                               // Safe addition
            .ok_or("Addition overflow")                   // Convert Option to Result
            .map(|result| format!("Final result: {}", result))  // Format final result
    }
    
    // Test the patterns
    println!("Context error demo:");
    match parse_with_context("abc") {
        Ok(val) => println!("Parsed: {}", val),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("Caused by: {}", source);
            }
        }
    }
    
    println!("\nFallback chain demo:");
    match load_config() {
        Ok(config) => println!("Loaded config: {}", config),
        Err(e) => println!("Failed to load config: {}", e),
    }
    
    println!("\nTransformation chain demo:");
    match process_number_chain("16.0") {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Processing failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21").unwrap(), 42);
        assert!(parse_and_double("abc").is_err());
    }
    
    #[test]
    fn test_calculate_sum() {
        assert_eq!(calculate_sum_from_strings(&["10", "20", "30"]).unwrap(), 60);
        assert!(calculate_sum_from_strings(&["10", "abc", "30"]).is_err());
        assert_eq!(calculate_sum_from_strings(&[]).unwrap(), 0);
    }
    
    #[test]
    fn test_custom_error_conversion() {
        #[derive(Debug)]
        enum TestError {
            Parse(ParseIntError),
        }
        
        impl From<ParseIntError> for TestError {
            fn from(err: ParseIntError) -> Self {
                TestError::Parse(err)
            }
        }
        
        fn test_function(s: &str) -> Result<i32, TestError> {
            let num = s.parse::<i32>()?;  // Automatic conversion
            Ok(num)
        }
        
        assert!(test_function("42").is_ok());
        assert!(test_function("abc").is_err());
    }
    
    #[test]
    fn test_try_parse_all() {
        assert_eq!(try_parse_all(&["10", "20", "30"]).unwrap(), vec![10, 20, 30]);
        assert!(try_parse_all(&["10", "abc", "30"]).is_err());
    }
    
    #[test]
    fn test_sum_valid_numbers() {
        assert_eq!(sum_valid_numbers(&["10", "abc", "20", "def", "30"]), 60);
        assert_eq!(sum_valid_numbers(&["abc", "def"]), 0);
    }
    
    #[test]
    fn test_result_combinators() {
        fn simple_process(input: &str) -> Result<i32, String> {
            input
                .parse::<i32>()
                .map_err(|e| format!("Parse error: {}", e))
                .and_then(|n| if n > 0 { Ok(n) } else { Err("Must be positive".to_string()) })
                .map(|n| n * 2)
        }
        
        assert_eq!(simple_process("5").unwrap(), 10);
        assert!(simple_process("0").is_err());
        assert!(simple_process("abc").is_err());
    }
}

// Helper function implementations referenced in exercises
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let number = s.parse::<i32>()?;
    Ok(number * 2)
}

fn calculate_sum_from_strings(nums: &[&str]) -> Result<i32, ParseIntError> {
    let mut sum = 0;
    for num_str in nums {
        let number = num_str.parse::<i32>()?;
        sum += number;
    }
    Ok(sum)
}

fn try_parse_all(strings: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    strings.iter().map(|s| s.parse::<i32>()).collect()
}

fn sum_valid_numbers(strings: &[&str]) -> i32 {
    strings.iter().filter_map(|s| s.parse::<i32>().ok()).sum()
}
```

## 🎓 Key Learning Points from Complete Solutions

### Result<T, E> Fundamentals
1. **Error Propagation**: `?` operator extracts `Ok` values or returns `Err` early
2. **Custom Error Types**: Enums with `From` traits enable automatic conversions
3. **Error Chaining**: `source()` method preserves original error information
4. **Explicit Handling**: Every possible failure mode must be addressed

### Advanced Error Patterns
```rust
// Pattern 1: Error context building
fn operation_with_context(input: &str) -> Result<i32, ContextualError> {
    input.parse::<i32>()
        .map_err(|e| ContextualError::new("parsing user input", e))?
        .validate()
        .map_err(|e| ContextualError::new("validating parsed value", e))
}

// Pattern 2: Fallback chains
fn resilient_operation() -> Result<Data, Error> {
    try_primary_source()
        .or_else(|_| try_secondary_source())
        .or_else(|_| try_fallback_source())
        .or_else(|_| create_default())
}

// Pattern 3: Result transformation pipelines
fn process_pipeline(input: &str) -> Result<ProcessedData, ProcessingError> {
    input
        .parse::<RawData>()?           // Parse input
        .validate()?                   // Validate data
        .transform()?                  // Transform to target format
        .optimize()                    // Optimize (might fail)
}
```

### Collection Processing Strategies
```rust
// All-or-nothing: Fail if any item fails
let results: Result<Vec<T>, E> = items.iter().map(process).collect();

// Best effort: Keep successes, ignore failures  
let successes: Vec<T> = items.iter().filter_map(|item| process(item).ok()).collect();

// Full accounting: Separate successes and failures
let (successes, failures): (Vec<T>, Vec<E>) = items
    .iter()
    .map(process)
    .partition_map(|result| match result {
        Ok(val) => Either::Left(val),
        Err(e) => Either::Right(e),
    });
```

### Performance Considerations
```rust
// Efficient: Avoid unnecessary allocations
result.map_err(|e| MyError::Wrapped(e))  // Wrap existing error

// Less efficient: Create new error messages
result.map_err(|e| MyError::Message(format!("Failed: {}", e)))

// Lazy evaluation: Use closures for expensive operations
result.unwrap_or_else(|| expensive_fallback())
```

## 🚀 Production-Ready Error Handling

### Structured Logging Integration
```rust
use log::{error, warn, info};

fn robust_operation(input: &str) -> Result<Data, OperationError> {
    info!("Starting operation with input: {}", input);
    
    let parsed = parse_input(input)
        .map_err(|e| {
            error!("Parse failed for input '{}': {}", input, e);
            OperationError::ParseFailed(input.to_string(), e)
        })?;
    
    let validated = validate_data(parsed)
        .map_err(|e| {
            warn!("Validation failed: {}", e);
            OperationError::ValidationFailed(e)
        })?;
    
    info!("Operation completed successfully");
    Ok(validated)
}
```

### Error Recovery Strategies
```rust
fn operation_with_retry(input: &str, max_attempts: u32) -> Result<Data, PersistentError> {
    let mut last_error = None;
    
    for attempt in 1..=max_attempts {
        match try_operation(input) {
            Ok(data) => return Ok(data),
            Err(e) if e.is_retryable() => {
                warn!("Attempt {} failed (retryable): {}", attempt, e);
                last_error = Some(e);
                thread::sleep(Duration::from_millis(100 * attempt as u64));
            }
            Err(e) => {
                error!("Operation failed permanently: {}", e);
                return Err(PersistentError::NonRetryable(e));
            }
        }
    }
    
    Err(PersistentError::MaxAttemptsExceeded(
        max_attempts,
        last_error.unwrap(),
    ))
}
```

### Error Metrics and Monitoring
```rust
use std::sync::atomic::{AtomicU64, Ordering};

static PARSE_ERRORS: AtomicU64 = AtomicU64::new(0);
static VALIDATION_ERRORS: AtomicU64 = AtomicU64::new(0);

fn monitored_operation(input: &str) -> Result<Data, MonitoredError> {
    let result = parse_input(input)
        .map_err(|e| {
            PARSE_ERRORS.fetch_add(1, Ordering::Relaxed);
            MonitoredError::Parse(e)
        })?
        .validate()
        .map_err(|e| {
            VALIDATION_ERRORS.fetch_add(1, Ordering::Relaxed);
            MonitoredError::Validation(e)
        })?;
    
    Ok(result)
}

fn get_error_metrics() -> ErrorMetrics {
    ErrorMetrics {
        parse_errors: PARSE_ERRORS.load(Ordering::Relaxed),
        validation_errors: VALIDATION_ERRORS.load(Ordering::Relaxed),
    }
}
```

## 💡 Result vs Exceptions: The Rust Advantage

**C# Exception Problems:**
```csharp
// Hidden failure modes
public ProcessedData ProcessInput(string input) {
    // Can throw ArgumentException, FormatException, ValidationException, etc.
    // Caller has no idea what can go wrong!
    var parsed = Parser.Parse(input);      // Might throw
    var validated = Validator.Check(parsed); // Might throw  
    return Processor.Transform(validated);   // Might throw
}
```

**Rust Result Clarity:**
```rust
// Explicit failure modes
fn process_input(input: &str) -> Result<ProcessedData, ProcessingError> {
    // Caller knows exactly what can fail and how
    let parsed = Parser::parse(input)?;        // ParseError -> ProcessingError
    let validated = Validator::check(parsed)?; // ValidationError -> ProcessingError
    let processed = Processor::transform(validated)?; // TransformError -> ProcessingError
    Ok(processed)
}
```

## 🔗 Integration Patterns

### With async/await
```rust
async fn async_operation(input: &str) -> Result<Data, AsyncError> {
    let data = fetch_remote_data(input).await?;  // Network error propagation
    let processed = process_locally(data)?;      // Processing error propagation
    save_to_database(processed).await?;         // Database error propagation
    Ok(())
}
```

### With iterators and functional style
```rust
fn process_batch(inputs: &[String]) -> Result<Vec<ProcessedData>, BatchError> {
    inputs
        .iter()
        .enumerate()
        .map(|(i, input)| {
            process_single_input(input)
                .map_err(|e| BatchError::ItemFailed { index: i, source: e })
        })
        .collect()  // Fail fast on first error
}
```

## 🏆 Mastery Checkpoint

You've mastered Result<T, E> when you can:
- ✅ Design custom error types with proper trait implementations
- ✅ Use `?` operator fluently for error propagation
- ✅ Chain Result operations with combinators
- ✅ Handle collections of Results with different strategies  
- ✅ Build error context and source chains for debugging
- ✅ Integrate error handling with logging and monitoring
- ✅ Design resilient systems with retry and fallback logic

## ➡️ Next Steps

Ready for even more advanced error handling? Continue with:
- **[Exercise 3](../ex03-error-types.rs)** - Custom error type design patterns
- **[Exercise 4](../ex04-conversions.rs)** - Advanced error conversions and context
- **[File Processor Project](../../project-file-processor/)** - Production error handling

You've now mastered explicit, recoverable error handling! 🦀