# Exercise 02 - Test Organization: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Missing Test Attribute
```rust
// C# equivalent:
// [Test]
// public void TestMethod() { }

// In Rust, you need:
#[test]
fn test_method() { }
```

The `#[test]` attribute is essential - without it, the function is just a regular function!

### Checkpoint 2: Test Module Configuration
```rust
// This module should only exist during testing:
#[cfg(test)]
mod tests {
    // Test functions go here
}
```

The `#[cfg(test)]` attribute tells Rust to only compile this module when running tests, similar to having a separate test project in C#.

### Checkpoint 3: Testing Private Functions
```rust
// Private function in parent module
fn private_helper() -> i32 { 42 }

#[cfg(test)]
mod tests {
    use super::*;  // Import everything from parent
    
    #[test]
    fn test_private_function() {
        assert_eq!(private_helper(), 42);  // Can access private items!
    }
}
```

Unlike C#, Rust tests in the same file can access private items through `super::*`.

### Checkpoint 4: Import Issues
```rust
// If your module structure is:
// - calculator (module)
//   - operations (submodule)
//     - tests (test module)

// Inside tests module:
use super::*;           // Import from operations
use super::super::*;    // Import from calculator
```

### Checkpoint 5: Dependency Injection Pattern
```rust
// Instead of:
fn get_current_time() -> SystemTime {
    SystemTime::now()  // Hard to test!
}

// Use:
fn process_with_time<F>(time_fn: F) -> String 
where F: Fn() -> SystemTime {
    let time = time_fn();
    // Process with time...
}

// In tests:
#[test]
fn test_with_fixed_time() {
    let result = process_with_time(|| {
        UNIX_EPOCH + Duration::from_secs(1234567890)
    });
}
```

### Checkpoint 6: Test Isolation
```rust
// Problem: Tests share static mutable state
static mut COUNTER: i32 = 0;

// Solution 1: Use thread_local!
thread_local! {
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

// Solution 2: Reset state in each test
#[test]
fn test_something() {
    reset_global_state();  // Always reset first
    // Test logic...
}

// Solution 3: Use serial testing
// Run with: cargo test -- --test-threads=1
```

## 🎯 Pattern Recognition

The key insight: Rust's `#[cfg(test)]` modules are like inner classes that can access private members, while C# typically uses separate test assemblies with `[InternalsVisibleTo]`.

Ready for Level 3 if you need complete solutions!