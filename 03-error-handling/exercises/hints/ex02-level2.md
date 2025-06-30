# Exercise 2 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Result<T, E> Patterns

You've tried Level 1 hints but need concrete implementation guidance. Here are specific solutions for each Result handling challenge.

## 🔧 Exercise 2.1: Basic Result Handling with ? Operator

**Problem**: Parse string to integer and double it, handling parse errors properly.

**Specific Solution:**
```rust
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let number = s.parse::<i32>()?;  // ? operator auto-propagates ParseIntError
    Ok(number * 2)                   // Wrap success value in Ok()
}

// Alternative without ? operator (for understanding):
fn parse_and_double_explicit(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(number) => Ok(number * 2),  // Transform success value
        Err(e) => Err(e),              // Propagate error
    }
}
```

**Key Learning**: `?` operator extracts value from `Ok()` or returns early with `Err()`.

## 🔧 Exercise 2.2: Chaining Multiple Results

**Problem**: Parse multiple strings and sum them, failing on first parse error.

**Specific Solution:**
```rust
fn calculate_sum_from_strings(nums: &[&str]) -> Result<i32, ParseIntError> {
    let mut sum = 0;
    
    for num_str in nums {
        let number = num_str.parse::<i32>()?;  // Early return on first error
        sum += number;
    }
    
    Ok(sum)
}

// Alternative using iterator approach:
fn calculate_sum_from_strings_iter(nums: &[&str]) -> Result<i32, ParseIntError> {
    nums.iter()
        .map(|s| s.parse::<i32>())        // Vec<Result<i32, ParseIntError>>
        .collect::<Result<Vec<i32>, _>>() // Result<Vec<i32>, ParseIntError>
        .map(|numbers| numbers.iter().sum()) // Sum if all parsing succeeded
}
```

**Key Learning**: 
- Loop with `?` gives "fail fast" behavior
- `collect()` on `Result` transforms `Vec<Result<T,E>>` to `Result<Vec<T>,E>`

## 🔧 Exercise 2.3: Custom Error Types with Conversions

**Problem**: Create unified error type for math operations with multiple failure modes.

**Specific Solution:**
```rust
#[derive(Debug)]
enum MathError {
    ParseError(ParseIntError),  // Wrap parsing errors
    DivisionByZero,            // Domain-specific error
    NegativeResult,            // Business logic error
}

// Automatic conversion enables ? operator usage
impl From<ParseIntError> for MathError {
    fn from(err: ParseIntError) -> Self {
        MathError::ParseError(err)
    }
}

// User-friendly error messages
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::ParseError(e) => write!(f, "Failed to parse number: {}", e),
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeResult => write!(f, "Result cannot be negative"),
        }
    }
}

// Required for proper error trait implementation
impl std::error::Error for MathError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MathError::ParseError(e) => Some(e),  // Chain to original error
            _ => None,
        }
    }
}

// Now we can use ? operator with automatic conversion
fn safe_divide_strings(a_str: &str, b_str: &str) -> Result<f64, MathError> {
    let a = a_str.parse::<i32>()?;  // ParseIntError auto-converted to MathError
    let b = b_str.parse::<i32>()?;  // ParseIntError auto-converted to MathError
    
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    
    let result = a as f64 / b as f64;
    if result < 0.0 {
        return Err(MathError::NegativeResult);
    }
    
    Ok(result)
}
```

**Key Learning**: `From` trait enables automatic error conversion with `?` operator.

## 🔧 Exercise 2.4: File Operations with Multiple Error Types

**Problem**: Read and process files with comprehensive error handling.

**Specific Solution:**
```rust
#[derive(Debug)]
enum FileProcessingError {
    IoError(std::io::Error),        // File I/O errors
    ParseError(ParseIntError),      // Number parsing errors
    EmptyFile,                      // Domain-specific error
}

// Automatic conversions for seamless ? operator usage
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

impl std::fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileProcessingError::IoError(e) => write!(f, "File I/O error: {}", e),
            FileProcessingError::ParseError(e) => write!(f, "Parse error: {}", e),
            FileProcessingError::EmptyFile => write!(f, "File is empty"),
        }
    }
}

impl std::error::Error for FileProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FileProcessingError::IoError(e) => Some(e),
            FileProcessingError::ParseError(e) => Some(e),
            FileProcessingError::EmptyFile => None,
        }
    }
}

fn sum_numbers_from_file(path: &str) -> Result<i32, FileProcessingError> {
    let content = fs::read_to_string(path)?;  // I/O error auto-converted
    
    if content.trim().is_empty() {
        return Err(FileProcessingError::EmptyFile);
    }
    
    let mut sum = 0;
    for line in content.lines() {
        let number = line.trim().parse::<i32>()?;  // Parse error auto-converted
        sum += number;
    }
    
    Ok(sum)
}
```

**Key Learning**: Multiple `From` implementations enable seamless error conversion chains.

## 🔧 Exercise 2.5: Result Combinators

**Problem**: Chain Result operations using combinators instead of explicit matching.

**Specific Solution:**
```rust
fn process_user_input(input: &str) -> Result<String, String> {
    input
        .parse::<i32>()                           // Result<i32, ParseIntError>
        .map_err(|e| format!("Parse error: {}", e))  // Transform error to String
        .and_then(|num| {                         // Chain another operation
            if num <= 0 {
                Err("Number must be positive".to_string())
            } else {
                Ok(num)
            }
        })
        .map(|num| num * num)                     // Transform success value (square)
        .map(|squared| format!("Square of input: {}", squared))  // Final transformation
}

// Alternative step-by-step approach:
fn process_user_input_verbose(input: &str) -> Result<String, String> {
    // Step 1: Parse input
    let number = input
        .parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))?;
    
    // Step 2: Validate positive
    if number <= 0 {
        return Err("Number must be positive".to_string());
    }
    
    // Step 3: Calculate square
    let squared = number * number;
    
    // Step 4: Format result
    Ok(format!("Square of input: {}", squared))
}
```

**Key Learning**: 
- `.map()` transforms success values
- `.map_err()` transforms error values  
- `.and_then()` chains operations that return Result

## 🔧 Exercise 2.6: Collecting Results

**Problem**: Handle collections of Results in different ways.

**Specific Solutions:**
```rust
// Fail on first error (all-or-nothing)
fn try_parse_all(strings: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    strings
        .iter()
        .map(|s| s.parse::<i32>())           // Iterator<Item = Result<i32, ParseIntError>>
        .collect()                           // Result<Vec<i32>, ParseIntError>
}

// Separate successes and failures
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

// Keep only successes, ignore failures
fn sum_valid_numbers(strings: &[&str]) -> i32 {
    strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Convert Result to Option, keep Some
        .sum()
}

// Alternative using partition approach for sum_valid_numbers:
fn sum_valid_numbers_alt(strings: &[&str]) -> i32 {
    strings
        .iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)              // Keep only Ok values
        .sum()
}
```

**Key Learning**: 
- `collect()` transforms `Iterator<Result<T,E>>` to `Result<Vec<T>,E>`
- `filter_map()` with `.ok()` extracts only successful values
- Different strategies for different error handling needs

## 💡 Result Method Cheat Sheet

### Basic Operations
- `match result { Ok(val) => ..., Err(e) => ... }` - Pattern match
- `result?` - Extract value or return error early
- `result.unwrap()` - Extract value, panic on error ⚠️
- `result.unwrap_or(default)` - Extract value or use default

### Transforming Values
- `.map(|val| transform(val))` - Transform success value
- `.map_err(|err| transform(err))` - Transform error value  
- `.and_then(|val| another_result(val))` - Chain Result operations
- `.or_else(|err| fallback_result(err))` - Provide fallback Result

### Converting Types
- `.ok()` - Convert Result to Option (discard error)
- `option.ok_or(error)` - Convert Option to Result
- `result.err()` - Extract error as Option<E>

### Working with Collections
- `collect::<Result<Vec<T>, E>>()` - All-or-nothing collection
- `filter_map(Result::ok)` - Keep only successes
- `partition_map()` - Separate successes and failures (requires itertools)

### Error Context
- `result.with_context(|| "Operation failed")` - Add context (requires anyhow)
- `result.context("Operation failed")` - Add static context (requires anyhow)

## 🔄 Common Error Handling Patterns

### Pattern 1: Early Return Chain
```rust
fn complex_operation(input: &str) -> Result<ProcessedData, MyError> {
    let parsed = parse_input(input)?;      // Return early if parsing fails
    let validated = validate(parsed)?;     // Return early if validation fails  
    let processed = process(validated)?;   // Return early if processing fails
    Ok(processed)
}
```

### Pattern 2: Error Context Building
```rust
fn load_config(path: &str) -> Result<Config, ConfigError> {
    fs::read_to_string(path)
        .map_err(|e| ConfigError::ReadFailed(path.to_string(), e))?
        .parse()
        .map_err(|e| ConfigError::ParseFailed(path.to_string(), e))
}
```

### Pattern 3: Fallback Chain
```rust
fn load_data() -> Result<Data, LoadError> {
    load_from_cache()
        .or_else(|_| load_from_database())
        .or_else(|_| load_from_file())
        .or_else(|_| create_default_data())
}
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex02-level3.md) for full solution code.

## 🎓 Understanding Check

You should now understand:
1. **? operator mechanics**: How it extracts values and propagates errors
2. **Custom error design**: Enums with automatic conversions
3. **Result combinators**: map(), map_err(), and_then() for chaining
4. **Collection handling**: Different strategies for Vec<Result<T,E>>
5. **Error context**: Building helpful error messages with source chains

Ready to build robust, error-safe Rust applications! 🦀