# Exercise 3 Hints - Level 1 (Gentle Guidance)

## 🔍 Understanding Custom Error Types for C# Developers

You're working on custom error type design, which is like creating your own exception hierarchy in C#, but as values rather than throwable objects. This is about designing error types that are both specific and actionable.

## 💡 Core Concept: Structured Error Information

### What are Custom Error Types?
Custom error types let you define exactly what can go wrong in your domain, with structured data that helps users understand and potentially recover from errors. Unlike generic errors, they carry specific context.

**C# Comparison:**
```csharp
// C# - Custom exception hierarchy
public class ValidationException : Exception {
    public string FieldName { get; }
    public string InvalidValue { get; }
    
    public ValidationException(string fieldName, string invalidValue, string message) 
        : base(message) {
        FieldName = fieldName;
        InvalidValue = invalidValue;
    }
}

try {
    ValidateUser(userData);
} catch (ValidationException ex) {
    Console.WriteLine($"Field '{ex.FieldName}' has invalid value '{ex.InvalidValue}': {ex.Message}");
}
```

**Rust Equivalent:**
```rust
// Rust - Custom error enum with structured data
#[derive(Debug)]
enum ValidationError {
    EmptyName,
    InvalidEmail { email: String },
    InvalidAge { age: u32, min: u32, max: u32 },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::EmptyName => write!(f, "Name cannot be empty"),
            ValidationError::InvalidEmail { email } => write!(f, "Invalid email format: {}", email),
            ValidationError::InvalidAge { age, min, max } => 
                write!(f, "Age {} is invalid (must be between {} and {})", age, min, max),
        }
    }
}
```

## 🎯 Exercise 3.1 Gentle Hints

### Pattern Goal: User Validation Error Design
**What you're trying to achieve:** Create a ValidationError type that can represent different validation failures with specific information.

**Key Insight:** Each error variant should carry the information needed to understand what went wrong and potentially fix it.

**Gentle Questions to Ask Yourself:**
- What different ways can user validation fail?
- What information would be helpful for each type of failure?
- How should error messages be formatted for users vs developers?
- What data should each error variant store?

**Error Design Pattern:**
```rust
#[derive(Debug)]
enum ValidationError {
    EmptyField { field_name: String },
    InvalidFormat { field: String, value: String, expected: String },
    OutOfRange { field: String, value: i32, min: i32, max: i32 },
}
```

## 🎯 Exercise 3.2 Gentle Hints

### Pattern Goal: Network Error Hierarchy
**What you're trying to achieve:** Model different network failure modes with appropriate error data.

**Key Insight:** Network operations can fail in many ways - connection issues, timeouts, HTTP errors, invalid URLs, etc.

**Gentle Questions:**
- What can go wrong during a network request?
- How do you distinguish between temporary and permanent failures?
- What information helps with debugging network issues?
- How do you represent HTTP status codes and their meanings?

**Network Error Thinking:**
```rust
#[derive(Debug)]
enum NetworkError {
    Timeout,                           // No additional data needed
    ConnectionFailed(String),          // Error message
    HttpError(u16),                   // HTTP status code
    InvalidUrl { url: String, reason: String },  // Structured data
}
```

## 🎯 Exercise 3.3 Gentle Hints

### Pattern Goal: Database Error Classification
**What you're trying to achieve:** Represent database operation failures with recovery hints.

**Key Insight:** Database errors often suggest different recovery strategies - retry for connection issues, don't retry for permission errors.

**Gentle Questions:**
- What database operations can fail and how?
- Which errors suggest retrying vs giving up?
- What information helps diagnose database issues?
- How do you represent things like "user not found" vs "connection failed"?

**Database Error Categories:**
- **Transient errors**: Connection issues, timeouts (retry-able)
- **Permanent errors**: Permission denied, invalid query (not retry-able)
- **Data errors**: Record not found, constraint violations (domain-specific)

## 🎯 Exercise 3.4 Gentle Hints

### Pattern Goal: Application-Level Error Aggregation
**What you're trying to achieve:** Create a top-level error type that can wrap all other error types in your application.

**Key Insight:** You want one error type that can represent any failure in your system, while preserving the original error information.

**Gentle Questions:**
- How do you combine different error types into one unified type?
- How do you preserve error chains for debugging?
- What's the difference between wrapping vs converting errors?
- How do you implement automatic error conversion?

**Application Error Pattern:**
```rust
#[derive(Debug)]
enum AppError {
    Validation(ValidationError),       // Wrap validation errors
    Network(NetworkError),            // Wrap network errors
    Database(DatabaseError),          // Wrap database errors
    Io(std::io::Error),              // Wrap I/O errors
    Parse(std::num::ParseIntError),   // Wrap parsing errors
}
```

## 🔄 General Learning Approach

### Error Design Principles
1. **Be Specific**: Each error variant should represent a distinct failure mode
2. **Include Context**: Store relevant data that helps understand the failure
3. **User-Friendly Messages**: Implement Display for human-readable error messages
4. **Developer-Friendly Debug**: Derive Debug for technical error inspection
5. **Error Chaining**: Implement Error trait with source() for error chains

### Mental Model Shift
**C# Thinking:** "I'll create an exception class for this error type"
**Rust Thinking:** "I'll create an enum variant that captures this failure mode"

### The Error Trait
```rust
impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Return the underlying error if this wraps another error
        match self {
            MyError::WrappedError(e) => Some(e),
            _ => None,
        }
    }
}
```

### Automatic Error Conversion
```rust
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)  // Automatic conversion enables ? operator
    }
}
```

## 💭 Think About It

**Why design custom error types instead of using strings?**
- **Structured Data**: Errors can carry specific information, not just messages
- **Pattern Matching**: Code can handle different errors differently
- **Type Safety**: Compiler ensures you handle all error variants
- **Composability**: Easy to wrap and chain errors
- **Tooling**: Better IDE support and debugging experience

**Error Granularity Trade-offs:**
- **Too Specific**: Many similar variants, complex to handle
- **Too Generic**: Not enough information for proper error handling
- **Just Right**: Each variant represents a meaningfully different failure mode

## 🆘 When You See Compilation Errors

- **"expected `Display`, found `Debug`"** → Implement Display trait for user messages
- **"trait bound not satisfied"** → Implement required traits (Display, Error)
- **"cannot find type in this scope"** → Import std::error::Error
- **"recursive type has infinite size"** → Use Box<dyn Error> for wrapping

## 🔧 Implementation Order

1. **Define Error Enum**: Start with the variants you need
2. **Implement Display**: User-friendly error messages  
3. **Implement Error Trait**: For proper error behavior
4. **Add From Conversions**: For automatic error conversion
5. **Test Error Handling**: Verify your error design works well

## ➡️ Next Level

Ready for specific implementation details? Try [Level 2 Hints](ex03-level2.md) for concrete error type implementations.

Remember: Good error types make failures explicit, informative, and actionable! 🦀