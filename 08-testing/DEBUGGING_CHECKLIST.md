# Testing Debugging Checklist 🧪

When writing and debugging tests in Rust, watch for these common issues:

## 1. Test Organization Problems

### ❌ Tests in Wrong Location
```rust
// In main.rs - tests won't run!
fn test_something() {
    assert_eq!(2 + 2, 4);
}
```
**Fix:** Use `#[cfg(test)]` module or `tests/` directory

### ❌ Missing Test Attribute
```rust
#[cfg(test)]
mod tests {
    fn test_add() { // Won't run without #[test]!
        assert_eq!(2 + 2, 4);
    }
}
```
**Fix:** Add `#[test]` attribute to test functions

## 2. Assertion Issues

### ❌ Poor Assertion Messages
```rust
assert!(result.is_ok()); // What failed? Why?
```
**Fix:** Use descriptive messages: `assert!(result.is_ok(), "Failed to parse: {:?}", result)`

### ❌ Wrong Assertion Macro
```rust
assert!(expected == actual); // Poor error output
```
**Fix:** Use `assert_eq!` for better failure messages

## 3. Test Isolation

### ❌ Shared Mutable State
```rust
static mut COUNTER: i32 = 0;
#[test]
fn test1() {
    unsafe { COUNTER += 1; } // Tests interfere!
}
```
**Fix:** Each test should be independent

### ❌ File System Conflicts
```rust
#[test]
fn test_file() {
    fs::write("test.txt", "data")?; // Parallel tests conflict!
}
```
**Fix:** Use unique temp files or `tempfile` crate

## 4. Async Testing

### ❌ Forgetting Async Test Macro
```rust
#[test]
async fn test_async() { // Won't work!
    something_async().await;
}
```
**Fix:** Use `#[tokio::test]` or `#[async_std::test]`

### ❌ Blocking in Async Tests
```rust
#[tokio::test]
async fn bad_test() {
    thread::sleep(Duration::from_secs(1)); // Blocks runtime!
}
```
**Fix:** Use `tokio::time::sleep` for async delays

## 5. Mock and Stub Issues

### ❌ No Trait for Mocking
```rust
struct Database { /* fields */ }
fn process(db: Database) { // Can't mock concrete type!
    // ...
}
```
**Fix:** Accept trait objects or generics for testability

### ❌ Over-Mocking
```rust
// Mocking everything makes tests fragile
let mock_everything = Mock::new();
```
**Fix:** Only mock external dependencies

## 6. Integration Test Problems

### ❌ Not Building Test Binaries
```rust
// In tests/integration.rs
use my_internal_module; // Can't access private modules!
```
**Fix:** Only test public API in integration tests

### ❌ Slow Integration Tests
```rust
#[test]
fn test_full_system() {
    // Starts real database, real servers... takes 30s!
}
```
**Fix:** Use test containers or in-memory alternatives

## C# to Rust Testing Patterns

| C# Pattern | Rust Equivalent | Key Differences |
|------------|-----------------|-----------------|
| `[Test]` / `[Fact]` | `#[test]` | Built into language |
| `Assert.AreEqual` | `assert_eq!` | Shows both values |
| `[TestFixture]` | `mod tests` | Module-based organization |
| `Mock<T>` | `mockall` crate | Trait-based mocking |
| `[SetUp]` | Test builder pattern | No built-in fixtures |
| `[TestCase]` | Parameterized tests | Use loops or macros |

## Quick Testing Best Practices

1. **Test Names**: Use descriptive names like `test_parse_invalid_json_returns_error`
2. **Arrange-Act-Assert**: Structure tests clearly
3. **One Assertion**: Ideally one logical assertion per test
4. **Fast Tests**: Unit tests should run in milliseconds
5. **Test Output**: Capture stdout/stderr in tests
6. **Property Testing**: Use `proptest` for generative testing

## Test Coverage Checklist

- [ ] Happy path tested?
- [ ] Error cases tested?
- [ ] Edge cases (empty, null, max values)?
- [ ] Concurrent access tested?
- [ ] Performance regression tests?
- [ ] Documentation examples tested?

## Pro Tips

- Use `cargo test -- --nocapture` to see println! output
- Run specific tests with `cargo test test_name`
- Use `#[should_panic(expected = "message")]` for panic tests
- Generate test coverage with `cargo tarpaulin`
- Use `cargo test --doc` to test documentation examples
- Consider snapshot testing with `insta` crate