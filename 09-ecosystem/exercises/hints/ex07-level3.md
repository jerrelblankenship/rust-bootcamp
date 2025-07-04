# Exercise 07 - Level 3 Hints: Publishing Problems

## 🎯 Complete Publication Setup

### 🔧 Cargo.toml
```toml
[package]
name = "student-text-utils"
version = "0.1.0"
edition = "2021"
authors = ["Student <student@example.com>"]
description = "A collection of text processing utilities for learning Rust"
license = "MIT OR Apache-2.0"
repository = "https://github.com/student/student-text-utils"
homepage = "https://github.com/student/student-text-utils"
keywords = ["text", "string", "processing", "utilities", "learning"]
categories = ["text-processing"]
readme = "README.md"

[lib]
name = "student_text_utils"
path = "src/lib.rs"

[[example]]
name = "usage"
path = "examples/usage.rs"
```

### 📝 lib.rs
```rust
//! A collection of text processing utilities.
//!
//! This crate provides simple, efficient text processing functions
//! for common operations like reversing strings, counting words,
//! and case conversion.
//!
//! # Examples
//!
//! ```
//! use student_text_utils::reverse;
//!
//! let reversed = reverse("Hello, World!");
//! assert_eq!(reversed, "!dlroW ,olleH");
//! ```

/// Reverses a string efficiently.
/// 
/// # Examples
/// 
/// ```
/// use student_text_utils::reverse;
/// 
/// let result = reverse("hello");
/// assert_eq!(result, "olleh");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Counts words in a string, handling multiple whitespace.
/// 
/// # Examples
/// 
/// ```
/// use student_text_utils::word_count;
/// 
/// assert_eq!(word_count("hello world"), 2);
/// assert_eq!(word_count("  hello   world  "), 2);
/// ```
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Converts text to title case.
/// 
/// # Examples
/// 
/// ```
/// use student_text_utils::to_title_case;
/// 
/// assert_eq!(to_title_case("hello world"), "Hello World");
/// ```
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("a"), "a");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
        assert_eq!(word_count("one"), 1);
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
        assert_eq!(to_title_case(""), "");
    }
}
```

### 📚 examples/usage.rs
```rust
use student_text_utils::{reverse, word_count, to_title_case};

fn main() {
    let text = "Hello, Rust World!";
    
    println!("Original: {}", text);
    println!("Reversed: {}", reverse(text));
    println!("Word count: {}", word_count(text));
    println!("Title case: {}", to_title_case(&text.to_lowercase()));
}
```

## 🎯 What Changed

1. **Proper library structure** - no main function
2. **Complete metadata** for publication
3. **Comprehensive documentation** with examples
4. **Better functionality** - more useful than simple wrappers
5. **Tests included** for reliability

## 🏆 Success Criteria

`cargo publish --dry-run` should succeed with no errors!