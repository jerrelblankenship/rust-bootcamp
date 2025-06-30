# Exercise 1 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working Option<T> implementations. Here's the full solution with all exercises implemented.

## 📝 Complete ex01-option-basics.rs Implementation

```rust
// Exercise 1: Option<T> Basics - Complete Solutions
//
// This file demonstrates comprehensive Option handling patterns

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Option<T> Basics (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_1_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_1_5();
    exercise_1_6();
    exercise_1_7();
    
    println!("\n🎉 All exercises completed successfully!");
}

// Exercise 1.1: Basic Option handling - SOLVED
fn exercise_1_1() {
    println!("Exercise 1.1: Basic Option handling");
    
    // SOLUTION: Handle Option<User> and extract name with fallback
    fn get_user_name(user_id: u32) -> String {
        match find_user(user_id) {
            Some(user) => user.name,
            None => "Unknown".to_string(),
        }
    }
    
    // Test with existing and non-existing users
    println!("User 1: {}", get_user_name(1));
    println!("User 999: {}", get_user_name(999));
    
    println!("✅ Exercise 1.1 complete\n");
}

// Exercise 1.2: Option chaining - SOLVED
fn exercise_1_2() {
    println!("Exercise 1.2: Option chaining");
    
    // SOLUTION: Chain Option operations to extract email domain
    fn get_user_email_domain(user_id: u32) -> Option<String> {
        find_user(user_id)
            .map(|user| user.email)              // Extract email from user
            .and_then(|email| {                 // Chain another Option operation
                email.split_once('@')           // Split at '@' symbol
                    .map(|(_, domain)| domain.to_string())  // Extract domain part
            })
    }
    
    // Test cases
    match get_user_email_domain(1) {
        Some(domain) => println!("User 1 email domain: {}", domain),
        None => println!("User 1 email domain not found"),
    }
    
    match get_user_email_domain(999) {
        Some(domain) => println!("User 999 email domain: {}", domain),
        None => println!("User 999 email domain not found"),
    }
    
    println!("✅ Exercise 1.2 complete\n");
}

// Exercise 1.3: Working with collections and Option - SOLVED
fn exercise_1_3() {
    println!("Exercise 1.3: Working with collections and Option");
    
    let scores = vec![Some(85), None, Some(92), Some(78), None, Some(88)];
    
    // SOLUTION: Calculate average of valid scores only
    fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
        let valid_scores: Vec<i32> = scores
            .iter()
            .filter_map(|&score| score)  // Extract only Some values
            .collect();
        
        if valid_scores.is_empty() {
            None  // No valid scores
        } else {
            let sum: i32 = valid_scores.iter().sum();
            let average = sum as f64 / valid_scores.len() as f64;
            Some(average)
        }
    }
    
    match calculate_average(&scores) {
        Some(avg) => println!("Average score: {:.2}", avg),
        None => println!("No scores available"),
    }
    
    println!("✅ Exercise 1.3 complete\n");
}

// Exercise 1.4: Option with error handling - SOLVED
fn exercise_1_4() {
    println!("Exercise 1.4: Option with error handling");
    
    // SOLUTION: Convert Result to Option for parsing
    fn safe_parse_int(s: &str) -> Option<i32> {
        s.parse().ok()  // Convert Result<i32, ParseIntError> to Option<i32>
    }
    
    // SOLUTION: Safe division with zero check
    fn safe_divide(a: i32, b: i32) -> Option<f64> {
        if b == 0 {
            None
        } else {
            Some(a as f64 / b as f64)
        }
    }
    
    // SOLUTION: Chain parsing and division operations
    fn parse_and_divide(a_str: &str, b_str: &str) -> Option<f64> {
        let a = safe_parse_int(a_str)?;  // Early return if parse fails
        let b = safe_parse_int(b_str)?;  // Early return if parse fails
        safe_divide(a, b)                // Handle division by zero
    }
    
    // Test cases
    let test_cases = [
        ("10", "2"),
        ("15", "3"),
        ("20", "0"),  // Division by zero
        ("abc", "5"), // Parse error
        ("8", "def"), // Parse error
    ];
    
    for (a, b) in test_cases {
        match parse_and_divide(a, b) {
            Some(result) => println!("{} / {} = {:.2}", a, b, result),
            None => println!("{} / {} = Error", a, b),
        }
    }
    
    println!("✅ Exercise 1.4 complete\n");
}

// Exercise 1.5: Option in structs - SOLVED
fn exercise_1_5() {
    println!("Exercise 1.5: Option in structs");
    
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,  // Email is optional
        phone: Option<String>,  // Phone is optional
    }
    
    impl Person {
        // SOLUTION: Return preferred contact method with fallback
        fn preferred_contact(&self) -> Option<&str> {
            self.email
                .as_deref()                           // Convert Option<String> to Option<&str>
                .or_else(|| self.phone.as_deref())    // Fallback to phone
        }
        
        // SOLUTION: Format contact info based on available fields
        fn contact_info(&self) -> String {
            match (&self.email, &self.phone) {
                (Some(email), _) => format!("{} <{}>", self.name, email),
                (None, Some(phone)) => format!("{} ({})", self.name, phone),
                (None, None) => self.name.clone(),
            }
        }
    }
    
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
            phone: Some("555-1234".to_string()),
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            email: None,
            phone: Some("555-5678".to_string()),
        },
        Person {
            name: "Carol".to_string(),
            age: 35,
            email: Some("carol@example.com".to_string()),
            phone: None,
        },
        Person {
            name: "Dave".to_string(),
            age: 28,
            email: None,
            phone: None,
        },
    ];
    
    for person in &people {
        println!("Contact info: {}", person.contact_info());
        match person.preferred_contact() {
            Some(contact) => println!("  Preferred: {}", contact),
            None => println!("  No contact available"),
        }
    }
    
    println!("✅ Exercise 1.5 complete\n");
}

// Exercise 1.6: Converting between Option and Result - SOLVED
fn exercise_1_6() {
    println!("Exercise 1.6: Converting between Option and Result");
    
    // SOLUTION: Convert Option to Result with custom error
    fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
        opt.ok_or_else(|| error_msg.to_string())
    }
    
    // SOLUTION: Convert Result to Option, discarding error
    fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
        res.ok()
    }
    
    // Test the conversions
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("Something went wrong");
    
    // Convert Option to Result
    println!("Some(42) -> Result: {:?}", option_to_result(some_value, "No value"));
    println!("None -> Result: {:?}", option_to_result(none_value, "No value"));
    
    // Convert Result to Option
    println!("Ok(100) -> Option: {:?}", result_to_option(ok_result));
    println!("Err -> Option: {:?}", result_to_option(err_result));
    
    println!("✅ Exercise 1.6 complete\n");
}

// Exercise 1.7: Option with iterators - SOLVED
fn exercise_1_7() {
    println!("Exercise 1.7: Option with iterators");
    
    let usernames = vec!["alice", "bob", "", "carol", "dave", ""];
    
    // SOLUTION: Filter out empty usernames and collect valid ones
    fn get_valid_usernames(usernames: &[&str]) -> Vec<String> {
        usernames
            .iter()
            .filter_map(|&name| {
                if name.is_empty() {
                    None  // Skip empty names
                } else {
                    Some(name.to_string())  // Keep valid names
                }
            })
            .collect()
    }
    
    // SOLUTION: Find first valid username starting with given letter
    fn find_user_starting_with(usernames: &[&str], letter: char) -> Option<String> {
        usernames
            .iter()
            .filter_map(|&name| {
                if !name.is_empty() && name.starts_with(letter) {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .next()  // Get first match
    }
    
    // SOLUTION: Get lengths of all valid usernames
    fn get_username_lengths(usernames: &[&str]) -> Vec<usize> {
        usernames
            .iter()
            .filter_map(|&name| {
                if name.is_empty() {
                    None
                } else {
                    Some(name.len())
                }
            })
            .collect()
    }
    
    let valid_users = get_valid_usernames(&usernames);
    println!("Valid usernames: {:?}", valid_users);
    
    match find_user_starting_with(&usernames, 'c') {
        Some(user) => println!("Found user starting with 'c': {}", user),
        None => println!("No user found starting with 'c'"),
    }
    
    let lengths = get_username_lengths(&usernames);
    println!("Username lengths: {:?}", lengths);
    
    println!("✅ Exercise 1.7 complete\n");
}

// Helper functions for the exercises (these work correctly!)
fn find_user(user_id: u32) -> Option<User> {
    let users = get_test_users();
    users.into_iter().find(|u| u.id == user_id)
}

fn get_test_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@test.org".to_string(),
        },
        User {
            id: 3,
            name: "Carol".to_string(),
            email: "carol@company.net".to_string(),
        },
    ]
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Demonstration of advanced Option patterns
fn demonstrate_advanced_patterns() {
    println!("=== Advanced Option Patterns ===");
    
    // Pattern 1: Option flattening
    let nested_option: Option<Option<i32>> = Some(Some(42));
    let flattened: Option<i32> = nested_option.flatten();
    println!("Nested Option flattened: {:?}", flattened);
    
    // Pattern 2: Combining multiple Options
    let opt1 = Some(10);
    let opt2 = Some(20);
    let combined = opt1.zip(opt2).map(|(a, b)| a + b);
    println!("Combined Options: {:?}", combined);
    
    // Pattern 3: Option with filter
    let maybe_number = Some(42);
    let filtered = maybe_number.filter(|&n| n > 50);
    println!("Filtered Option: {:?}", filtered);
    
    // Pattern 4: Option chain with multiple fallbacks
    let primary: Option<String> = None;
    let secondary: Option<String> = None;
    let fallback: Option<String> = Some("fallback".to_string());
    
    let result = primary
        .or(secondary)
        .or(fallback)
        .unwrap_or_else(|| "default".to_string());
    println!("Fallback chain result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_user() {
        assert!(find_user(1).is_some());
        assert!(find_user(999).is_none());
    }
    
    #[test]
    fn test_get_user_name() {
        fn get_user_name(user_id: u32) -> String {
            match find_user(user_id) {
                Some(user) => user.name,
                None => "Unknown".to_string(),
            }
        }
        
        assert_eq!(get_user_name(1), "Alice");
        assert_eq!(get_user_name(999), "Unknown");
    }
    
    #[test]
    fn test_email_domain_extraction() {
        fn get_user_email_domain(user_id: u32) -> Option<String> {
            find_user(user_id)
                .map(|user| user.email)
                .and_then(|email| {
                    email.split_once('@')
                        .map(|(_, domain)| domain.to_string())
                })
        }
        
        assert_eq!(get_user_email_domain(1), Some("example.com".to_string()));
        assert_eq!(get_user_email_domain(999), None);
    }
    
    #[test]
    fn test_safe_parsing() {
        fn safe_parse_int(s: &str) -> Option<i32> {
            s.parse().ok()
        }
        
        assert_eq!(safe_parse_int("42"), Some(42));
        assert_eq!(safe_parse_int("abc"), None);
    }
    
    #[test]
    fn test_option_conversions() {
        fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
            opt.ok_or_else(|| error_msg.to_string())
        }
        
        assert_eq!(option_to_result(Some(42), "error"), Ok(42));
        assert_eq!(option_to_result(None::<i32>, "error"), Err("error".to_string()));
    }
    
    #[test]
    fn test_valid_usernames() {
        fn get_valid_usernames(usernames: &[&str]) -> Vec<String> {
            usernames
                .iter()
                .filter_map(|&name| {
                    if name.is_empty() {
                        None
                    } else {
                        Some(name.to_string())
                    }
                })
                .collect()
        }
        
        let input = vec!["alice", "", "bob", ""];
        let result = get_valid_usernames(&input);
        assert_eq!(result, vec!["alice".to_string(), "bob".to_string()]);
    }
}
```

## 🎓 Key Learning Points from Complete Solutions

### Option Handling Patterns
1. **Basic Handling**: `match` for explicit control, `.unwrap_or()` for simple defaults
2. **Chaining**: `.map()` for transformations, `.and_then()` for Option-returning operations
3. **Early Returns**: `?` operator for clean error propagation
4. **Conversions**: `.ok()` for Result→Option, `.ok_or()` for Option→Result

### Iterator Integration
```rust
// Power pattern: filter_map for processing collections with optional values
let valid_items: Vec<ProcessedItem> = items
    .iter()
    .filter_map(|item| process_item(item).ok())  // Keep only successful processing
    .collect();
```

### Struct Field Handling
```rust
// Pattern: Handle multiple optional fields with tuple matching
match (&self.primary, &self.secondary) {
    (Some(primary), _) => use_primary(primary),
    (None, Some(secondary)) => use_secondary(secondary),
    (None, None) => use_default(),
}
```

### Performance Considerations
```rust
// Efficient: Use references where possible
option.as_ref().map(|s| s.len())         // Don't move the String
option.as_deref()                        // Convert Option<String> to Option<&str>

// Lazy evaluation: Use closures for expensive defaults
option.unwrap_or_else(|| expensive_computation())
```

## 🚀 Production-Ready Patterns

### Error Context with Option
```rust
fn get_config_value(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()                            // Convert Result to Option
        .filter(|s| !s.is_empty())       // Filter out empty values
        .or_else(|| load_from_config_file(key))  // Fallback to config file
}
```

### Option with Validation
```rust
fn validate_email(email: Option<String>) -> Option<String> {
    email
        .filter(|e| e.contains('@') && e.contains('.'))  // Basic validation
        .map(|e| e.to_lowercase())                       // Normalize
}
```

### Combining Multiple Options
```rust
fn create_user(name: Option<String>, email: Option<String>) -> Option<User> {
    match (name, email) {
        (Some(n), Some(e)) if !n.is_empty() && e.contains('@') => {
            Some(User { name: n, email: e })
        }
        _ => None,
    }
}
```

## 💡 Option vs Null: Why It's Revolutionary

**C# Null Problems:**
```csharp
// Runtime crash waiting to happen
string ProcessUser(User user) {
    return user.Name.ToUpper();  // 💥 NullReferenceException if user is null
}
```

**Rust Option Safety:**
```rust
// Compiler prevents runtime crashes
fn process_user(user: Option<User>) -> String {
    match user {
        Some(u) => u.name.to_uppercase(),
        None => "UNKNOWN".to_string(),
    }
    // Impossible to forget to handle the None case!
}
```

## 🔗 Integration with Other Rust Features

### With Result<T, E>
```rust
// Chain Option and Result operations
fn load_user_config(user_id: u32) -> Result<Config, Error> {
    find_user(user_id)                    // Returns Option<User>
        .ok_or(Error::UserNotFound)?      // Convert to Result
        .load_config()                    // Returns Result<Config, Error>
}
```

### With Iterators
```rust
// Powerful combination for data processing
let processed_data: Vec<ProcessedItem> = raw_data
    .iter()
    .filter_map(|item| validate(item).ok())     // Keep only valid items
    .map(|item| process(item))                  // Transform valid items
    .filter_map(|result| result.ok())           // Keep only successful processing
    .collect();
```

## 🏆 Mastery Checkpoint

You've mastered Option<T> when you can:
- ✅ Handle optional values without panicking
- ✅ Chain operations on optional data
- ✅ Convert between Option, Result, and plain values
- ✅ Process collections with optional/invalid elements
- ✅ Design APIs that make missing data explicit
- ✅ Write null-safe code that prevents runtime crashes

## ➡️ Next Steps

Ready for more advanced error handling? Continue with:
- **[Exercise 2](../ex02-result-chain.rs)** - Result<T, E> and error propagation
- **[Exercise 3](../ex03-error-types.rs)** - Custom error types
- **[Exercise 4](../ex04-conversions.rs)** - Advanced error patterns

You've now eliminated an entire class of runtime errors! 🦀