// Exercise 5: Advanced Ownership Patterns - Master level challenges!
//
// Your task: Complete advanced real-world ownership patterns
// This combines all concepts from Module 02: ownership, borrowing, lifetimes, and smart pointers
//
// INSTRUCTIONS:
// 1. These are challenging problems that combine multiple concepts
// 2. Read the requirements carefully and plan your approach
// 3. Use your knowledge from exercises 1-4 to solve these
// 4. Run `rustc ex05-advanced-patterns.rs && ./ex05-advanced-patterns` to test

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    println!("=== Exercise 5: Advanced Ownership Patterns (Master Level!) ===\n");
    
    // CHALLENGE: Uncomment each exercise as you complete the implementations
    exercise_5_1();
    // exercise_5_2();
    // exercise_5_3();
    // exercise_5_4();
    // exercise_5_5();
    // exercise_5_6();
    // exercise_5_7();
    // exercise_5_8();
    
    println!("\n🎉 You've mastered Rust ownership patterns!");
}

// Exercise 5.1: Zero-copy string processing
// CHALLENGE: Process text without creating unnecessary allocations
fn exercise_5_1() {
    println!("Exercise 5.1: Zero-copy string processing");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    // TODO: Implement these functions to work with string slices
    let word_count = count_words(&text);
    let longest = find_longest_word(&text);
    let words_starting_with_t = find_words_starting_with(&text, 't');
    
    println!("Word count: {}", word_count);
    println!("Longest word: {:?}", longest);
    println!("Words starting with 't': {:?}", words_starting_with_t);
    
    // REQUIREMENT: Your functions should not allocate new strings
    // HINT: Return string slices and vectors of slices
    
    println!("✅ Exercise 5.1 complete\n");
}

// TODO: Implement these zero-copy string processing functions
fn count_words(text: &str) -> usize {
    // TODO: Count words without allocation
    // HINT: Use split_whitespace()
    todo!("Count words in the text")
}

fn find_longest_word(text: &str) -> Option<&str> {
    // TODO: Find the longest word, return a slice
    // HINT: Use split_whitespace() and max_by_key()
    todo!("Find the longest word")
}

fn find_words_starting_with(text: &str, prefix: char) -> Vec<&str> {
    // TODO: Find all words starting with the given character
    // HINT: Use filter() and starts_with()
    todo!("Find words with prefix")
}

// Exercise 5.2: Builder pattern with ownership transfer
// CHALLENGE: Implement a builder that properly handles ownership
fn exercise_5_2() {
    println!("Exercise 5.2: Builder pattern with ownership transfer");
    
    // TODO: Make this builder pattern work
    let config = ConfigBuilder::new()
        .name("MyApp".to_string())           // Should transfer ownership
        .version("1.0.0".to_string())        // Should transfer ownership  
        .add_feature("logging".to_string())  // Should transfer ownership
        .add_feature("metrics".to_string())  // Should transfer ownership
        .build();                            // Should consume builder
    
    println!("Config: {:#?}", config);
    
    // REQUIREMENT: Builder should be consumed on build()
    // REQUIREMENT: All strings should be owned, not borrowed
    
    println!("✅ Exercise 5.2 complete\n");
}

// TODO: Implement the Config and ConfigBuilder structs
#[derive(Debug)]
struct Config {
    // TODO: Define the fields
    // name: String,
    // version: String,
    // features: Vec<String>,
}

struct ConfigBuilder {
    // TODO: Define the builder fields
    // What should these be? Option<String>? Or just String with defaults?
}

impl ConfigBuilder {
    fn new() -> Self {
        // TODO: Initialize builder
        todo!("Create new ConfigBuilder")
    }
    
    fn name(self, name: String) -> Self {
        // TODO: Set name and return self
        // THINK: Why does this take `self` not `&mut self`?
        todo!("Set name")
    }
    
    fn version(self, version: String) -> Self {
        // TODO: Set version and return self
        todo!("Set version")
    }
    
    fn add_feature(self, feature: String) -> Self {
        // TODO: Add feature to the list
        todo!("Add feature")
    }
    
    fn build(self) -> Config {
        // TODO: Consume builder and create Config
        // THINK: What should happen if required fields are missing?
        todo!("Build the config")
    }
}

// Exercise 5.3: Shared mutable state with interior mutability
// CHALLENGE: Create a thread-safe cache
fn exercise_5_3() {
    println!("Exercise 5.3: Shared mutable state with interior mutability");
    
    let cache = ThreadSafeCache::new();
    
    // Store some values
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    
    // Retrieve values
    println!("key1: {:?}", cache.get("key1"));
    println!("key2: {:?}", cache.get("key2"));
    println!("key3: {:?}", cache.get("key3"));
    
    // Clear cache
    cache.clear();
    println!("After clear - key1: {:?}", cache.get("key1"));
    
    println!("✅ Exercise 5.3 complete\n");
}

// TODO: Implement a thread-safe cache using Arc<Mutex<HashMap>>
struct ThreadSafeCache {
    // TODO: What type should this be?
    // data: /* TODO: Arc<Mutex<HashMap<String, String>>> */,
}

impl ThreadSafeCache {
    fn new() -> Self {
        // TODO: Initialize the cache
        todo!("Create new ThreadSafeCache")
    }
    
    fn insert(&self, key: String, value: String) {
        // TODO: Lock the mutex and insert
        todo!("Insert key-value pair")
    }
    
    fn get(&self, key: &str) -> Option<String> {
        // TODO: Lock the mutex and get value
        // HINT: You might need to clone the value
        todo!("Get value by key")
    }
    
    fn clear(&self) {
        // TODO: Lock the mutex and clear all entries
        todo!("Clear all entries")
    }
}

// TODO: Implement Clone for ThreadSafeCache
// impl Clone for ThreadSafeCache {
//     fn clone(&self) -> Self {
//         // TODO: Clone the Arc, not the data
//     }
// }

// Exercise 5.4: Command pattern with ownership
// CHALLENGE: Implement a command system that owns its data
fn exercise_5_4() {
    println!("Exercise 5.4: Command pattern with ownership");
    
    let mut executor = CommandExecutor::new();
    
    // Create and execute commands
    let cmd1 = PrintCommand::new("Hello, World!".to_string());
    let cmd2 = AddNumberCommand::new(5, 3);
    let cmd3 = PrintCommand::new("Goodbye!".to_string());
    
    executor.execute(Box::new(cmd1));
    executor.execute(Box::new(cmd2));
    executor.execute(Box::new(cmd3));
    
    println!("Executed {} commands", executor.command_count());
    
    println!("✅ Exercise 5.4 complete\n");
}

// TODO: Define the Command trait
trait Command {
    fn execute(&self);
}

// TODO: Implement PrintCommand
struct PrintCommand {
    // TODO: Store the message to print
    // message: String,
}

impl PrintCommand {
    fn new(message: String) -> Self {
        // TODO: Create new print command
        todo!("Create PrintCommand")
    }
}

// TODO: Implement Command trait for PrintCommand
// impl Command for PrintCommand {
//     fn execute(&self) {
//         // TODO: Print the message
//     }
// }

// TODO: Implement AddNumberCommand
struct AddNumberCommand {
    // TODO: Store the numbers to add
    // a: i32,
    // b: i32,
}

impl AddNumberCommand {
    fn new(a: i32, b: i32) -> Self {
        // TODO: Create new add command
        todo!("Create AddNumberCommand")
    }
}

// TODO: Implement Command trait for AddNumberCommand
// impl Command for AddNumberCommand {
//     fn execute(&self) {
//         // TODO: Add the numbers and print result
//     }
// }

// TODO: Implement CommandExecutor
struct CommandExecutor {
    // TODO: Track how many commands have been executed
    // count: usize,
}

impl CommandExecutor {
    fn new() -> Self {
        // TODO: Create new executor
        todo!("Create CommandExecutor")
    }
    
    fn execute(&mut self, command: Box<dyn Command>) {
        // TODO: Execute the command and increment count
        todo!("Execute command")
    }
    
    fn command_count(&self) -> usize {
        // TODO: Return the number of executed commands
        todo!("Get command count")
    }
}

// Exercise 5.5: Memory pool for batch processing
// CHALLENGE: Implement a memory pool that reuses allocations
fn exercise_5_5() {
    println!("Exercise 5.5: Memory pool for batch processing");
    
    let mut pool = StringPool::new();
    
    // Process multiple batches of strings
    for batch in 0..3 {
        println!("Processing batch {}", batch);
        
        let mut strings = Vec::new();
        
        // Get strings from pool (reused memory)
        for i in 0..5 {
            let mut s = pool.get_string();
            s.push_str(&format!("Batch {} Item {}", batch, i));
            strings.push(s);
        }
        
        // Use the strings
        for s in &strings {
            println!("  {}", s);
        }
        
        // Return strings to pool for reuse
        for s in strings {
            pool.return_string(s);
        }
    }
    
    println!("Pool efficiency: {:.1}% reuse", pool.reuse_rate() * 100.0);
    
    println!("✅ Exercise 5.5 complete\n");
}

// TODO: Implement a string pool for memory efficiency
struct StringPool {
    // TODO: Store reusable strings
    // available: Vec<String>,
    // total_requests: usize,
    // reuses: usize,
}

impl StringPool {
    fn new() -> Self {
        // TODO: Create new pool
        todo!("Create StringPool")
    }
    
    fn get_string(&mut self) -> String {
        // TODO: Return a reused string if available, otherwise create new
        // HINT: Pop from available vec or create String::new()
        todo!("Get string from pool")
    }
    
    fn return_string(&mut self, mut s: String) {
        // TODO: Clear the string and add it back to available pool
        // HINT: Use s.clear() to reset it
        todo!("Return string to pool")
    }
    
    fn reuse_rate(&self) -> f64 {
        // TODO: Calculate what percentage of requests were reuses
        todo!("Calculate reuse rate")
    }
}

// Exercise 5.6: Copy-on-write (COW) implementation
// CHALLENGE: Implement efficient copy-on-write semantics
fn exercise_5_6() {
    println!("Exercise 5.6: Copy-on-write (COW) implementation");
    
    let original = MyString::from("Hello, World!");
    println!("Original: {}", original);
    
    // Create copies that share data
    let copy1 = original.clone();
    let copy2 = original.clone();
    
    println!("Copy1: {}", copy1);
    println!("Copy2: {}", copy2);
    println!("Shared data: {}", MyString::shared_instances());
    
    // Modify one copy - should trigger copy-on-write
    let mut copy3 = copy1.clone();
    copy3.push_str(" Modified");
    
    println!("After modification:");
    println!("Original: {}", original);
    println!("Copy3: {}", copy3);
    println!("Shared instances: {}", MyString::shared_instances());
    
    println!("✅ Exercise 5.6 complete\n");
}

// TODO: Implement copy-on-write string
use std::fmt;

struct MyString {
    // TODO: Use Rc to share data until modification
    // data: Rc<String>,
}

impl MyString {
    fn from(s: &str) -> Self {
        // TODO: Create new MyString with shared data
        todo!("Create MyString from &str")
    }
    
    fn push_str(&mut self, s: &str) {
        // TODO: Implement copy-on-write logic
        // If Rc has only one reference, modify in place
        // Otherwise, create a new string with the modification
        // HINT: Use Rc::try_unwrap() or Rc::make_mut() pattern
        todo!("Push string with COW")
    }
    
    fn shared_instances() -> usize {
        // TODO: Count how many MyString instances share data
        // This is a simplified version - in real COW you'd track this
        0
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        // TODO: Clone the Rc, not the string data
        todo!("Clone MyString (should share data)")
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Display the inner string
        todo!("Display MyString")
    }
}

// Exercise 5.7: Observer pattern with weak references
// CHALLENGE: Implement observer pattern that avoids memory leaks
fn exercise_5_7() {
    println!("Exercise 5.7: Observer pattern with weak references");
    
    let mut subject = Subject::new();
    
    // Create observers
    let observer1 = Rc::new(RefCell::new(ConsoleObserver::new("Observer1")));
    let observer2 = Rc::new(RefCell::new(ConsoleObserver::new("Observer2")));
    
    // Register observers
    subject.attach(Rc::downgrade(&observer1));
    subject.attach(Rc::downgrade(&observer2));
    
    // Notify observers
    subject.notify("First notification");
    subject.notify("Second notification");
    
    // Drop an observer and notify again
    drop(observer1);
    subject.notify("Third notification (after drop)");
    
    println!("✅ Exercise 5.7 complete\n");
}

// TODO: Implement Subject that holds weak references to observers
struct Subject {
    // TODO: Store weak references to avoid cycles
    // observers: Vec<Weak<RefCell<dyn Observer>>>,
}

trait Observer {
    fn update(&mut self, message: &str);
}

impl Subject {
    fn new() -> Self {
        // TODO: Create new subject
        todo!("Create Subject")
    }
    
    fn attach(&mut self, observer: Weak<RefCell<dyn Observer>>) {
        // TODO: Add weak reference to observer list
        todo!("Attach observer")
    }
    
    fn notify(&mut self, message: &str) {
        // TODO: Notify all alive observers
        // HINT: Use weak.upgrade() to check if observer still exists
        todo!("Notify observers")
    }
}

// TODO: Implement ConsoleObserver
struct ConsoleObserver {
    // name: String,
}

impl ConsoleObserver {
    fn new(name: &str) -> Self {
        // TODO: Create new console observer
        todo!("Create ConsoleObserver")
    }
}

// TODO: Implement Observer trait for ConsoleObserver
// impl Observer for ConsoleObserver {
//     fn update(&mut self, message: &str) {
//         // TODO: Print the message with observer name
//     }
// }

// Exercise 5.8: Advanced data structure with all patterns
// CHALLENGE: Combine everything into a complex data structure
fn exercise_5_8() {
    println!("Exercise 5.8: Advanced data structure with all patterns");
    
    let mut graph = Graph::new();
    
    // Add nodes
    let node1 = graph.add_node("Node1".to_string());
    let node2 = graph.add_node("Node2".to_string());
    let node3 = graph.add_node("Node3".to_string());
    
    // Add edges
    graph.add_edge(node1, node2);
    graph.add_edge(node2, node3);
    graph.add_edge(node3, node1);  // Create a cycle
    
    // Traverse the graph
    println!("Graph traversal:");
    graph.traverse_from(node1);
    
    println!("✅ Exercise 5.8 complete\n");
}

// TODO: Implement a graph data structure using multiple smart pointer patterns
use std::collections::VecDeque;

type NodeId = usize;

struct Graph {
    // TODO: Store nodes and edges
    // nodes: Vec<Rc<RefCell<GraphNode>>>,
    // next_id: NodeId,
}

struct GraphNode {
    // TODO: Define node structure
    // id: NodeId,
    // data: String,
    // edges: RefCell<Vec<Weak<RefCell<GraphNode>>>>,  // Use Weak to avoid cycles
}

impl Graph {
    fn new() -> Self {
        // TODO: Create new graph
        todo!("Create Graph")
    }
    
    fn add_node(&mut self, data: String) -> NodeId {
        // TODO: Add a new node and return its ID
        todo!("Add node to graph")
    }
    
    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        // TODO: Add edge between nodes
        // HINT: Use weak references to avoid cycles
        todo!("Add edge between nodes")
    }
    
    fn traverse_from(&self, start: NodeId) {
        // TODO: Implement breadth-first traversal
        // HINT: Use a queue and track visited nodes
        todo!("Traverse graph from starting node")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_when_implemented_zero_copy_processing() {
        // This test will work once you implement the zero-copy functions
        // let text = "hello world rust";
        // assert_eq!(count_words(text), 3);
        // assert_eq!(find_longest_word(text), Some("world"));
        // assert_eq!(find_words_starting_with(text, 'r'), vec!["rust"]);
    }
    
    #[test]
    fn test_when_implemented_config_builder() {
        // This test will work once you implement the builder pattern
        // let config = ConfigBuilder::new()
        //     .name("Test".to_string())
        //     .version("1.0".to_string())
        //     .build();
        // 
        // // Verify the config is built correctly
    }
    
    // TODO: Add more tests for your implementations
}
