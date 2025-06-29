# Exercise 2 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working implementation. Here's the full solution with detailed explanations.

## 📝 Complete ex02-borrowing.rs Implementation

```rust
// Exercise 2: Borrowing and References - Complete Solutions
//
// This file demonstrates all borrowing concepts from Module 02

use std::io::{self, Write};

fn main() {
    println!("=== Exercise 2: Borrowing and References (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_2_1();
    exercise_2_2();
    exercise_2_3();
    exercise_2_4();
    exercise_2_5();
    exercise_2_6();
    exercise_2_7();
    exercise_2_8();
    
    println!("\n🎉 All borrowing exercises completed!");
}

// Exercise 2.1: Basic immutable borrowing - SOLVED
fn exercise_2_1() {
    println!("Exercise 2.1: Basic immutable borrowing");
    
    let data = String::from("borrowed data");
    
    // SOLUTION: Use & to create immutable reference instead of moving
    let reference = &data;  // Borrow, don't move ownership
    
    // Both work now because data still owns the string
    println!("Original: {}", data);
    println!("Reference: {}", reference);
    
    // Can access methods through the reference
    let length = reference.len();
    println!("Length through reference: {}", length);
    
    // ANSWER: Moving transfers ownership and makes original invalid.
    // Borrowing creates a reference while keeping original valid.
    
    println!("✅ Exercise 2.1 complete\n");
}

// Exercise 2.2: Mutable borrowing - SOLVED
fn exercise_2_2() {
    println!("Exercise 2.2: Mutable borrowing");
    
    // SOLUTION: Make the binding mutable
    let mut data = String::from("mutable data");
    
    // SOLUTION: Create mutable reference with &mut
    let mut_ref = &mut data;
    
    // Now we can modify through the mutable reference
    mut_ref.push_str(" - modified");
    
    println!("Modified data: {}", data);
    
    // ANSWERS:
    // 1. `let mut data` makes the variable binding mutable
    // 2. `&mut data` creates a mutable reference to the mutable data
    
    println!("✅ Exercise 2.2 complete\n");
}

// Exercise 2.3: Borrowing rules - multiple references - SOLVED
fn exercise_2_3() {
    println!("Exercise 2.3: Borrowing rules");
    
    let mut data = String::from("rules demo");
    
    // Multiple immutable references are allowed
    let r1 = &data;
    let r2 = &data;
    println!("Multiple immutable refs: {} and {}", r1, r2);
    
    // SOLUTION: Immutable references end after their last use above
    // Now we can create mutable references
    
    // Single mutable reference is allowed when no immutable refs exist
    let mut_ref1 = &mut data;
    mut_ref1.push_str(" - modified");
    println!("Mutable ref1: {}", mut_ref1);
    
    // SOLUTION: Use scopes to control reference lifetimes
    {
        let mut_ref2 = &mut data;
        mut_ref2.push_str(" - again");
        println!("Mutable ref2: {}", mut_ref2);
    } // mut_ref2 scope ends here
    
    // Now we can use data normally again
    println!("Final data: {}", data);
    
    println!("✅ Exercise 2.3 complete\n");
}

// Exercise 2.4: Reference scope and lifetime - SOLVED
fn exercise_2_4() {
    println!("Exercise 2.4: Reference scope and lifetime");
    
    // SOLUTION: Move data to outer scope so reference doesn't outlive it
    let temp_data = String::from("temporary");
    let reference = &temp_data;  // Reference and data in same scope
    
    println!("Same scope: {}", reference);
    println!("Still same scope: {}", reference);
    
    // Alternative: Use reference immediately without storing
    {
        let inner_data = String::from("inner scope");
        println!("Immediate use: {}", &inner_data);  // Use without storing
        // This works because we don't try to use the reference later
    }
    
    // Demonstrate lifetime issue and solution
    let long_lived_ref;
    {
        let short_lived_data = String::from("short");
        // long_lived_ref = &short_lived_data;  // This would fail
        
        // SOLUTION: Clone the data to extend its lifetime
        long_lived_ref = short_lived_data.clone();  // Now we own the data
    }
    println!("Long-lived data: {}", long_lived_ref);
    
    println!("✅ Exercise 2.4 complete\n");
}

// Exercise 2.5: Functions with borrowing - SOLVED
fn exercise_2_5() {
    println!("Exercise 2.5: Functions with borrowing");
    
    let mut text = String::from("hello");
    
    // SOLUTION: Use corrected function that borrows instead of taking ownership
    let length = calculate_length_correctly(&text);
    println!("Length: {}", length);
    println!("Original text: {}", text);  // Works because we borrowed, not moved
    
    // Demonstrate the difference
    let text2 = String::from("world");
    let len2 = calculate_length_badly(text2.clone());  // Clone to avoid move
    println!("Length of clone: {}", len2);
    println!("Original text2: {}", text2);  // Still works because we cloned
    
    println!("✅ Exercise 2.5 complete\n");
}

// Original function (for comparison) - takes ownership
fn calculate_length_badly(s: String) -> usize {
    s.len()
}  // s is dropped here - wasteful!

// SOLUTION: Corrected function that borrows
fn calculate_length_correctly(s: &String) -> usize {
    s.len()  // Can access length through reference
}  // Reference goes out of scope, but doesn't own the data

// Exercise 2.6: Modifying through mutable references - SOLVED
fn exercise_2_6() {
    println!("Exercise 2.6: Modifying through mutable references");
    
    let mut message = String::from("Hello");
    
    // SOLUTION: Use function that takes mutable reference
    append_exclamation_correctly(&mut message);
    println!("After function: {}", message);  // Works - we still own message
    
    // Demonstrate multiple modifications
    append_text_correctly(&mut message, " World");
    append_text_correctly(&mut message, "!");
    println!("After multiple modifications: {}", message);
    
    println!("✅ Exercise 2.6 complete\n");
}

// Original function (for comparison) - takes ownership
fn append_exclamation_badly(s: String) {
    println!("Cannot modify: {}", s);
    // Can't modify because s is moved in, not borrowed
}

// SOLUTION: Function that takes mutable reference
fn append_exclamation_correctly(s: &mut String) {
    s.push_str("!");  // Modify the original string through reference
}

// Additional helper function
fn append_text_correctly(s: &mut String, text: &str) {
    s.push_str(text);
}

// Exercise 2.7: Slices and borrowing - SOLVED
fn exercise_2_7() {
    println!("Exercise 2.7: Slices and borrowing");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // SOLUTION: Create slice instead of moving entire vector
    let first_three = &numbers[0..3];  // Slice borrows part of the vector
    
    println!("First three: {:?}", first_three);
    println!("Original numbers: {:?}", numbers);  // Still works!
    
    // Demonstrate different slice patterns
    let first_two = &numbers[0..2];      // First 2 elements
    let last_two = &numbers[3..5];       // Last 2 elements
    let middle = &numbers[2..3];         // Just the middle element
    let all_slice = &numbers[..];        // Entire vector as slice
    
    println!("First 2: {:?}", first_two);
    println!("Last 2: {:?}", last_two);
    println!("Middle: {:?}", middle);
    println!("All as slice: {:?}", all_slice);
    
    // Demonstrate slice methods
    println!("Slice length: {}", first_three.len());
    println!("Slice contains 2: {}", first_three.contains(&2));
    
    // Slices work with ranges
    for (i, &value) in first_three.iter().enumerate() {
        println!("Index {}: {}", i, value);
    }
    
    println!("✅ Exercise 2.7 complete\n");
}

// Exercise 2.8: String slices - SOLVED
fn exercise_2_8() {
    println!("Exercise 2.8: String slices");
    
    let sentence = String::from("The quick brown fox");
    
    // SOLUTION: Use functions that return string slices
    let first_word = get_first_word_correctly(&sentence);
    println!("First word: {}", first_word);
    
    let second_word = get_second_word_correctly(&sentence);
    println!("Second word: {}", second_word);
    
    // Original sentence still usable because we borrowed
    println!("Original sentence: {}", sentence);
    
    // Demonstrate more string slice operations
    let words = get_all_words_correctly(&sentence);
    println!("All words: {:?}", words);
    
    let substring = get_substring_correctly(&sentence, 4, 9);
    println!("Substring (4..9): '{}'", substring);
    
    // String slices work with string literals too
    let literal_word = get_first_word_correctly("Hello world");
    println!("First word from literal: {}", literal_word);
    
    println!("✅ Exercise 2.8 complete\n");
}

// Original functions (for comparison) - take ownership
fn get_first_word_badly(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0].to_string()  // Inefficient: allocates new String
}

fn get_second_word_badly(s: String) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[1].to_string()  // Inefficient: allocates new String
}

// SOLUTION: Efficient functions using string slices
fn get_first_word_correctly(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[0]  // Return slice - no allocation needed
}

fn get_second_word_correctly(s: &str) -> &str {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[1]  // Return slice - no allocation needed
}

// Additional utility functions
fn get_all_words_correctly(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()  // Returns vector of string slices
}

fn get_substring_correctly(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]  // Return slice of the string
}

// BONUS IMPLEMENTATIONS

// Advanced borrowing pattern: processing with temporary references
fn process_data_advanced() {
    println!("=== Advanced Borrowing Patterns ===");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // Pattern 1: Sequential borrows
    {
        let read_only = &data;
        println!("Reading data: {:?}", read_only);
        println!("Data length: {}", read_only.len());
    } // read_only borrow ends here
    
    {
        let mutable = &mut data;
        mutable.push(6);
        mutable.sort();
        println!("After modification: {:?}", mutable);
    } // mutable borrow ends here
    
    // Pattern 2: Slice processing
    let (first_half, second_half) = data.split_at(data.len() / 2);
    println!("First half: {:?}", first_half);
    println!("Second half: {:?}", second_half);
    
    // Pattern 3: Iterator borrowing
    for item in &data {  // Borrows each element
        println!("Item: {}", item);
    }
    
    println!("Original data still accessible: {:?}", data);
}

// Challenge 1: Fixed function with multiple borrowing issues
fn process_data_fixed(data: &mut Vec<i32>) {
    // SOLUTION: Use separate scopes for different borrows
    {
        let first = &data[0];        // Immutable borrow
        println!("First: {}", first);
    } // Immutable borrow ends here
    
    data.push(6);                    // Now mutable borrow is OK
    println!("After push: {:?}", data);
}

// Challenge 2: Function with explicit lifetime annotation
fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Challenge 3: Struct with lifetime parameter
struct Container<'a> {
    data: &'a str,
}

impl<'a> Container<'a> {
    fn new(data: &'a str) -> Self {
        Container { data }
    }
    
    fn get_data(&self) -> &str {
        self.data
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
}

// Demonstrate usage of lifetime-parameterized struct
fn demonstrate_container() {
    let text = String::from("Hello, lifetime!");
    let container = Container::new(&text);
    
    println!("Container data: {}", container.get_data());
    println!("Container data length: {}", container.len());
    
    // container and text must have compatible lifetimes
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_borrowing_patterns() {
        let mut data = String::from("test");
        
        // Test immutable borrowing
        let len = calculate_length_correctly(&data);
        assert_eq!(len, 4);
        assert_eq!(data, "test");  // data still usable
        
        // Test mutable borrowing
        append_text_correctly(&mut data, "ing");
        assert_eq!(data, "testing");
    }
    
    #[test]
    fn test_string_slices() {
        let sentence = String::from("hello world rust");
        
        let first = get_first_word_correctly(&sentence);
        let second = get_second_word_correctly(&sentence);
        
        assert_eq!(first, "hello");
        assert_eq!(second, "world");
        assert_eq!(sentence, "hello world rust");  // Original unchanged
    }
    
    #[test]
    fn test_vector_slices() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        let first_three = &numbers[0..3];
        let last_two = &numbers[3..5];
        
        assert_eq!(first_three, &[1, 2, 3]);
        assert_eq!(last_two, &[4, 5]);
        assert_eq!(numbers.len(), 5);  // Original unchanged
    }
    
    #[test]
    fn test_lifetime_functions() {
        let s1 = "short";
        let s2 = "longer string";
        let result = find_longest(s1, s2);
        assert_eq!(result, "longer string");
    }
    
    #[test]
    fn test_container_lifetime() {
        let text = String::from("container test");
        let container = Container::new(&text);
        assert_eq!(container.get_data(), "container test");
        assert_eq!(container.len(), 14);
    }
    
    #[test]
    fn test_advanced_borrowing() {
        let mut data = vec![1, 2, 3];
        
        // Test that our fixed function works
        process_data_fixed(&mut data);
        assert_eq!(data, vec![1, 2, 3, 6]);
        
        // Test split borrowing
        let (left, right) = data.split_at_mut(2);
        left[0] = 10;
        right[0] = 30;
        assert_eq!(data, vec![10, 2, 30, 6]);
    }
}
```

## 🎓 Complete Code Walkthrough

### 1. Reference Creation Patterns
```rust
// Immutable reference (shared access)
let data = String::from("hello");
let reference = &data;  // Multiple allowed

// Mutable reference (exclusive access)
let mut data = String::from("hello");
let mut_ref = &mut data;  // Only one allowed

// Automatic dereferencing
println!("{}", reference);  // Rust automatically dereferences
println!("{}", *reference); // Explicit dereferencing (same result)
```

### 2. Borrowing Rules in Practice
```rust
fn demonstrate_borrowing_rules() {
    let mut data = vec![1, 2, 3];
    
    // ✅ Multiple immutable borrows OK
    let r1 = &data;
    let r2 = &data;
    println!("{:?} {:?}", r1, r2);
    
    // ✅ Single mutable borrow OK (after immutable borrows end)
    let r3 = &mut data;
    r3.push(4);
    
    // ❌ This would fail: can't have immutable and mutable together
    // let r4 = &data;  // Error if r3 still active
}
```

### 3. Function Parameter Patterns
```rust
// Read-only access (prefer &str for strings)
fn read_text(text: &str) -> usize { text.len() }

// Read-only access to complex types
fn read_vec(vec: &Vec<i32>) -> usize { vec.len() }

// Modify without taking ownership
fn modify_text(text: &mut String) { text.push('!'); }

// Return references with same lifetime as input
fn get_first_char(text: &str) -> Option<&str> {
    text.get(0..1)  // Returns slice of input
}
```

### 4. Slice Patterns
```rust
fn demonstrate_slices() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Vector slices
    let first_half = &data[0..2];     // [1, 2]
    let second_half = &data[3..];     // [4, 5]
    let middle = &data[1..4];         // [2, 3, 4]
    
    let text = String::from("Hello, World!");
    
    // String slices
    let hello = &text[0..5];          // "Hello"
    let world = &text[7..12];         // "World"
    let all = &text[..];              // Entire string
}
```

### 5. Lifetime Management
```rust
fn lifetime_examples() {
    // ✅ References live within data lifetime
    let data = String::from("valid");
    let reference = &data;
    println!("{}", reference);  // Works - same scope
    
    // ❌ This pattern would fail:
    // let reference;
    // {
    //     let data = String::from("invalid");
    //     reference = &data;  // data dropped at end of scope
    // }
    // println!("{}", reference);  // Error - data no longer exists
    
    // ✅ Solution: move data to outer scope
    let reference;
    let data = String::from("valid");
    reference = &data;
    println!("{}", reference);  // Works - data outlives reference
}
```

## 🏆 Advanced Borrowing Patterns

### Interior Mutability with RefCell
```rust
use std::cell::RefCell;

fn interior_mutability() {
    let data = RefCell::new(vec![1, 2, 3]);
    
    // Runtime borrowing rules
    {
        let borrow1 = data.borrow();     // Immutable borrow
        let borrow2 = data.borrow();     // Multiple immutable OK
        println!("{:?} {:?}", borrow1, borrow2);
    }  // Borrows end here
    
    {
        let mut mut_borrow = data.borrow_mut();  // Mutable borrow
        mut_borrow.push(4);
        // Only one mutable borrow allowed
    }  // Mutable borrow ends here
}
```

### Split Borrowing
```rust
fn split_borrowing() {
    let mut data = [1, 2, 3, 4, 5];
    
    // Borrow different parts simultaneously
    let (left, right) = data.split_at_mut(2);
    left[0] = 10;   // Modify left part
    right[0] = 30;  // Modify right part simultaneously
    
    // This is safe because we're modifying different memory locations
}
```

### Iterator Borrowing
```rust
fn iterator_patterns() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Immutable iteration (borrows each element)
    for item in &data {
        println!("Item: {}", item);
    }
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // Mutable iteration (mutably borrows each element)
    for item in &mut data {
        *item *= 2;
    }
    
    // data is still accessible after iteration
    println!("Modified: {:?}", data);
}
```

## 🎯 Key Learning Achievements

By completing this exercise, you've mastered:

### ✅ **Borrowing Fundamentals**
- Difference between `&T` and `&mut T` references
- When multiple references are allowed vs prohibited
- How reference lifetimes work and why they matter

### ✅ **Function Design Patterns**
- When to take references vs ownership in parameters
- How to return references safely
- Using string slices (`&str`) for maximum flexibility

### ✅ **Memory Safety**
- How borrowing prevents data races at compile time
- Why you can't have aliasing and mutation together
- How Rust ensures references don't outlive their data

### ✅ **Slice Operations**
- Creating slices of vectors and strings
- Working with parts of data without copying
- Iterator patterns that borrow vs consume

### ✅ **C# to Rust Translation**
You can now translate C# reference patterns:
- Object references → Rust borrowing with `&` and `&mut`
- Method parameters → Choose between ownership and borrowing
- Collection access → Use slices for efficient partial access
- Data sharing → Understand when to clone vs borrow

## 🚀 Next Steps

**Congratulations!** You've mastered Rust's borrowing system. You're ready for:

1. **Module 02 Exercise 3**: Lifetimes and Reference Validity
2. **Module 02 Exercise 4**: Smart Pointers (Rc, Arc, Box)
3. **Module 02 Exercise 5**: Advanced Ownership Patterns

The borrowing concepts you've learned are fundamental to all Rust programming! 🦀