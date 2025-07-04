# C# to Rust Testing: A Complete Guide 🔄

*For experienced C# developers transitioning to Rust testing*

## 🚀 Quick Start Cheat Sheet

### Most Common Patterns (Copy & Paste Ready)

```rust
// Basic test structure
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }
}

// Async test
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert!(result.is_ok());
}

// Mock with mockall
#[automock]
trait MyService {
    fn do_something(&self, input: &str) -> Result<String, Error>;
}

// Property test
proptest! {
    #[test]
    fn test_property(input: String) {
        prop_assert!(my_function(&input).len() <= input.len());
    }
}
```

### Emergency Translation Guide

| **When you want to...** | **C# Way** | **Rust Way** |
|-------------------------|------------|--------------|
| Mark a test | `[Test]` or `[Fact]` | `#[test]` |
| Test async code | `async Task` method | `#[tokio::test] async fn` |
| Assert equality | `Assert.AreEqual(expected, actual)` | `assert_eq!(actual, expected)` ⚠️ *Order flipped!* |
| Assert true | `Assert.IsTrue(condition)` | `assert!(condition)` |
| Test exceptions | `Assert.Throws<T>(() => {})` | `#[should_panic]` or `matches!(result, Err(_))` |
| Mock interfaces | `Mock<IService>` | `#[automock] trait Service` |
| Parameterized tests | `[Theory] [InlineData]` | Loop with test cases or `proptest!` |

### 30-Second Setup Checklist

```toml
# Add to Cargo.toml [dev-dependencies]
tokio = { version = "1.0", features = ["test-util", "macros"] }  # For async
mockall = "0.11"           # For mocking  
proptest = "1.0"           # For property testing
```

---

## Quick Reference Table

| C# Concept | Rust Equivalent | Notes |
|------------|-----------------|-------|
| `[Test]` / `[Fact]` | `#[test]` | Basic test marking |
| `[TestMethod]` | `#[test]` | Same concept, different syntax |
| `[SetUp]` / `[TearDown]` | Manual setup/Drop trait | No built-in attributes |
| `Assert.AreEqual(a, b)` | `assert_eq!(a, b)` | Built into language |
| `Assert.IsTrue(condition)` | `assert!(condition)` | Built into language |
| `StringAssert.Contains()` | Custom macro/method | No built-in, easy to create |
| `[TestCategory]` | `#[cfg(test)]` groups | Different organization |
| `[Ignore]` | `#[ignore]` | Same functionality |
| `[ExpectedException]` | `#[should_panic]` | Similar but more limited |
| `TestContext` | No equivalent | Different approach needed |

## Testing Framework Equivalents

### Unit Testing Frameworks

| C# Framework | Rust Equivalent | Key Differences |
|--------------|-----------------|-----------------|
| **xUnit** | Built-in `#[test]` | Rust has testing built into language |
| **NUnit** | Built-in `#[test]` | No test fixtures, use modules |
| **MSTest** | Built-in `#[test]` | Simpler attribute model |

### Assertion Libraries

| C# Library | Rust Equivalent | Usage Pattern |
|------------|-----------------|---------------|
| **FluentAssertions** | Custom macros | `assert_contains!()`, `assert_between!()` |
| **Shouldly** | Custom macros | Build your own fluent API |
| **Built-in Assert** | `assert_eq!`, `assert!` | More concise syntax |

### Mocking Frameworks

| C# Framework | Rust Equivalent | Key Differences |
|--------------|-----------------|-----------------|
| **Moq** | `mockall` crate | Trait-based, compile-time |
| **NSubstitute** | `mockall` crate | More explicit setup |
| **FakeItEasy** | Manual mocks | Often manual is simpler |

### Property Testing

| C# Library | Rust Equivalent | Maturity |
|------------|-----------------|-----------|
| **FsCheck** | `proptest` | Very mature |
| **AutoFixture** | `proptest` + custom generators | More explicit |

## Detailed Comparisons

### 1. Basic Test Structure

**C# (xUnit):**
```csharp
public class CalculatorTests
{
    [Fact]
    public void Add_TwoPositiveNumbers_ReturnsSum()
    {
        // Arrange
        var calculator = new Calculator();
        
        // Act
        var result = calculator.Add(2, 3);
        
        // Assert
        Assert.Equal(5, result);
    }
    
    [Theory]
    [InlineData(1, 2, 3)]
    [InlineData(0, 0, 0)]
    [InlineData(-1, 1, 0)]
    public void Add_VariousInputs_ReturnsCorrectSum(int a, int b, int expected)
    {
        var calculator = new Calculator();
        var result = calculator.Add(a, b);
        Assert.Equal(expected, result);
    }
}
```

**Rust:**
```rust
mod tests {
    use super::*;
    
    #[test]
    fn add_two_positive_numbers_returns_sum() {
        // Arrange
        let calculator = Calculator::new();
        
        // Act
        let result = calculator.add(2, 3);
        
        // Assert
        assert_eq!(result, 5);
    }
    
    #[test]
    fn add_various_inputs_returns_correct_sum() {
        let test_cases = vec![
            (1, 2, 3),
            (0, 0, 0),
            (-1, 1, 0),
        ];
        
        let calculator = Calculator::new();
        for (a, b, expected) in test_cases {
            assert_eq!(calculator.add(a, b), expected);
        }
    }
}
```

**Key Differences:**
- Rust tests live in the same file (with `#[cfg(test)]`)
- No built-in parameterized tests, use loops or `proptest`
- Snake_case naming convention instead of PascalCase

### 2. Test Organization

**C# Test Project Structure:**
```
MyApp.Tests/
├── UnitTests/
│   ├── Services/
│   │   ├── UserServiceTests.cs
│   │   └── EmailServiceTests.cs
│   └── Models/
│       └── UserTests.cs
├── IntegrationTests/
│   ├── Api/
│   │   └── UserControllerTests.cs
│   └── Database/
│       └── UserRepositoryTests.cs
└── TestHelpers/
    ├── Builders/
    │   └── UserBuilder.cs
    └── Mocks/
        └── MockEmailService.cs
```

**Rust Test Organization:**
```
src/
├── lib.rs                    // Re-exports
├── services/
│   ├── mod.rs
│   ├── user_service.rs       // Tests at bottom of file
│   └── email_service.rs      // Tests at bottom of file
├── models/
│   ├── mod.rs
│   └── user.rs              // Tests at bottom of file
└── test_helpers/            // Optional separate module
    ├── mod.rs
    ├── builders.rs
    └── mocks.rs
tests/                       // Integration tests
├── api_tests.rs
├── database_tests.rs
└── common/                  // Shared test utilities
    └── mod.rs
```

**Key Differences:**
- Unit tests colocated with source code
- Integration tests in separate `tests/` directory
- Shared utilities in `tests/common/mod.rs`

### 3. Assertions and Expectations

**C# Assertion Patterns:**
```csharp
// Basic assertions
Assert.Equal(expected, actual);
Assert.True(condition);
Assert.False(condition);
Assert.Null(value);
Assert.NotNull(value);

// Collection assertions
Assert.Contains(item, collection);
Assert.DoesNotContain(item, collection);
Assert.Empty(collection);
Assert.Single(collection);

// Exception assertions
Assert.Throws<ArgumentException>(() => method());

// FluentAssertions style
result.Should().Be(42);
collection.Should().HaveCount(3);
action.Should().Throw<InvalidOperationException>();
```

**Rust Assertion Patterns:**
```rust
// Basic assertions
assert_eq!(actual, expected);          // Note: order reversed!
assert_ne!(actual, unexpected);
assert!(condition);
assert!(!condition);

// Option/Result assertions
assert!(option.is_some());
assert!(option.is_none());
assert!(result.is_ok());
assert!(result.is_err());

// Custom assertions (you build these)
assert_contains!("hello world", "world");
assert_between!(value, 1, 10);

// Panic assertions
#[test]
#[should_panic(expected = "argument")]
fn test_panic() {
    panic_function();
}

// Advanced pattern matching
assert!(matches!(result, Ok(42)));
assert!(matches!(error, MyError::ValidationFailed(_)));
```

**Key Differences:**
- Parameter order often reversed (`actual, expected` vs `expected, actual`)
- More explicit about Option/Result handling
- Pattern matching provides powerful assertion capabilities

### 4. Mocking and Dependency Injection

**C# with Moq:**
```csharp
[Test]
public void ProcessOrder_PaymentFails_ThrowsException()
{
    // Arrange
    var mockPaymentService = new Mock<IPaymentService>();
    mockPaymentService
        .Setup(x => x.ProcessPayment(It.IsAny<decimal>()))
        .Throws<PaymentException>();
    
    var orderService = new OrderService(mockPaymentService.Object);
    var order = new Order { Amount = 100 };
    
    // Act & Assert
    Assert.Throws<PaymentException>(() => orderService.Process(order));
    
    // Verify
    mockPaymentService.Verify(x => x.ProcessPayment(100), Times.Once);
}
```

**Rust with mockall:**
```rust
#[automock]
trait PaymentService {
    fn process_payment(&self, amount: f64) -> Result<(), PaymentError>;
}

#[test]
fn process_order_payment_fails_returns_error() {
    // Arrange
    let mut mock_payment = MockPaymentService::new();
    mock_payment
        .expect_process_payment()
        .with(eq(100.0))
        .times(1)
        .returning(|_| Err(PaymentError::Declined));
    
    let order_service = OrderService::new(Box::new(mock_payment));
    let order = Order { amount: 100.0 };
    
    // Act
    let result = order_service.process(&order);
    
    // Assert
    assert!(matches!(result, Err(OrderError::PaymentFailed(_))));
}
```

**Key Differences:**
- Traits instead of interfaces (but similar concept)
- Compile-time mock generation with `#[automock]`
- More explicit about ownership (Box<dyn Trait>)
- Return types are explicit (Result<T, E> instead of exceptions)

### 5. Async Testing

**C# Async Tests:**
```csharp
[Test]
public async Task GetUserAsync_ValidId_ReturnsUser()
{
    // Arrange
    var userService = new UserService();
    
    // Act
    var user = await userService.GetUserAsync(1);
    
    // Assert
    Assert.NotNull(user);
    Assert.Equal(1, user.Id);
}

[Test]
public async Task ProcessBatch_LargeData_CompletesWithinTimeout()
{
    // Arrange
    var processor = new DataProcessor();
    var data = GenerateLargeDataSet();
    
    // Act & Assert
    using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(30));
    await processor.ProcessBatchAsync(data, cts.Token);
}
```

**Rust Async Tests:**
```rust
#[tokio::test]
async fn get_user_valid_id_returns_user() {
    // Arrange
    let user_service = UserService::new();
    
    // Act
    let user = user_service.get_user(1).await;
    
    // Assert
    assert!(user.is_ok());
    let user = user.unwrap();
    assert_eq!(user.id, 1);
}

#[tokio::test]
async fn process_batch_large_data_completes_within_timeout() {
    // Arrange
    let processor = DataProcessor::new();
    let data = generate_large_data_set();
    
    // Act & Assert
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        processor.process_batch(data)
    ).await;
    
    assert!(result.is_ok());
}
```

**Key Differences:**
- `#[tokio::test]` instead of async Task methods
- Explicit timeout handling with `tokio::time::timeout`
- No built-in cancellation tokens (use `tokio::select!` or channels)

### 6. Test Data and Fixtures

**C# Test Builder Pattern:**
```csharp
public class UserBuilder
{
    private User _user = new User();
    
    public UserBuilder WithName(string name)
    {
        _user.Name = name;
        return this;
    }
    
    public UserBuilder WithAge(int age)
    {
        _user.Age = age;
        return this;
    }
    
    public User Build() => _user;
}

// Usage
var user = new UserBuilder()
    .WithName("Alice")
    .WithAge(30)
    .Build();
```

**Rust Test Builder Pattern:**
```rust
pub struct UserBuilder {
    name: String,
    age: u32,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            name: "Default User".to_string(),
            age: 25,
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    
    pub fn with_age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }
    
    pub fn build(self) -> User {
        User {
            name: self.name,
            age: self.age,
        }
    }
}

// Usage
let user = UserBuilder::new()
    .with_name("Alice")
    .with_age(30)
    .build();
```

**Key Differences:**
- Move semantics instead of mutation (self vs &mut self)
- `impl Into<String>` for flexible string handling
- Consuming `build()` method

### 7. Property-Based Testing

**C# with FsCheck:**
```csharp
[Property]
public bool ReverseReverse_IsIdentity(int[] array)
{
    var reversed = array.Reverse().Reverse().ToArray();
    return array.SequenceEqual(reversed);
}

[Property]
public bool Sort_PreservesLength(int[] array)
{
    var sorted = array.OrderBy(x => x).ToArray();
    return array.Length == sorted.Length;
}
```

**Rust with proptest:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn reverse_reverse_is_identity(array: Vec<i32>) {
        let mut reversed = array.clone();
        reversed.reverse();
        reversed.reverse();
        prop_assert_eq!(array, reversed);
    }
    
    #[test] 
    fn sort_preserves_length(mut array: Vec<i32>) {
        let original_len = array.len();
        array.sort();
        prop_assert_eq!(array.len(), original_len);
    }
}
```

**Key Differences:**
- `proptest!` macro instead of attributes
- Explicit clone() for immutable operations
- `prop_assert_eq!` instead of returning bool

## Common Migration Patterns

### Pattern 1: Test Class → Test Module

**C#:**
```csharp
public class CalculatorTests
{
    private readonly Calculator _calculator;
    
    public CalculatorTests()
    {
        _calculator = new Calculator();
    }
    
    [Test]
    public void Add_ReturnsSum() { /* ... */ }
    
    [Test] 
    public void Subtract_ReturnsDifference() { /* ... */ }
}
```

**Rust:**
```rust
#[cfg(test)]
mod calculator_tests {
    use super::*;
    
    fn setup() -> Calculator {
        Calculator::new()
    }
    
    #[test]
    fn add_returns_sum() {
        let calculator = setup();
        // test implementation
    }
    
    #[test]
    fn subtract_returns_difference() {
        let calculator = setup();
        // test implementation
    }
}
```

### Pattern 2: Exception Testing → Panic Testing

**C#:**
```csharp
[Test]
public void Divide_ByZero_ThrowsException()
{
    var calculator = new Calculator();
    Assert.Throws<DivideByZeroException>(() => calculator.Divide(10, 0));
}
```

**Rust:**
```rust
#[test]
#[should_panic(expected = "division by zero")]
fn divide_by_zero_panics() {
    let calculator = Calculator::new();
    calculator.divide(10, 0); // Should panic
}

// Or better - test Result types:
#[test]
fn divide_by_zero_returns_error() {
    let calculator = Calculator::new();
    let result = calculator.safe_divide(10, 0);
    assert!(matches!(result, Err(CalculatorError::DivisionByZero)));
}
```

### Pattern 3: Interface Mocking → Trait Mocking

**C#:**
```csharp
public interface IEmailService
{
    Task<bool> SendEmailAsync(string to, string subject, string body);
}

// In tests:
var mockEmail = new Mock<IEmailService>();
mockEmail.Setup(x => x.SendEmailAsync(It.IsAny<string>(), It.IsAny<string>(), It.IsAny<string>()))
         .ReturnsAsync(true);
```

**Rust:**
```rust
#[automock]
trait EmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError>;
}

// In tests:
let mut mock_email = MockEmailService::new();
mock_email
    .expect_send_email()
    .returning(|_, _, _| Ok(()));
```

## Best Practices for C# Developers

### 1. Embrace Result Types
Instead of exceptions, use `Result<T, E>`:

```rust
// Instead of throwing exceptions
fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

#[test]
fn test_divide_success() {
    let result = divide(10.0, 2.0);
    assert_eq!(result, Ok(5.0));
}

#[test]
fn test_divide_by_zero() {
    let result = divide(10.0, 0.0);
    assert!(matches!(result, Err(MathError::DivisionByZero)));
}
```

### 2. Use Pattern Matching for Complex Assertions

```rust
#[test]
fn test_user_creation() {
    let result = create_user("Alice", 25);
    
    match result {
        Ok(user) => {
            assert_eq!(user.name, "Alice");
            assert_eq!(user.age, 25);
            assert!(user.id > 0);
        },
        Err(e) => panic!("Expected success, got error: {:?}", e),
    }
}
```

### 3. Leverage Rust's Type System for Test Safety

```rust
// Use type-safe test data
#[derive(Debug)]
struct TestUser {
    name: String,
    age: u32,
}

impl TestUser {
    fn valid() -> Self {
        Self {
            name: "Valid User".to_string(),
            age: 25,
        }
    }
    
    fn invalid_age() -> Self {
        Self {
            name: "Invalid User".to_string(),
            age: 0, // Invalid
        }
    }
}
```

### 4. Use Modules for Test Organization

```rust
mod user_service_tests {
    use super::*;
    
    mod creation_tests {
        use super::*;
        
        #[test]
        fn creates_user_with_valid_data() { /* ... */ }
        
        #[test]
        fn rejects_invalid_email() { /* ... */ }
    }
    
    mod update_tests {
        use super::*;
        
        #[test]
        fn updates_existing_user() { /* ... */ }
        
        #[test]
        fn rejects_invalid_update() { /* ... */ }
    }
}
```

## Migration Checklist

- [ ] Convert test classes to test modules
- [ ] Replace `[Test]` with `#[test]`
- [ ] Move from exceptions to Result types
- [ ] Update assertion patterns (parameter order!)
- [ ] Convert interfaces to traits for mocking
- [ ] Update async test patterns to use `#[tokio::test]`
- [ ] Migrate test builders to use move semantics
- [ ] Replace parameterized tests with loops or proptest
- [ ] Organize integration tests in `tests/` directory
- [ ] Set up test utilities in appropriate modules

## 🔧 Common Issues & Quick Fixes

### "My test won't run!"
```rust
// ❌ Missing attribute
fn test_something() { }

// ✅ Add test attribute  
#[test]
fn test_something() { }
```

### "Can't access private functions in tests!"
```rust
// ✅ Put tests in same file with #[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;  // Now you can access private items!
    
    #[test]
    fn test_private_function() {
        assert_eq!(private_helper(), 42);
    }
}
```

### "Async test not working!"
```rust
// ❌ Regular test with async
#[test]
async fn test_async() { }  // Won't compile!

// ✅ Use tokio::test
#[tokio::test]
async fn test_async() { }
```

### "Mock expectations failing!"
```rust
// ✅ Remember: mockall verifies on drop
#[test]
fn test_with_mock() {
    let mut mock = MockService::new();
    mock.expect_call().times(1);
    
    // Use mock here
    service.do_something(&mock);
    
    // Verification happens automatically when mock drops
}
```

### "Assert parameter order confusion!"
```rust
// C# style (will compile but fail confusingly)
assert_eq!(expected, actual);  // ❌ Backwards!

// Rust style (recommended)
assert_eq!(actual, expected);  // ✅ Actual first!
```

---

## 📝 Migration Checklist

**Phase 1: Basic Setup**
- [ ] Add `#[cfg(test)]` modules to source files
- [ ] Convert `[Test]` to `#[test]`
- [ ] Update assertion parameter order
- [ ] Add tokio for async tests

**Phase 2: Advanced Patterns**
- [ ] Convert interfaces to traits for mocking
- [ ] Migrate test builders to use move semantics
- [ ] Update async test patterns
- [ ] Set up integration tests in `tests/` directory

**Phase 3: Optimization**
- [ ] Organize test utilities in common modules
- [ ] Implement property-based tests for algorithms
- [ ] Set up CI with proper test coverage
- [ ] Add custom assertion helpers

---

## Conclusion

While Rust testing has a different philosophy than C#, the core concepts translate well. The main adjustments are:

1. **Explicit error handling** with Result types instead of exceptions
2. **Trait-based mocking** instead of interface mocking
3. **Colocated unit tests** instead of separate test projects
4. **Pattern matching** for complex assertions
5. **Move semantics** in test builders

The learning curve is manageable, and many patterns will feel familiar once you understand Rust's ownership and type system!