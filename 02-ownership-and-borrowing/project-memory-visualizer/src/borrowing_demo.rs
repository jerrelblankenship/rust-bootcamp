// Borrowing Demonstration - Fix the borrowing errors!
//
// Your task: Make all the borrowing examples compile and work correctly
// This demonstrates borrowing and reference concepts from Module 02

use crate::memory_tracker::MemoryTracker;
use colored::*;

pub fn run_demonstrations(tracker: &mut MemoryTracker) {
    println!("Running borrowing demonstrations...\n");
    
    println!("{}", "1. Immutable Borrowing".bold());
    demo_immutable_borrowing(tracker);
    
    println!("\n{}", "2. Mutable Borrowing".bold());
    demo_mutable_borrowing(tracker);
    
    println!("\n{}", "3. Borrowing Rules".bold());
    demo_borrowing_rules(tracker);
    
    println!("\n{}", "4. Multiple References".bold());
    demo_multiple_references(tracker);
    
    println!("\n{}", "5. Reference Scope and Lifetimes".bold());
    demo_reference_scope(tracker);
}

fn demo_immutable_borrowing(tracker: &mut MemoryTracker) {
    println!("Demonstrating immutable borrowing...");
    
    let data = String::from("borrowed data");
    tracker.track_allocation("data", "String", data.len());
    
    // TODO: Create an immutable reference to data
    let reference = /* TODO: borrow data */;
    tracker.track_borrow("reference", "data", false);
    
    // TODO: Use the reference to print the data
    println!("Original: {}", data);
    println!("Reference: {}", /* TODO: use reference */);
    
    // QUESTION: Can we modify data through the reference?
    // TODO: Try uncommenting this line and see what happens:
    // reference.push_str(" modified");  // Should this work?
    
    // TODO: Calculate the length using the reference
    let length = /* TODO: get length through reference */;
    println!("Length through reference: {}", length);
}

fn demo_mutable_borrowing(tracker: &mut MemoryTracker) {
    println!("Demonstrating mutable borrowing...");
    
    // TODO: Make data mutable
    let data = String::from("mutable data");  // FIXME: needs to be mutable
    tracker.track_allocation("data", "String", data.len());
    
    // TODO: Create a mutable reference
    let mut_ref = /* TODO: create mutable reference */;
    tracker.track_borrow("mut_ref", "data", true);
    
    // TODO: Modify data through the mutable reference
    // HINT: Use push_str() to add text
    // mut_ref./* TODO: modify the string */;
    
    println!("Modified data: {}", data);
    
    // CHALLENGE: Why can't we have both data and mut_ref in scope at the same time?
    // Try printing both simultaneously and see what happens
}

fn demo_borrowing_rules(tracker: &mut MemoryTracker) {
    println!("Demonstrating borrowing rules...");
    
    let mut data = String::from("rules demo");
    tracker.track_allocation("data", "String", data.len());
    
    // Rule 1: Multiple immutable references are OK
    let r1 = &data;
    let r2 = &data;
    tracker.track_borrow("r1", "data", false);
    tracker.track_borrow("r2", "data", false);
    
    println!("r1: {}, r2: {}", r1, r2);  // This works!
    
    // Rule 2: Only one mutable reference at a time
    let mut_ref = &mut data;
    tracker.track_borrow("mut_ref", "data", true);
    
    // TODO: Try uncommenting these lines to see the compile error:
    // let another_mut_ref = &mut data;  // FIXME: Second mutable borrow
    // println!("Two mutable refs: {}, {}", mut_ref, another_mut_ref);
    
    // Rule 3: Can't have immutable and mutable references simultaneously
    // TODO: Try uncommenting these lines:
    // let immut_ref = &data;  // FIXME: Immutable borrow while mutable exists
    // println!("Mixed refs: {}, {}", mut_ref, immut_ref);
    
    mut_ref.push_str(" - modified");
    println!("Final data: {}", data);
}

fn demo_multiple_references(tracker: &mut MemoryTracker) {
    println!("Demonstrating multiple reference patterns...");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    tracker.track_allocation("numbers", "Vec<i32>", numbers.len() * 4);
    
    // Multiple immutable references to different elements
    let first = &numbers[0];
    let last = &numbers[numbers.len() - 1];
    tracker.track_borrow("first", "numbers[0]", false);
    tracker.track_borrow("last", "numbers[4]", false);
    
    println!("First: {}, Last: {}", first, last);
    
    // TODO: Try to modify the vector while references exist
    // FIXME: This should cause a compile error
    // numbers.push(6);  // Can't modify while borrowed
    
    // References go out of scope here...
    
    // Now we can modify
    numbers.push(6);
    println!("Modified vector: {:?}", numbers);
    
    // TODO: Demonstrate slice borrowing
    let slice = /* TODO: borrow a slice of numbers */;
    println!("Slice: {:?}", slice);
}

fn demo_reference_scope(tracker: &mut MemoryTracker) {
    println!("Demonstrating reference scope and lifetimes...");
    
    let mut data = String::from("lifetime demo");
    tracker.track_allocation("data", "String", data.len());
    
    let reference;
    {
        let temp_data = String::from("temporary");
        tracker.track_allocation("temp_data", "String", temp_data.len());
        
        // TODO: Try uncommenting this line - what happens?
        // reference = &temp_data;  // FIXME: temp_data doesn't live long enough
        
        // Instead, reference the outer data
        reference = &data;
        tracker.track_borrow("reference", "data", false);
        
        println!("Inside scope: {}", reference);
    }
    // temp_data is dropped here
    tracker.track_deallocation("temp_data", 9);  // "temporary".len()
    
    // TODO: Can we still use reference here?
    println!("Outside scope: {}", reference);  // Should this work?
    
    // CHALLENGE: What determines how long a reference can live?
}

// TODO: Fix this function that tries to return a reference
fn get_string_reference() -> &str {  // FIXME: Missing lifetime parameter
    let s = String::from("local string");
    &s  // FIXME: Returning reference to local variable
}

// TODO: Fix this function to work correctly
fn get_string_correctly() -> String {
    // HINT: Return the owned value, not a reference
    let s = String::from("owned string");
    // TODO: Return s in a way that transfers ownership
}

// TODO: This function should borrow rather than take ownership
fn calculate_length(s: String) -> usize {  // FIXME: Takes ownership
    s.len()
}  // s is dropped here - wasteful!

// TODO: Fix the function to borrow instead
fn calculate_length_correctly(/* TODO: parameter */) -> usize {
    // TODO: Implementation
}

pub fn interactive_demo(tracker: &mut MemoryTracker) {
    use std::io::{self, Write};
    
    println!("Interactive Borrowing Demo");
    println!("Commands:");
    println!("  create <name>     - Create a new String");
    println!("  borrow <name>     - Create immutable reference");
    println!("  borrow_mut <name> - Create mutable reference");
    println!("  modify <ref>      - Modify through mutable reference");
    println!("  print <name>      - Print value or reference");
    println!("  back              - Return to main menu");
    
    let mut values: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    loop {
        print!("borrowing> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        match parts.as_slice() {
            ["create", name] => {
                let value = format!("string_{}", name);
                tracker.track_allocation(name, "String", value.len());
                values.insert(name.to_string(), value);
                println!("✅ Created {}", name);
            }
            ["borrow", name] => {
                if values.contains_key(*name) {
                    tracker.track_borrow(&format!("&{}", name), name, false);
                    println!("✅ Created immutable reference to {}", name);
                } else {
                    println!("❌ {} not found", name);
                }
            }
            ["borrow_mut", name] => {
                if values.contains_key(*name) {
                    tracker.track_borrow(&format!("&mut {}", name), name, true);
                    println!("✅ Created mutable reference to {}", name);
                } else {
                    println!("❌ {} not found", name);
                }
            }
            ["print", name] => {
                if let Some(value) = values.get(*name) {
                    println!("{}: \"{}\"", name, value);
                } else {
                    println!("❌ {} not found", name);
                }
            }
            ["back"] => break,
            _ => {
                println!("Unknown command");
            }
        }
    }
}

// CHALLENGE EXERCISES:

// TODO: Fix this function that has borrowing issues
fn process_strings(strings: Vec<String>) -> Vec<usize> {
    let mut lengths = Vec::new();
    
    for s in strings {  // FIXME: This moves each string
        lengths.push(s.len());
        // Can't use s again after this
    }
    
    // TODO: How can we access the original strings after processing?
    // HINT: Consider borrowing instead of moving
    
    lengths
}

// TODO: Implement a function that sorts strings in place
fn sort_strings_in_place(strings: /* TODO: parameter type */) {
    // TODO: Sort the strings vector in place
    // HINT: You need mutable access
}

// TODO: Implement a function that finds the longest string
fn find_longest_string<'a>(strings: /* TODO: parameter */) -> Option</* TODO: return type */> {
    // TODO: Return a reference to the longest string
    // HINT: You'll need lifetime annotations
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_immutable_borrowing() {
        let data = String::from("test");
        let reference = &data;
        
        // Both should work
        assert_eq!(data, "test");
        assert_eq!(*reference, "test");
    }
    
    #[test]
    fn test_calculate_length_correctly() {
        let s = String::from("test string");
        let len = calculate_length_correctly(&s);
        
        // s should still be usable after the function call
        assert_eq!(len, 11);
        assert_eq!(s, "test string");
    }
    
    // TODO: Add more tests for borrowing scenarios
    #[test]
    fn test_multiple_immutable_borrows() {
        // TODO: Test that multiple immutable borrows work
    }
    
    #[test]
    fn test_mutable_borrow_exclusivity() {
        // TODO: Test that mutable borrows are exclusive
        // Note: Can't test compile-time errors, but can test behavior
    }
}
