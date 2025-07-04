# Advanced Error Messages and Test Diagnostics 🔍

*A comprehensive guide to creating informative test failures, custom assertions, and debugging-friendly error output*

## 🚀 Quick Reference

### Essential Error Message Patterns

```rust
// Basic descriptive assertions
assert!(condition, "Expected condition to be true, but got false for input: {:?}", input);
assert_eq!(actual, expected, "Values don't match: expected {} but got {}", expected, actual);

// Custom assertion macros
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!($haystack.contains($needle), 
                "Expected '{}' to contain '{}', but it didn't", $haystack, $needle);
    };
}

// Result testing with context
let result = risky_operation();
assert!(result.is_ok(), "Operation failed: {:?}", result.err());

// Complex data comparison
assert_eq!(actual_user, expected_user, 
           "User mismatch:\nActual: {:#?}\nExpected: {:#?}", actual_user, expected_user);
```

### Quick Debugging Helpers

```rust
// Debug printing in tests
#[test]
fn test_with_debug_output() {
    let data = complex_operation();
    eprintln!("Debug: data = {:#?}", data);  // Shows in test output
    dbg!(&data);                             // Shows variable name + value
    assert!(data.is_valid());
}

// Conditional compilation for test debugging
#[cfg(test)]
macro_rules! test_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*);
    };
}
```

---

## Advanced Assertion Patterns

### 1. Descriptive Assertion Messages

```rust
#[test]
fn test_user_validation_with_context() {
    let user = User {
        name: "".to_string(),
        email: "invalid-email".to_string(),
        age: 150,
    };
    
    // ❌ Bad: Generic error message
    assert!(user.is_valid());
    
    // ✅ Good: Specific error message with context
    let validation_result = user.validate();
    assert!(
        validation_result.is_ok(),
        "User validation failed for user {:?}. Errors: {:?}",
        user,
        validation_result.err()
    );
    
    // ✅ Better: Individual field validation with specific messages
    assert!(
        !user.name.is_empty(),
        "User name should not be empty, but got: '{}'",
        user.name
    );
    
    assert!(
        user.email.contains('@'),
        "Email should contain '@' symbol, but got: '{}'",
        user.email
    );
    
    assert!(
        user.age <= 120,
        "Age should be realistic (≤120), but got: {}",
        user.age
    );
}

#[test]
fn test_collection_operations_with_context() {
    let numbers = vec![1, 2, 3, 4, 5];
    let result = numbers.iter().filter(|&&x| x > 3).collect::<Vec<_>>();
    
    // ✅ Good: Show both expected and actual collections
    let expected = vec![&4, &5];
    assert_eq!(
        result, expected,
        "Filter operation failed:\nInput: {:?}\nExpected: {:?}\nActual: {:?}",
        numbers, expected, result
    );
    
    // ✅ Good: Check collection properties with context
    assert!(
        !result.is_empty(),
        "Expected filtered result to be non-empty for input {:?}, but got empty result",
        numbers
    );
    
    assert_eq!(
        result.len(), 2,
        "Expected filtered result to have 2 elements for input {:?}, but got {} elements: {:?}",
        numbers, result.len(), result
    );
}
```

### 2. Custom Assertion Macros

```rust
// String-specific assertions
macro_rules! assert_str_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected string '{}' to contain '{}', but it didn't",
            $haystack, $needle
        )
    };
    ($haystack:expr, $needle:expr, $($arg:tt)*) => {
        assert!(
            $haystack.contains($needle),
            $($arg)*
        )
    };
}

macro_rules! assert_str_starts_with {
    ($string:expr, $prefix:expr) => {
        assert!(
            $string.starts_with($prefix),
            "Expected string '{}' to start with '{}', but it didn't",
            $string, $prefix
        )
    };
}

macro_rules! assert_str_ends_with {
    ($string:expr, $suffix:expr) => {
        assert!(
            $string.ends_with($suffix),
            "Expected string '{}' to end with '{}', but it didn't",
            $string, $suffix
        )
    };
}

// Numeric assertions
macro_rules! assert_between {
    ($value:expr, $min:expr, $max:expr) => {
        assert!(
            $value >= $min && $value <= $max,
            "Expected {} to be between {} and {}, but got {}",
            stringify!($value), $min, $max, $value
        )
    };
}

macro_rules! assert_close {
    ($a:expr, $b:expr, $epsilon:expr) => {
        let diff = ($a - $b).abs();
        assert!(
            diff <= $epsilon,
            "Expected {} and {} to be within {} of each other, but difference was {}",
            $a, $b, $epsilon, diff
        )
    };
}

// Collection assertions
macro_rules! assert_contains {
    ($collection:expr, $item:expr) => {
        assert!(
            $collection.contains(&$item),
            "Expected collection {:?} to contain {:?}, but it didn't",
            $collection, $item
        )
    };
}

macro_rules! assert_not_contains {
    ($collection:expr, $item:expr) => {
        assert!(
            !$collection.contains(&$item),
            "Expected collection {:?} to NOT contain {:?}, but it did",
            $collection, $item
        )
    };
}

macro_rules! assert_len {
    ($collection:expr, $expected_len:expr) => {
        assert_eq!(
            $collection.len(), $expected_len,
            "Expected collection to have length {}, but got length {}. Collection: {:?}",
            $expected_len, $collection.len(), $collection
        )
    };
}

// Usage examples
#[test]
fn test_custom_assertions() {
    let text = "Hello, World!";
    assert_str_contains!(text, "World");
    assert_str_starts_with!(text, "Hello");
    assert_str_ends_with!(text, "!");
    
    let number = 42.7;
    assert_between!(number, 40.0, 45.0);
    assert_close!(number, 42.8, 0.2);
    
    let items = vec![1, 2, 3, 4, 5];
    assert_contains!(items, 3);
    assert_not_contains!(items, 10);
    assert_len!(items, 5);
}
```

### 3. Result and Option Testing

```rust
// Enhanced Result testing
macro_rules! assert_ok {
    ($result:expr) => {
        assert!(
            $result.is_ok(),
            "Expected Ok, but got Err: {:?}",
            $result.err()
        )
    };
    ($result:expr, $expected:expr) => {
        match $result {
            Ok(value) => assert_eq!(
                value, $expected,
                "Expected Ok({}), but got Ok({})",
                $expected, value
            ),
            Err(e) => panic!("Expected Ok({}), but got Err: {:?}", $expected, e),
        }
    };
}

macro_rules! assert_err {
    ($result:expr) => {
        assert!(
            $result.is_err(),
            "Expected Err, but got Ok: {:?}",
            $result.ok()
        )
    };
    ($result:expr, $expected_err:expr) => {
        match $result {
            Err(err) => assert_eq!(
                err, $expected_err,
                "Expected Err({}), but got Err({})",
                $expected_err, err
            ),
            Ok(value) => panic!("Expected Err({}), but got Ok: {:?}", $expected_err, value),
        }
    };
}

// Enhanced Option testing
macro_rules! assert_some {
    ($option:expr) => {
        assert!(
            $option.is_some(),
            "Expected Some, but got None"
        )
    };
    ($option:expr, $expected:expr) => {
        match $option {
            Some(value) => assert_eq!(
                value, $expected,
                "Expected Some({}), but got Some({})",
                $expected, value
            ),
            None => panic!("Expected Some({}), but got None", $expected),
        }
    };
}

macro_rules! assert_none {
    ($option:expr) => {
        assert!(
            $option.is_none(),
            "Expected None, but got Some: {:?}",
            $option
        )
    };
}

#[test]
fn test_result_option_assertions() {
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("something went wrong");
    
    assert_ok!(success, 42);
    assert_err!(failure, "something went wrong");
    
    let some_value = Some(100);
    let no_value: Option<i32> = None;
    
    assert_some!(some_value, 100);
    assert_none!(no_value);
}
```

### 4. Pattern Matching Assertions

```rust
use std::collections::HashMap;

// Pattern matching assertion macro
macro_rules! assert_matches {
    ($expression:expr, $pattern:pat) => {
        match $expression {
            $pattern => (),
            ref e => panic!(
                "Expected expression to match pattern {}, but got: {:?}",
                stringify!($pattern), e
            ),
        }
    };
    ($expression:expr, $pattern:pat, $($arg:tt)*) => {
        match $expression {
            $pattern => (),
            ref e => panic!($($arg)*),
        }
    };
}

// Enum variant testing
#[derive(Debug, PartialEq)]
enum ApiResponse {
    Success { data: String, code: u16 },
    Error { message: String, code: u16 },
    Timeout,
}

#[test]
fn test_enum_pattern_matching() {
    let response = ApiResponse::Success {
        data: "user data".to_string(),
        code: 200,
    };
    
    // Test specific enum variant
    assert_matches!(
        response,
        ApiResponse::Success { code: 200, .. },
        "Expected successful response with code 200, but got: {:?}",
        response
    );
    
    let error_response = ApiResponse::Error {
        message: "Not found".to_string(),
        code: 404,
    };
    
    assert_matches!(
        error_response,
        ApiResponse::Error { code: 404, .. }
    );
}

// Complex data structure assertions
#[test]
fn test_complex_structure_matching() {
    let mut config = HashMap::new();
    config.insert("database_url".to_string(), "postgres://localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    
    // Check specific keys exist with expected patterns
    if let Some(db_url) = config.get("database_url") {
        assert_str_contains!(db_url, "postgres://");
    } else {
        panic!("Expected database_url to be present in config: {:?}", config);
    }
    
    // Pattern match on complex structures
    let user_data = (
        "Alice".to_string(),
        25,
        Some("alice@example.com".to_string()),
    );
    
    assert_matches!(
        user_data,
        (ref name, age, Some(ref email)) 
        if name == "Alice" && age > 18 && email.contains('@'),
        "User data doesn't match expected pattern: {:?}",
        user_data
    );
}
```

---

## Debugging and Diagnostic Tools

### 1. Test Output and Logging

```rust
// Comprehensive test logging
#[test]
fn test_with_comprehensive_logging() {
    // Initialize logging for tests
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
    
    log::info!("Starting test with comprehensive logging");
    
    let input = "test data";
    log::debug!("Processing input: {:?}", input);
    
    let result = process_data(input);
    
    // Use eprintln! for test-specific output that always shows
    eprintln!("Test result: {:#?}", result);
    
    // Use dbg! macro for variable inspection
    dbg!(&result);
    
    // Conditional debug output
    #[cfg(debug_assertions)]
    {
        eprintln!("Debug mode: Additional validation checks");
        validate_result_integrity(&result);
    }
    
    assert!(result.is_valid(), "Result validation failed");
    
    log::info!("Test completed successfully");
}

// Test-specific debug macros
macro_rules! test_debug {
    ($($arg:tt)*) => {
        #[cfg(test)]
        eprintln!("[TEST DEBUG] {}", format!($($arg)*));
    };
}

macro_rules! test_trace {
    ($var:expr) => {
        #[cfg(test)]
        eprintln!("[TEST TRACE] {} = {:#?}", stringify!($var), $var);
    };
}

#[test]
fn test_with_debug_macros() {
    let data = vec![1, 2, 3, 4, 5];
    test_debug!("Starting test with data length: {}", data.len());
    
    let processed = data.iter().map(|x| x * 2).collect::<Vec<_>>();
    test_trace!(processed);
    
    assert_eq!(processed, vec![2, 4, 6, 8, 10]);
}

fn process_data(_input: &str) -> ProcessResult {
    ProcessResult { valid: true, data: "processed".to_string() }
}

#[derive(Debug)]
struct ProcessResult {
    valid: bool,
    data: String,
}

impl ProcessResult {
    fn is_valid(&self) -> bool {
        self.valid
    }
}

fn validate_result_integrity(_result: &ProcessResult) {
    // Additional validation in debug mode
}
```

### 2. Error Context and Stack Traces

```rust
use std::backtrace::Backtrace;

// Enhanced error types for testing
#[derive(Debug)]
struct TestError {
    message: String,
    context: Vec<String>,
    backtrace: Backtrace,
}

impl TestError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
            backtrace: Backtrace::capture(),
        }
    }
    
    fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Test Error: {}", self.message)?;
        
        if !self.context.is_empty() {
            writeln!(f, "Context:")?;
            for (i, ctx) in self.context.iter().enumerate() {
                writeln!(f, "  {}: {}", i + 1, ctx)?;
            }
        }
        
        writeln!(f, "Backtrace:")?;
        write!(f, "{}", self.backtrace)
    }
}

impl std::error::Error for TestError {}

// Test helper for creating detailed error messages
fn assert_with_context<F>(condition: bool, context: &str, error_fn: F) 
where
    F: FnOnce() -> String,
{
    if !condition {
        let error = TestError::new(error_fn())
            .with_context(context.to_string());
        panic!("{}", error);
    }
}

#[test]
fn test_with_enhanced_error_context() {
    let user_id = 123;
    let user = find_user(user_id);
    
    assert_with_context(
        user.is_some(),
        &format!("Finding user with ID: {}", user_id),
        || format!("User with ID {} was not found", user_id)
    );
    
    let user = user.unwrap();
    
    assert_with_context(
        user.is_active(),
        &format!("Checking if user {} is active", user.id),
        || format!("User {} is not active: {:?}", user.id, user)
    );
}

#[derive(Debug)]
struct User {
    id: u32,
    active: bool,
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

fn find_user(id: u32) -> Option<User> {
    if id == 123 {
        Some(User { id, active: true })
    } else {
        None
    }
}
```

### 3. Data Comparison and Diff Tools

```rust
// Enhanced data comparison for tests
use serde_json::{json, Value};

macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {
        let left_json = serde_json::to_value(&$left).unwrap();
        let right_json = serde_json::to_value(&$right).unwrap();
        
        if left_json != right_json {
            let left_pretty = serde_json::to_string_pretty(&left_json).unwrap();
            let right_pretty = serde_json::to_string_pretty(&right_json).unwrap();
            
            panic!(
                "JSON values are not equal:\n\nLeft:\n{}\n\nRight:\n{}\n",
                left_pretty, right_pretty
            );
        }
    };
}

// String diff utility
fn string_diff(expected: &str, actual: &str) -> String {
    let mut diff = String::new();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    
    let max_lines = expected_lines.len().max(actual_lines.len());
    
    for i in 0..max_lines {
        let expected_line = expected_lines.get(i).unwrap_or(&"<missing>");
        let actual_line = actual_lines.get(i).unwrap_or(&"<missing>");
        
        if expected_line != actual_line {
            diff.push_str(&format!(
                "Line {}: Expected '{}', got '{}'\n",
                i + 1, expected_line, actual_line
            ));
        }
    }
    
    diff
}

macro_rules! assert_str_eq_diff {
    ($expected:expr, $actual:expr) => {
        if $expected != $actual {
            let diff = string_diff($expected, $actual);
            panic!(
                "Strings are not equal:\n\nDifferences:\n{}\n\nExpected:\n{}\n\nActual:\n{}\n",
                diff, $expected, $actual
            );
        }
    };
}

#[test]
fn test_enhanced_data_comparison() {
    #[derive(serde::Serialize)]
    struct TestData {
        name: String,
        value: i32,
        tags: Vec<String>,
    }
    
    let expected = TestData {
        name: "test".to_string(),
        value: 42,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };
    
    let actual = TestData {
        name: "test".to_string(),
        value: 42,
        tags: vec!["tag1".to_string(), "tag2".to_string()],
    };
    
    assert_json_eq!(expected, actual);
    
    let expected_text = "Line 1\nLine 2\nLine 3";
    let actual_text = "Line 1\nLine 2\nLine 3";
    
    assert_str_eq_diff!(expected_text, actual_text);
}
```

### 4. Performance and Resource Monitoring in Tests

```rust
use std::time::{Duration, Instant};

// Performance assertion macros
macro_rules! assert_duration_less_than {
    ($duration:expr, $max:expr) => {
        assert!(
            $duration < $max,
            "Operation took too long: {:?} (max allowed: {:?})",
            $duration, $max
        )
    };
}

macro_rules! assert_performance {
    ($code:block, $max_duration:expr) => {
        let start = Instant::now();
        $code
        let duration = start.elapsed();
        assert_duration_less_than!(duration, $max_duration);
    };
}

// Memory usage tracking
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TestAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TestAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TestAllocator = TestAllocator;

fn get_allocated_bytes() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

macro_rules! assert_memory_usage {
    ($code:block, $max_bytes:expr) => {
        let start_memory = get_allocated_bytes();
        $code
        let end_memory = get_allocated_bytes();
        let used_memory = end_memory - start_memory;
        
        assert!(
            used_memory <= $max_bytes,
            "Memory usage exceeded limit: {} bytes used (max: {} bytes)",
            used_memory, $max_bytes
        );
    };
}

#[test]
fn test_performance_and_memory() {
    // Performance testing
    assert_performance!({
        let result: u64 = (0..1000).sum();
        assert_eq!(result, 499500);
    }, Duration::from_millis(10));
    
    // Memory usage testing
    assert_memory_usage!({
        let data: Vec<u64> = (0..1000).collect();
        assert_eq!(data.len(), 1000);
    }, 8192); // 1000 * 8 bytes for u64
}
```

---

## Test Organization and Reporting

### 1. Structured Test Output

```rust
// Test result reporting
struct TestReport {
    test_name: String,
    status: TestStatus,
    duration: Duration,
    message: Option<String>,
}

#[derive(Debug)]
enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

impl TestReport {
    fn new(test_name: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            status: TestStatus::Passed,
            duration: Duration::default(),
            message: None,
        }
    }
    
    fn with_status(mut self, status: TestStatus) -> Self {
        self.status = status;
        self
    }
    
    fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
    
    fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl std::fmt::Display for TestReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {} ({:?})", 
               self.status, self.test_name, self.duration)?;
        
        if let Some(ref message) = self.message {
            write!(f, " - {}", message)?;
        }
        
        Ok(())
    }
}

// Test wrapper macro for automatic reporting
macro_rules! test_with_report {
    ($test_name:expr, $test_fn:expr) => {
        {
            let start = Instant::now();
            let result = std::panic::catch_unwind(|| $test_fn);
            let duration = start.elapsed();
            
            let report = match result {
                Ok(_) => TestReport::new($test_name)
                    .with_status(TestStatus::Passed)
                    .with_duration(duration),
                Err(e) => {
                    let message = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "Unknown panic".to_string()
                    };
                    
                    TestReport::new($test_name)
                        .with_status(TestStatus::Failed)
                        .with_duration(duration)
                        .with_message(message)
                }
            };
            
            eprintln!("{}", report);
            
            if let Err(e) = result {
                std::panic::resume_unwind(e);
            }
        }
    };
}

#[test]
fn test_with_structured_reporting() {
    test_with_report!("user_creation_test", || {
        let user = User { id: 1, active: true };
        assert!(user.is_active());
    });
    
    test_with_report!("data_processing_test", || {
        let data = vec![1, 2, 3];
        let sum: i32 = data.iter().sum();
        assert_eq!(sum, 6);
    });
}
```

### 2. Custom Test Runners

```rust
// Custom test harness for detailed reporting
use std::collections::HashMap;

struct TestSuite {
    tests: HashMap<String, Box<dyn Fn() + Send + Sync>>,
    setup: Option<Box<dyn Fn() + Send + Sync>>,
    teardown: Option<Box<dyn Fn() + Send + Sync>>,
}

impl TestSuite {
    fn new() -> Self {
        Self {
            tests: HashMap::new(),
            setup: None,
            teardown: None,
        }
    }
    
    fn add_test<F>(&mut self, name: impl Into<String>, test_fn: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.tests.insert(name.into(), Box::new(test_fn));
    }
    
    fn set_setup<F>(&mut self, setup_fn: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.setup = Some(Box::new(setup_fn));
    }
    
    fn set_teardown<F>(&mut self, teardown_fn: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.teardown = Some(Box::new(teardown_fn));
    }
    
    fn run(&self) -> TestSuiteResult {
        let mut results = Vec::new();
        
        for (test_name, test_fn) in &self.tests {
            // Run setup
            if let Some(ref setup) = self.setup {
                setup();
            }
            
            // Run test
            let start = Instant::now();
            let result = std::panic::catch_unwind(|| test_fn());
            let duration = start.elapsed();
            
            let test_result = match result {
                Ok(_) => TestResult::Passed { duration },
                Err(e) => {
                    let message = format!("{:?}", e);
                    TestResult::Failed { duration, message }
                }
            };
            
            results.push((test_name.clone(), test_result));
            
            // Run teardown
            if let Some(ref teardown) = self.teardown {
                teardown();
            }
        }
        
        TestSuiteResult { results }
    }
}

#[derive(Debug)]
enum TestResult {
    Passed { duration: Duration },
    Failed { duration: Duration, message: String },
}

struct TestSuiteResult {
    results: Vec<(String, TestResult)>,
}

impl TestSuiteResult {
    fn summary(&self) -> String {
        let total = self.results.len();
        let passed = self.results.iter()
            .filter(|(_, result)| matches!(result, TestResult::Passed { .. }))
            .count();
        let failed = total - passed;
        
        format!("Tests: {} passed, {} failed, {} total", passed, failed, total)
    }
    
    fn detailed_report(&self) -> String {
        let mut report = String::new();
        
        for (name, result) in &self.results {
            match result {
                TestResult::Passed { duration } => {
                    report.push_str(&format!("✓ {} ({:?})\n", name, duration));
                }
                TestResult::Failed { duration, message } => {
                    report.push_str(&format!("✗ {} ({:?}): {}\n", name, duration, message));
                }
            }
        }
        
        report.push_str(&format!("\n{}\n", self.summary()));
        report
    }
}

#[test]
fn test_custom_test_suite() {
    let mut suite = TestSuite::new();
    
    suite.set_setup(|| {
        eprintln!("Setting up test environment");
    });
    
    suite.set_teardown(|| {
        eprintln!("Cleaning up test environment");
    });
    
    suite.add_test("simple_addition", || {
        assert_eq!(2 + 2, 4);
    });
    
    suite.add_test("string_operations", || {
        let s = "hello".to_uppercase();
        assert_eq!(s, "HELLO");
    });
    
    let results = suite.run();
    eprintln!("{}", results.detailed_report());
}
```

---

## Integration with External Tools

### 1. JUnit XML Output

```rust
// JUnit XML report generation
use std::fs::File;
use std::io::Write;

struct JUnitTestCase {
    name: String,
    duration: Duration,
    failure: Option<String>,
}

struct JUnitTestSuite {
    name: String,
    test_cases: Vec<JUnitTestCase>,
}

impl JUnitTestSuite {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            test_cases: Vec::new(),
        }
    }
    
    fn add_test_case(&mut self, test_case: JUnitTestCase) {
        self.test_cases.push(test_case);
    }
    
    fn to_xml(&self) -> String {
        let total_duration: Duration = self.test_cases.iter()
            .map(|tc| tc.duration)
            .sum();
        
        let failures = self.test_cases.iter()
            .filter(|tc| tc.failure.is_some())
            .count();
        
        let mut xml = format!(
            r#"<testsuite name="{}" tests="{}" failures="{}" time="{:.3}">"#,
            self.name,
            self.test_cases.len(),
            failures,
            total_duration.as_secs_f64()
        );
        
        for test_case in &self.test_cases {
            xml.push_str(&format!(
                r#"<testcase name="{}" time="{:.3}">"#,
                test_case.name,
                test_case.duration.as_secs_f64()
            ));
            
            if let Some(ref failure) = test_case.failure {
                xml.push_str(&format!(
                    r#"<failure message="Test failed">{}</failure>"#,
                    html_escape(failure)
                ));
            }
            
            xml.push_str("</testcase>");
        }
        
        xml.push_str("</testsuite>");
        xml
    }
    
    fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(self.to_xml().as_bytes())?;
        Ok(())
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[test]
fn test_junit_output() {
    let mut test_suite = JUnitTestSuite::new("example_tests");
    
    test_suite.add_test_case(JUnitTestCase {
        name: "test_success".to_string(),
        duration: Duration::from_millis(100),
        failure: None,
    });
    
    test_suite.add_test_case(JUnitTestCase {
        name: "test_failure".to_string(),
        duration: Duration::from_millis(50),
        failure: Some("Assertion failed: expected 5, got 4".to_string()),
    });
    
    // Save to file for CI/CD consumption
    test_suite.save_to_file("test-results.xml").unwrap();
    
    eprintln!("JUnit XML:\n{}", test_suite.to_xml());
}
```

### 2. Test Coverage Integration

```rust
// Coverage-aware test assertions
macro_rules! assert_with_coverage {
    ($condition:expr, $coverage_id:expr) => {
        // Mark this code path as covered
        #[cfg(feature = "coverage")]
        coverage::mark($coverage_id);
        
        if !$condition {
            panic!("Assertion failed at coverage point: {}", $coverage_id);
        }
    };
}

#[cfg(feature = "coverage")]
mod coverage {
    use std::collections::HashSet;
    use std::sync::Mutex;
    
    lazy_static::lazy_static! {
        static ref COVERED_POINTS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    }
    
    pub fn mark(id: &str) {
        let mut covered = COVERED_POINTS.lock().unwrap();
        covered.insert(id.to_string());
    }
    
    pub fn get_covered_points() -> Vec<String> {
        let covered = COVERED_POINTS.lock().unwrap();
        covered.iter().cloned().collect()
    }
    
    pub fn coverage_report() -> String {
        let covered = get_covered_points();
        format!("Covered {} code paths: {:?}", covered.len(), covered)
    }
}

#[test]
#[cfg(feature = "coverage")]
fn test_with_coverage_tracking() {
    let value = 42;
    
    if value > 0 {
        assert_with_coverage!(value == 42, "positive_branch");
    } else {
        assert_with_coverage!(false, "negative_branch");
    }
    
    eprintln!("{}", coverage::coverage_report());
}
```

---

## Best Practices Summary

### 1. Error Message Guidelines

```rust
// ✅ Good error message practices
#[test]
fn good_error_messages_example() {
    let user_input = "invalid@email";
    
    // Include context about what was being tested
    assert!(
        validate_email(user_input),
        "Email validation failed for input '{}'. Expected valid email format.",
        user_input
    );
    
    // Show both expected and actual values
    let calculated = calculate_total(&[1, 2, 3]);
    let expected = 6;
    assert_eq!(
        calculated, expected,
        "Total calculation incorrect: expected {}, got {} for input [1, 2, 3]",
        expected, calculated
    );
    
    // Provide debugging information
    let config = load_config();
    assert!(
        config.is_valid(),
        "Configuration validation failed: {:?}\nProblems: {:?}",
        config, config.validate_problems()
    );
}

fn validate_email(_email: &str) -> bool { true }
fn calculate_total(items: &[i32]) -> i32 { items.iter().sum() }

#[derive(Debug)]
struct Config { valid: bool }
impl Config {
    fn is_valid(&self) -> bool { self.valid }
    fn validate_problems(&self) -> Vec<String> { vec![] }
}
fn load_config() -> Config { Config { valid: true } }
```

### 2. Assertion Organization

```rust
// Organize assertions logically
#[test]
fn well_organized_test() {
    // Arrange
    let user = create_test_user();
    let context = "User creation and validation test";
    
    // Act
    let result = process_user(&user);
    
    // Assert - grouped by concern
    
    // 1. Basic success check
    assert!(
        result.is_ok(),
        "{}: Operation should succeed, but got error: {:?}",
        context, result.err()
    );
    
    let processed_user = result.unwrap();
    
    // 2. Data integrity checks
    assert_eq!(
        processed_user.id, user.id,
        "{}: User ID should be preserved",
        context
    );
    
    assert_eq!(
        processed_user.email, user.email,
        "{}: Email should be preserved",
        context
    );
    
    // 3. Business logic checks
    assert!(
        processed_user.is_verified(),
        "{}: User should be verified after processing",
        context
    );
    
    assert!(
        processed_user.created_at <= std::time::SystemTime::now(),
        "{}: Created timestamp should not be in the future",
        context
    );
}

fn create_test_user() -> TestUser {
    TestUser {
        id: 1,
        email: "test@example.com".to_string(),
    }
}

fn process_user(user: &TestUser) -> Result<ProcessedUser, String> {
    Ok(ProcessedUser {
        id: user.id,
        email: user.email.clone(),
        verified: true,
        created_at: std::time::SystemTime::now(),
    })
}

#[derive(Debug)]
struct TestUser {
    id: u32,
    email: String,
}

#[derive(Debug)]
struct ProcessedUser {
    id: u32,
    email: String,
    verified: bool,
    created_at: std::time::SystemTime,
}

impl ProcessedUser {
    fn is_verified(&self) -> bool {
        self.verified
    }
}
```

---

## Conclusion

Advanced error messages and test diagnostics in Rust provide:

1. **Clear Failure Communication** - Descriptive messages that guide debugging
2. **Rich Context Information** - Relevant data and state when tests fail
3. **Debugging Integration** - Seamless integration with debugging tools
4. **Structured Reporting** - Machine-readable and human-friendly output
5. **Performance Insights** - Memory and timing information for optimization

Key principles:
- **Always include context** in assertion messages
- **Show both expected and actual values** for comparisons
- **Use custom assertion macros** for common patterns
- **Provide debugging information** in test output
- **Structure test output** for both humans and tools

With these advanced error message techniques, debugging test failures becomes much more efficient and productive, leading to faster development cycles and higher code quality.