// Ownership Demonstration - Fix the broken code!
//
// Your task: Make all the functions compile and work correctly
// This demonstrates the ownership concepts from Module 02

use crate::memory_tracker::MemoryTracker;
use colored::*;

pub fn run_demonstrations(tracker: &mut MemoryTracker) {
    println!("Running ownership demonstrations...\n");
    
    println!("{}", "1. Basic Ownership Transfer".bold());
    demo_basic_ownership(tracker);
    
    println!("\n{}", "2. Function Ownership".bold());
    demo_function_ownership(tracker);
    
    println!("\n{}", "3. Ownership with Cloning".bold());
    demo_cloning(tracker);
    
    println!("\n{}", "4. Copy vs Move Semantics".bold());
    demo_copy_vs_move(tracker);
    
    println!("\n{}", "5. RAII and Drop".bold());
    demo_raii_and_drop(tracker);
}

pub fn show_basic_example() {
    println!("{}", "Basic ownership example:".blue());
    println!("  let s1 = String::from(\"hello\");");
    println!("  let s2 = s1;  // s1 is moved to s2");
    println!("  // s1 is no longer valid!");
    println!("  println!(\"{{}}\", s2);  // Only s2 works");
}

fn demo_basic_ownership(tracker: &mut MemoryTracker) {
    // FIXME: This code doesn't compile - make it work!
    let s1 = String::from("hello");
    tracker.track_allocation("s1", "String", s1.len());
    
    let s2 = s1;  // PROBLEM: s1 is moved here
    tracker.track_move("s1", "s2", "String");
    
    // TODO: Fix these print statements
    // HINT: Only one of these variables is valid after the move
    println!("s1: {}", s1);  // FIXME: This won't compile
    println!("s2: {}", s2);  // This should work
    
    // CHALLENGE: Make both prints work
    // HINT: What method creates an independent copy?
}

fn demo_function_ownership(tracker: &mut MemoryTracker) {
    println!("Demonstrating function ownership transfer...");
    
    let data = String::from("function data");
    tracker.track_allocation("data", "String", data.len());
    
    // FIXME: After calling this function, `data` is no longer valid
    takes_ownership(data);
    
    // TODO: Fix this print statement
    println!("Data after function call: {}", data);  // FIXME: Won't compile
    
    // CHALLENGE: Modify the code so `data` is still accessible after the function call
    // HINT: Look at the function signature below
}

// TODO: Modify this function to not take ownership
// HINT: What if it borrowed the data instead?
fn takes_ownership(s: String) {
    println!("Function received: {}", s);
    // s is dropped here when function ends
}

// TODO: Create a function that borrows instead of taking ownership
fn borrows_data(/* TODO: Add parameter */) {
    // TODO: Implement this function
    println!("Function borrowed: {}", /* TODO: use parameter */);
    // Nothing is dropped - we're just borrowing!
}

fn demo_cloning(tracker: &mut MemoryTracker) {
    println!("Demonstrating cloning vs moving...");
    
    let original = String::from("original data");
    tracker.track_allocation("original", "String", original.len());
    
    // TODO: Create a clone instead of moving
    let copy = original;  // FIXME: This moves, but we want a clone
    tracker.track_move("original", "copy", "String");  // TODO: Update this for cloning
    
    // TODO: Make both of these work
    println!("Original: {}", original);  // FIXME: Won't compile after move
    println!("Copy: {}", copy);
    
    // QUESTION: What's the difference between clone() and move?
    // ANSWER: [Write your answer here]
}

fn demo_copy_vs_move(tracker: &mut MemoryTracker) {
    println!("Copy vs Move semantics...");
    
    // This works - integers implement Copy
    let x = 42;
    let y = x;
    println!("x: {}, y: {}", x, y);  // Both work!
    
    // TODO: Explain why this works but String moves don't
    // HINT: Check what traits these types implement
    
    // This doesn't work - Strings don't implement Copy
    let s1 = String::from("hello");
    tracker.track_allocation("s1", "String", s1.len());
    
    let s2 = s1;  // Move occurs
    tracker.track_move("s1", "s2", "String");
    
    // TODO: Fix the compilation error
    println!("s1: {}, s2: {}", s1, s2);  // FIXME: s1 moved
    
    // CHALLENGE: What types implement Copy? Make a list.
    // Copy types: i32, f64, bool, char, [TODO: add more]
}

fn demo_raii_and_drop(tracker: &mut MemoryTracker) {
    println!("RAII (Resource Acquisition Is Initialization) demo...");
    
    {
        let scoped_data = String::from("scoped");
        tracker.track_allocation("scoped_data", "String", scoped_data.len());
        
        println!("Inside scope: {}", scoped_data);
        
        // TODO: What happens when this scope ends?
        // HINT: The Drop trait is automatically called
    }
    // scoped_data is automatically dropped here
    tracker.track_deallocation("scoped_data", 6);  // "scoped".len()
    
    println!("After scope - scoped_data is gone!");
    
    // CHALLENGE: Create a custom struct that prints when it's dropped
    // HINT: Implement the Drop trait
}

// TODO: Implement this struct with custom Drop behavior
struct DroppableResource {
    name: String,
}

impl DroppableResource {
    fn new(name: &str) -> Self {
        println!("🔧 Creating resource: {}", name);
        DroppableResource {
            name: name.to_string(),
        }
    }
}

// TODO: Implement Drop trait for DroppableResource
// impl Drop for DroppableResource {
//     fn drop(&mut self) {
//         println!("🗑️ Dropping resource: {}", self.name);
//     }
// }

pub fn interactive_demo(tracker: &mut MemoryTracker) {
    use std::io::{self, Write};
    
    println!("Interactive Ownership Demo");
    println!("Try these commands:");
    println!("  create <name>  - Create a new String");
    println!("  move <from> <to>  - Move ownership");
    println!("  clone <from> <to> - Clone a String");
    println!("  drop <name>    - Drop a value");
    println!("  list           - List active values");
    println!("  back           - Return to main menu");
    
    let mut active_values: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    loop {
        print!("ownership> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        match parts.as_slice() {
            ["create", name] => {
                let value = format!("value_{}", name);
                tracker.track_allocation(name, "String", value.len());
                active_values.insert(name.to_string(), value);
                println!("✅ Created {}", name);
            }
            ["move", from, to] => {
                if let Some(value) = active_values.remove(*from) {
                    tracker.track_move(from, to, "String");
                    active_values.insert(to.to_string(), value);
                    println!("✅ Moved {} to {}", from, to);
                } else {
                    println!("❌ {} not found", from);
                }
            }
            ["clone", from, to] => {
                if let Some(value) = active_values.get(*from) {
                    tracker.track_clone(from, to, "String", value.len());
                    active_values.insert(to.to_string(), value.clone());
                    println!("✅ Cloned {} to {}", from, to);
                } else {
                    println!("❌ {} not found", from);
                }
            }
            ["drop", name] => {
                if let Some(value) = active_values.remove(*name) {
                    tracker.track_deallocation(name, value.len());
                    println!("✅ Dropped {}", name);
                } else {
                    println!("❌ {} not found", name);
                }
            }
            ["list"] => {
                if active_values.is_empty() {
                    println!("No active values");
                } else {
                    println!("Active values:");
                    for (name, value) in &active_values {
                        println!("  {}: \"{}\"", name, value);
                    }
                }
            }
            ["back"] => break,
            _ => {
                println!("Unknown command. Available: create, move, clone, drop, list, back");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ownership_concepts() {
        let mut tracker = MemoryTracker::new();
        
        // TODO: Write tests that verify ownership behavior
        // Test that moves transfer ownership
        // Test that clones create independent copies
        // Test that values are properly dropped
    }
    
    #[test]
    fn test_copy_types() {
        // TODO: Test that Copy types can be used after assignment
        let x = 42;
        let y = x;
        assert_eq!(x, 42);  // x should still be valid
        assert_eq!(y, 42);  // y should have the same value
    }
    
    #[test]
    fn test_move_types() {
        // TODO: Test move semantics with String
        // Note: You can't test compilation errors in unit tests,
        // but you can test the behavior that should work
    }
}
