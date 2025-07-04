# Exercise 03 - Integration Tests: Level 2 Hints 🌿

## More Specific Guidance

### Checkpoint 1: Integration Test Location
```rust
// Integration tests go in the `tests/` directory at project root:
// project/
//   ├── src/
//   │   └── lib.rs
//   ├── tests/
//   │   ├── integration_test.rs    // Each file is a separate test binary
//   │   └── another_test.rs
//   └── Cargo.toml
```

Each file in `tests/` is compiled as a separate binary that links against your library.

### Checkpoint 2: Importing Your Crate
```rust
// In tests/integration_test.rs:

// Import your library crate (use the name from Cargo.toml)
use my_library::Calculator;
use my_library::Config;

// NOT use crate:: (that's for internal use)
// NOT use super:: (no parent module)

#[test]
fn test_public_api() {
    let calc = Calculator::new();
    assert_eq!(calc.add(2, 2), 4);
}
```

### Checkpoint 3: Test Setup and Teardown
```rust
// Common pattern for test setup:
use std::fs;
use std::path::Path;

fn setup_test_directory() -> String {
    let test_dir = format!("test_output_{}", uuid::Uuid::new_v4());
    fs::create_dir(&test_dir).unwrap();
    test_dir
}

fn cleanup_test_directory(dir: &str) {
    if Path::new(dir).exists() {
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn test_file_operations() {
    let test_dir = setup_test_directory();
    
    // Run your test...
    
    cleanup_test_directory(&test_dir);  // Always cleanup!
}

// Better: Use a guard pattern
struct TestDir(String);
impl Drop for TestDir {
    fn drop(&mut self) {
        cleanup_test_directory(&self.0);
    }
}
```

### Checkpoint 4: Test File Naming
```rust
// Valid integration test files:
// ✅ tests/basic_test.rs
// ✅ tests/api_tests.rs
// ✅ tests/integration.rs

// These won't be run as tests:
// ❌ tests/mod.rs (reserved name)
// ❌ tests/helpers.rs (if in subdirectory)
// ❌ tests/common/utils.rs (subdirectory files)

// Main function isn't needed - Rust creates it
#[test]
fn my_test() {
    // Test code
}
```

### Checkpoint 5: Sharing Test Utilities
```rust
// Create a module in tests/common/mod.rs:
// tests/
//   ├── common/
//   │   └── mod.rs        // Shared test utilities
//   ├── test1.rs
//   └── test2.rs

// In tests/common/mod.rs:
pub fn create_test_config() -> Config {
    Config {
        debug: true,
        timeout: 1000,
    }
}

// In tests/test1.rs:
mod common;
use common::create_test_config;

#[test]
fn test_with_config() {
    let config = create_test_config();
    // Use config...
}
```

### Checkpoint 6: Database Test Isolation
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Ensure database tests run serially
static DB_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[test]
fn test_database_operation_1() {
    let _lock = DB_MUTEX.lock().unwrap();
    
    // Setup: Create test database/schema
    setup_test_database();
    
    // Test your database operations
    
    // Cleanup: Always clean up!
    teardown_test_database();
}

// Alternative: Use serial_test crate
#[serial_test::serial]
#[test]
fn test_database_operation_2() {
    // This test won't run in parallel with other [serial] tests
}
```

## 🎯 Pattern Recognition

Integration tests in Rust are like separate console applications that reference your library - they can only use the public API, just like external users of your crate!

Ready for Level 3 if you need complete solutions!