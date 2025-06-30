# Exercise 1 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Option<T> Patterns

You've tried Level 1 hints but need concrete implementation guidance. Here are specific solutions for each Option exercise challenge.

## 🔧 Exercise 1.1: Basic Option Handling

**Problem**: `get_user_name` needs to return a String, but `find_user` returns `Option<User>`.

**Specific Solution:**
```rust
fn get_user_name(user_id: u32) -> String {
    let user = find_user(user_id);
    match user {
        Some(u) => u.name,      // Extract name from found user
        None => "Unknown".to_string(),  // Provide default for missing user
    }
}

// Alternative using unwrap_or_else:
fn get_user_name(user_id: u32) -> String {
    find_user(user_id)
        .map(|u| u.name)                    // Extract name if Some
        .unwrap_or_else(|| "Unknown".to_string())  // Default if None
}
```

**Key Learning**: `match` on Option gives you access to the inner value in the `Some` branch.

## 🔧 Exercise 1.2: Option Chaining

**Problem**: Chain operations user → email → domain, handling None at each step.

**Specific Solution:**
```rust
fn get_user_email_domain(user_id: u32) -> Option<String> {
    find_user(user_id)
        .map(|user| user.email)              // Option<User> → Option<String>
        .and_then(|email| {                 // Chain another Option operation
            email.split_once('@')           // Returns Option<(&str, &str)>
                .map(|(_, domain)| domain.to_string())  // Extract domain part
        })
}

// Alternative with explicit matching:
fn get_user_email_domain(user_id: u32) -> Option<String> {
    let user = find_user(user_id)?;         // Early return if None
    let email = &user.email;
    let domain = email.split_once('@')?.1;  // Early return if no '@'
    Some(domain.to_string())
}
```

**Key Learning**: 
- `.map()` transforms `Some(value)` → `Some(new_value)`, leaves `None` unchanged
- `.and_then()` chains operations that return `Option`
- `?` operator provides early return for None

## 🔧 Exercise 1.3: Collections and Option

**Problem**: Calculate average of valid scores, ignoring None values.

**Specific Solution:**
```rust
fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
    // Extract all valid scores
    let valid_scores: Vec<i32> = scores
        .iter()
        .filter_map(|score| *score)  // Keep only Some values, extract i32
        .collect();
    
    if valid_scores.is_empty() {
        None  // No valid scores to average
    } else {
        let sum: i32 = valid_scores.iter().sum();
        let average = sum as f64 / valid_scores.len() as f64;
        Some(average)
    }
}

// More concise version:
fn calculate_average(scores: &[Option<i32>]) -> Option<f64> {
    let valid_scores: Vec<i32> = scores.iter().filter_map(|&s| s).collect();
    
    match valid_scores.len() {
        0 => None,
        len => Some(valid_scores.iter().sum::<i32>() as f64 / len as f64),
    }
}
```

**Key Learning**: `.filter_map()` combines filtering and transformation, perfect for working with Option collections.

## 🔧 Exercise 1.4: Option with Error Handling

**Problem**: Convert Result to Option and handle division by zero.

**Specific Solutions:**
```rust
fn safe_parse_int(s: &str) -> Option<i32> {
    s.parse().ok()  // Convert Result<i32, ParseIntError> to Option<i32>
}

fn safe_divide(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None  // Division by zero
    } else {
        Some(a as f64 / b as f64)
    }
}

fn parse_and_divide(a_str: &str, b_str: &str) -> Option<f64> {
    let a = safe_parse_int(a_str)?;  // Early return if parse fails
    let b = safe_parse_int(b_str)?;  // Early return if parse fails
    safe_divide(a, b)                // Returns Option<f64>
}

// Alternative using and_then:
fn parse_and_divide(a_str: &str, b_str: &str) -> Option<f64> {
    safe_parse_int(a_str).and_then(|a| {
        safe_parse_int(b_str).and_then(|b| {
            safe_divide(a, b)
        })
    })
}
```

**Key Learning**: `.ok()` converts `Result<T, E>` to `Option<T>`, discarding error details.

## 🔧 Exercise 1.5: Option in Structs

**Problem**: Handle optional struct fields gracefully.

**Specific Solutions:**
```rust
impl Person {
    fn preferred_contact(&self) -> Option<&str> {
        self.email
            .as_deref()                    // Convert Option<String> to Option<&str>
            .or_else(|| self.phone.as_deref())  // Fallback to phone if no email
    }
    
    fn contact_info(&self) -> String {
        match (&self.email, &self.phone) {
            (Some(email), _) => format!("{} <{}>", self.name, email),
            (None, Some(phone)) => format!("{} ({})", self.name, phone),
            (None, None) => self.name.clone(),
        }
    }
}
```

**Key Learning**: 
- `.as_deref()` converts `Option<String>` to `Option<&str>`
- `.or_else()` provides fallback Option computation
- Pattern match on tuples of Options for complex logic

## 🔧 Exercise 1.6: Option ↔ Result Conversions

**Problem**: Convert between Option and Result types.

**Specific Solutions:**
```rust
fn option_to_result<T>(opt: Option<T>, error_msg: &str) -> Result<T, String> {
    opt.ok_or_else(|| error_msg.to_string())
}

fn result_to_option<T, E>(res: Result<T, E>) -> Option<T> {
    res.ok()  // Discard error, keep only success value
}
```

**Key Learning**: 
- `.ok_or()` and `.ok_or_else()` convert Option to Result
- `.ok()` converts Result to Option, discarding error information

## 🔧 Exercise 1.7: Option with Iterators

**Problem**: Process collections while handling optional/invalid values.

**Specific Solutions:**
```rust
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
```

**Key Learning**: `.filter_map()` is perfect for transforming and filtering in one operation.

## 💡 Option Method Cheat Sheet

### Extracting Values
- `match option { Some(val) => ..., None => ... }` - Pattern match
- `.unwrap()` - Extract value, panic if None ⚠️ 
- `.unwrap_or(default)` - Extract value or use default
- `.unwrap_or_else(|| compute_default())` - Extract value or compute default

### Transforming Values
- `.map(|val| transform(val))` - Transform Some value, leave None unchanged
- `.and_then(|val| another_option(val))` - Chain Option-returning operations
- `.or_else(|| another_option())` - Provide fallback Option

### Converting Types
- `.ok_or(error)` - Convert to Result
- `result.ok()` - Convert Result to Option
- `.as_ref()` - Convert `Option<T>` to `Option<&T>`
- `.as_deref()` - Convert `Option<String>` to `Option<&str>`

### Working with Collections
- `.filter_map(|item| process(item))` - Filter and transform
- `option?` - Early return if None (in functions returning Option)

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex01-level3.md) for full solution code.

## 🎓 Understanding Check

You should now understand:
1. **Basic Option handling**: match vs unwrap_or patterns
2. **Option chaining**: map() and and_then() for transformations  
3. **Collection processing**: filter_map() for working with optional values
4. **Type conversions**: Between Option, Result, and references
5. **Real-world patterns**: Handling optional struct fields and user input

Ready to master Rust's null-safety system! 🦀