# Exercise 02 - Test Organization: Level 3 Hints 🌳

## Complete Solutions

### Checkpoint 1: Complete Fix
```rust
// Add the #[test] attribute:
#[test]
fn test_string_parsing() {
    let result = parse_number("42");
    assert_eq!(result, Ok(42));
}
```

### Checkpoint 2: Complete Module Structure
```rust
// Add #[cfg(test)] to the module:
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid"));
    }
}
```

### Checkpoint 3: Testing Private Functions
```rust
fn complex_calculation(x: i32, y: i32) -> i32 {
    helper_function(x) + helper_function(y)
}

fn helper_function(n: i32) -> i32 {
    n * n + 2 * n + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_helper_function() {
        assert_eq!(helper_function(0), 1);
        assert_eq!(helper_function(1), 4);
        assert_eq!(helper_function(2), 9);
    }
    
    #[test]
    fn test_complex_calculation() {
        assert_eq!(complex_calculation(1, 2), 13);
    }
}
```

### Checkpoint 4: Complete Import Solution
```rust
pub struct User {
    pub name: String,
    pub age: u32,
}

pub mod validation {
    use super::User;
    
    pub fn validate_user(user: &User) -> Result<(), String> {
        if user.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if user.age < 18 {
            return Err("User must be 18 or older".to_string());
        }
        Ok(())
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::User;  // Import User from parent's parent
        
        #[test]
        fn test_valid_user() {
            let user = User {
                name: "Alice".to_string(),
                age: 25,
            };
            assert!(validate_user(&user).is_ok());
        }
    }
}
```

### Checkpoint 5: Testable Time-Dependent Code
```rust
use std::time::SystemTime;

pub trait Clock {
    fn now(&self) -> SystemTime;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

pub fn create_timestamped_message(message: &str, clock: &dyn Clock) -> String {
    let timestamp = clock.now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("[{}] {}", timestamp, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    struct MockClock {
        time: SystemTime,
    }
    
    impl Clock for MockClock {
        fn now(&self) -> SystemTime {
            self.time
        }
    }
    
    #[test]
    fn test_timestamped_message() {
        let mock_clock = MockClock {
            time: UNIX_EPOCH + Duration::from_secs(1000),
        };
        
        let result = create_timestamped_message("Hello", &mock_clock);
        assert_eq!(result, "[1000] Hello");
    }
}
```

### Checkpoint 6: Isolated Parallel Tests
```rust
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Use Lazy for thread-safe initialization
static SHARED_RESOURCE: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

pub fn add_to_log(message: &str) {
    let mut log = SHARED_RESOURCE.lock().unwrap();
    log.push(message.to_string());
}

pub fn get_log_count() -> usize {
    let log = SHARED_RESOURCE.lock().unwrap();
    log.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper to reset state for each test
    fn reset_log() {
        let mut log = SHARED_RESOURCE.lock().unwrap();
        log.clear();
    }
    
    #[test]
    fn test_add_to_log() {
        reset_log();  // Always reset first!
        
        add_to_log("First");
        add_to_log("Second");
        
        assert_eq!(get_log_count(), 2);
    }
    
    #[test]
    fn test_empty_log() {
        reset_log();  // Always reset first!
        
        assert_eq!(get_log_count(), 0);
    }
}
```

## 🎉 Congratulations!

You've mastered Rust test organization! Key takeaways:
- `#[test]` marks test functions
- `#[cfg(test)]` creates test-only modules
- Tests can access private items in the same file
- Dependency injection enables testable code
- Always consider test isolation in parallel execution

This is quite different from C#'s approach but achieves the same goals of organized, maintainable tests!