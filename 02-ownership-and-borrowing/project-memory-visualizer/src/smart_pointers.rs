// Smart Pointers Demonstration - Complete the implementations!
//
// Your task: Complete the smart pointer examples and fix compilation errors
// This demonstrates Box, Rc, Arc, and RefCell from Module 02

use crate::memory_tracker::MemoryTracker;
use colored::*;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

pub fn run_demonstrations(tracker: &mut MemoryTracker) {
    println!("Running smart pointer demonstrations...\n");
    
    println!("{}", "1. Box<T> - Heap Allocation".bold());
    demo_box_pointer(tracker);
    
    println!("\n{}", "2. Rc<T> - Reference Counting".bold());
    demo_rc_pointer(tracker);
    
    println!("\n{}", "3. RefCell<T> - Interior Mutability".bold());
    demo_refcell(tracker);
    
    println!("\n{}", "4. Arc<T> - Thread-Safe Reference Counting".bold());
    demo_arc_pointer(tracker);
    
    println!("\n{}", "5. Combining Smart Pointers".bold());
    demo_combined_patterns(tracker);
}

fn demo_box_pointer(tracker: &mut MemoryTracker) {
    println!("Demonstrating Box<T> for heap allocation...");
    
    // Box moves data to the heap
    let boxed_value = Box::new(42);
    tracker.track_allocation("boxed_value", "Box<i32>", std::mem::size_of::<i32>());
    
    println!("Boxed value: {}", boxed_value);
    
    // TODO: Create a Box containing a String
    let boxed_string = /* TODO: Box a String */;
    // tracker.track_allocation("boxed_string", "Box<String>", /* TODO: size */);
    
    println!("Boxed string: {}", boxed_string);
    
    // Box is useful for recursive data structures
    // TODO: Uncomment and fix this recursive type definition
    // enum List {
    //     Cons(i32, List),  // FIXME: This doesn't work - infinite size!
    //     Nil,
    // }
    
    // TODO: Fix the recursive type using Box
    enum FixedList {
        // TODO: Use Box to make this work
        Nil,
    }
    
    // TODO: Create a simple linked list using Box
    let list = /* TODO: Create a list with values 1, 2, 3 */;
    
    println!("Box demo complete");
}

fn demo_rc_pointer(tracker: &mut MemoryTracker) {
    println!("Demonstrating Rc<T> for shared ownership...");
    
    // Create shared data
    let shared_data = Rc::new(String::from("shared"));
    tracker.track_allocation("shared_data", "Rc<String>", shared_data.len());
    
    println!("Reference count: {}", Rc::strong_count(&shared_data));
    
    // TODO: Clone the Rc to create another reference
    let reference1 = /* TODO: Clone the Rc */;
    tracker.track_clone("shared_data", "reference1", "Rc<String>", 0); // Rc cloning doesn't allocate
    
    // TODO: Create another reference
    let reference2 = /* TODO: Clone the Rc again */;
    
    println!("Reference count after cloning: {}", Rc::strong_count(&shared_data));
    
    // All references point to the same data
    println!("shared_data: {}", shared_data);
    println!("reference1: {}", reference1);
    println!("reference2: {}", reference2);
    
    // TODO: What happens when we drop references?
    drop(reference1);
    println!("After dropping reference1: {}", Rc::strong_count(&shared_data));
    
    // TODO: Drop the remaining references and observe the count
    
    // CHALLENGE: When is the actual data deallocated?
    
    // Example: Multiple owners pattern
    demo_multiple_owners(tracker);
}

fn demo_multiple_owners(tracker: &mut MemoryTracker) {
    println!("\nMultiple owners pattern with Rc...");
    
    // TODO: Create a data structure that needs multiple owners
    // Example: A node that can be referenced by multiple parents
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        // TODO: Add children field using Rc
        // children: Vec</*TODO: Rc<Node>*/>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                // children: Vec::new(),
            })
        }
        
        // TODO: Add method to add child
        // fn add_child(&mut self, child: /*TODO: type*/) {
        //     // TODO: Implementation
        // }
    }
    
    // TODO: Create a tree structure where nodes can have multiple parents
}

fn demo_refcell(tracker: &mut MemoryTracker) {
    println!("Demonstrating RefCell<T> for interior mutability...");
    
    // RefCell allows mutation through immutable references
    let data = RefCell::new(42);
    tracker.track_allocation("data", "RefCell<i32>", std::mem::size_of::<i32>());
    
    println!("Initial value: {}", data.borrow());
    
    // TODO: Modify the value through RefCell
    // HINT: Use borrow_mut()
    {
        let mut borrowed = /* TODO: Get mutable borrow */;
        *borrowed = 100;
    }  // Mutable borrow is dropped here
    
    println!("Modified value: {}", data.borrow());
    
    // TODO: What happens if we try to violate borrowing rules at runtime?
    // Try this and see what happens:
    // let _borrow1 = data.borrow();
    // let _borrow2 = data.borrow_mut();  // PANIC! Runtime borrow check failure
    
    // Practical example: Interior mutability pattern
    demo_interior_mutability_pattern(tracker);
}

fn demo_interior_mutability_pattern(tracker: &mut MemoryTracker) {
    println!("\nInterior mutability pattern...");
    
    // TODO: Complete this struct that needs interior mutability
    struct Counter {
        // TODO: Use RefCell to allow mutation through &self
        // count: /*TODO: RefCell<i32>*/,
    }
    
    impl Counter {
        fn new() -> Self {
            Counter {
                // count: /*TODO: Initialize RefCell*/,
            }
        }
        
        // TODO: Method to increment counter (takes &self, not &mut self)
        fn increment(&self) {
            // TODO: Use RefCell to modify count
        }
        
        // TODO: Method to get current count
        fn get(&self) -> i32 {
            // TODO: Return current count
            0
        }
    }
    
    // TODO: Test the counter
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter value: {}", counter.get());
}

fn demo_arc_pointer(tracker: &mut MemoryTracker) {
    println!("Demonstrating Arc<T> for thread-safe shared ownership...");
    
    // Arc is the thread-safe version of Rc
    let shared_data = Arc::new(String::from("thread-safe data"));
    tracker.track_allocation("shared_data", "Arc<String>", shared_data.len());
    
    println!("Arc reference count: {}", Arc::strong_count(&shared_data));
    
    // TODO: Clone Arc for sharing between threads
    let data_clone = /* TODO: Clone the Arc */;
    
    // Note: We can't actually spawn threads in this simple demo,
    // but in real code you would do something like:
    
    // let data_for_thread = Arc::clone(&shared_data);
    // thread::spawn(move || {
    //     println!("Thread data: {}", data_for_thread);
    // });
    
    println!("Data: {}", shared_data);
    println!("Clone: {}", data_clone);
    
    // TODO: When would you use Arc vs Rc?
    // Arc: [Your answer here]
    // Rc:  [Your answer here]
}

fn demo_combined_patterns(tracker: &mut MemoryTracker) {
    println!("Demonstrating combined smart pointer patterns...");
    
    // Common pattern: Arc<Mutex<T>> for shared mutable state
    use std::sync::Mutex;
    
    let shared_counter = Arc::new(Mutex::new(0));
    tracker.track_allocation("shared_counter", "Arc<Mutex<i32>>", std::mem::size_of::<i32>());
    
    // TODO: Clone for sharing
    let counter_clone = /* TODO: Clone the Arc */;
    
    // TODO: Modify through the mutex
    {
        let mut guard = /* TODO: Lock the mutex */;
        *guard += 1;
    }
    
    // Access through the clone
    {
        let guard = counter_clone.lock().unwrap();
        println!("Counter value: {}", *guard);
    }
    
    // TODO: Complete this more complex example
    demo_shared_data_structure(tracker);
}

fn demo_shared_data_structure(tracker: &mut MemoryTracker) {
    println!("\nShared data structure example...");
    
    // TODO: Create a data structure that can be safely shared between threads
    // and allows mutation
    
    use std::sync::Mutex;
    
    #[derive(Debug)]
    struct SharedList {
        // TODO: Use appropriate smart pointers
        // items: /*TODO: Thread-safe mutable collection*/,
    }
    
    impl SharedList {
        fn new() -> Arc<Self> {
            Arc::new(SharedList {
                // items: /*TODO: Initialize*/,
            })
        }
        
        // TODO: Add method to insert item
        fn push(&self, item: i32) {
            // TODO: Implementation using Mutex
        }
        
        // TODO: Add method to get length
        fn len(&self) -> usize {
            // TODO: Implementation
            0
        }
    }
    
    // TODO: Test the shared list
    let list = SharedList::new();
    list.push(1);
    list.push(2);
    println!("List length: {}", list.len());
}

pub fn interactive_demo(tracker: &mut MemoryTracker) {
    use std::io::{self, Write};
    
    println!("Interactive Smart Pointers Demo");
    println!("Commands:");
    println!("  box <value>     - Create Box<i32>");
    println!("  rc <value>      - Create Rc<i32>");
    println!("  clone_rc <n>    - Clone an Rc");
    println!("  refcell <value> - Create RefCell<i32>");
    println!("  modify <n>      - Modify RefCell value");
    println!("  count <n>       - Show Rc reference count");
    println!("  back            - Return to main menu");
    
    let mut boxes: std::collections::HashMap<String, Box<i32>> = std::collections::HashMap::new();
    let mut rcs: std::collections::HashMap<String, Rc<i32>> = std::collections::HashMap::new();
    let mut refcells: std::collections::HashMap<String, RefCell<i32>> = std::collections::HashMap::new();
    
    loop {
        print!("smart-pointers> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        match parts.as_slice() {
            ["box", value] => {
                if let Ok(val) = value.parse::<i32>() {
                    let name = format!("box_{}", boxes.len());
                    tracker.track_allocation(&name, "Box<i32>", 4);
                    boxes.insert(name.clone(), Box::new(val));
                    println!("✅ Created {} with value {}", name, val);
                } else {
                    println!("❌ Invalid number: {}", value);
                }
            }
            ["rc", value] => {
                if let Ok(val) = value.parse::<i32>() {
                    let name = format!("rc_{}", rcs.len());
                    tracker.track_allocation(&name, "Rc<i32>", 4);
                    rcs.insert(name.clone(), Rc::new(val));
                    println!("✅ Created {} with value {}", name, val);
                } else {
                    println!("❌ Invalid number: {}", value);
                }
            }
            ["clone_rc", name] => {
                if let Some(rc) = rcs.get(*name) {
                    let clone_name = format!("{}_clone", name);
                    let cloned = Rc::clone(rc);
                    println!("✅ Cloned {} to {} (ref count: {})", 
                        name, clone_name, Rc::strong_count(&cloned));
                    rcs.insert(clone_name, cloned);
                } else {
                    println!("❌ Rc {} not found", name);
                }
            }
            ["refcell", value] => {
                if let Ok(val) = value.parse::<i32>() {
                    let name = format!("refcell_{}", refcells.len());
                    tracker.track_allocation(&name, "RefCell<i32>", 4);
                    refcells.insert(name.clone(), RefCell::new(val));
                    println!("✅ Created {} with value {}", name, val);
                } else {
                    println!("❌ Invalid number: {}", value);
                }
            }
            ["count", name] => {
                if let Some(rc) = rcs.get(*name) {
                    println!("Reference count for {}: {}", name, Rc::strong_count(rc));
                } else {
                    println!("❌ Rc {} not found", name);
                }
            }
            ["back"] => break,
            _ => {
                println!("Unknown command");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_allocation() {
        let boxed = Box::new(42);
        assert_eq!(*boxed, 42);
    }
    
    #[test]
    fn test_rc_sharing() {
        let rc1 = Rc::new(100);
        let rc2 = Rc::clone(&rc1);
        
        assert_eq!(*rc1, 100);
        assert_eq!(*rc2, 100);
        assert_eq!(Rc::strong_count(&rc1), 2);
        assert_eq!(Rc::strong_count(&rc2), 2);
    }
    
    #[test]
    fn test_refcell_interior_mutability() {
        let cell = RefCell::new(5);
        
        // Modify through immutable reference
        *cell.borrow_mut() = 10;
        
        assert_eq!(*cell.borrow(), 10);
    }
    
    // TODO: Add more tests for smart pointer patterns
    #[test]
    fn test_combined_patterns() {
        // TODO: Test Arc<Mutex<T>> pattern
    }
}
