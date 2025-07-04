// Exercise 07: Publishing Problems - Prepare for crates.io!
//
// This exercise shows a crate that's not ready for publication.
// It's missing metadata, has naming issues, and lacks proper structure.
//
// Your mission: Make this publication-ready!

// Expected: A crate ready for crates.io with proper metadata and structure
// Currently: Missing required fields, poor naming, no documentation

//! A utility library for text processing.
//! 
//! This library provides various text processing functions.
//! But it's missing a lot of required information for publishing!

pub mod text_utils {
    /// Reverses a string.
    /// 
    /// But wait, this function name conflicts with std library naming!
    /// Also, it's not the most efficient implementation.
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    
    /// Counts words in a string.
    /// 
    /// This is a very naive implementation.
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }
    
    /// Converts to uppercase.
    /// 
    /// This just wraps the standard library function.
    /// Do we really need this in a published crate?
    pub fn to_upper(s: &str) -> String {
        s.to_uppercase()
    }
}

// The main function shouldn't be in a library crate!
fn main() {
    println!("This is a library, not a binary!");
    
    let text = "Hello, World!";
    println!("Original: {}", text);
    println!("Reversed: {}", text_utils::reverse(text));
    println!("Word count: {}", text_utils::word_count(text));
    println!("Uppercase: {}", text_utils::to_upper(text));
}

// Cargo.toml for this exercise (missing publication metadata):
/*
[package]
name = "ex07-publishing"  # Bad name - too generic and conflicts with exercise
version = "0.1.0"
edition = "2021"

# Missing required fields for publication:
# - authors or license
# - description
# - repository or homepage
# - readme
# - keywords
# - categories
# - license-file or license

[dependencies]
# Empty - but should we have dependencies for a text processing library?

# Also missing:
# - [lib] section configuration
# - exclude/include for packaging
# - metadata for docs.rs
# - badges for README

# Problems to fix:
# 1. Add proper metadata fields
# 2. Choose a better, unique name
# 3. Add meaningful description and keywords
# 4. Set up proper license
# 5. Remove main() function from lib.rs
# 6. Add comprehensive documentation
# 7. Add examples directory
# 8. Consider if the functionality is worth publishing
# 9. Add proper README.md
# 10. Set up CI/CD badges
*/