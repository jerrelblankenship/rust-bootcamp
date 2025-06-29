# Exercise 5 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working implementation. Here's the full solution with detailed explanations for all advanced ownership patterns.

## 📝 Complete ex05-advanced-patterns.rs Implementation

```rust
// Exercise 5: Advanced Ownership Patterns - Complete Solutions
//
// This file demonstrates master-level ownership patterns combining all Module 02 concepts

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Exercise 5: Advanced Ownership Patterns (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_5_1();
    exercise_5_2();
    exercise_5_3();
    exercise_5_4();
    exercise_5_5();
    exercise_5_6();
    exercise_5_7();
    exercise_5_8();
    
    println!("\n🎉 You've mastered Rust ownership patterns!");
    
    // Demonstrate production-ready patterns
    demonstrate_production_patterns();
}

// Exercise 5.1: Zero-copy string processing - SOLVED
fn exercise_5_1() {
    println!("Exercise 5.1: Zero-copy string processing");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    let word_count = count_words(text);
    let longest = find_longest_word(text);
    let words_starting_with_t = find_words_starting_with(text, 't');
    
    println!("Word count: {}", word_count);
    println!("Longest word: {:?}", longest);
    println!("Words starting with 't': {:?}", words_starting_with_t);
    
    // Demonstrate advanced zero-copy operations
    demonstrate_advanced_text_processing(text);
    
    println!("✅ Exercise 5.1 complete\n");
}

// SOLUTION: Zero-copy string processing functions
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn find_longest_word(text: &str) -> Option<&str> {
    text.split_whitespace()
        .max_by_key(|word| word.len())
}

fn find_words_starting_with(text: &str, prefix: char) -> Vec<&str> {
    text.split_whitespace()
        .filter(|word| word.starts_with(prefix))
        .collect()
}

fn demonstrate_advanced_text_processing(text: &str) {
    println!("Advanced zero-copy operations:");
    
    // Find words by length
    let short_words: Vec<&str> = text.split_whitespace()
        .filter(|word| word.len() <= 3)
        .collect();
    println!("Short words: {:?}", short_words);
    
    // Get word frequencies (still zero-copy keys)
    let mut word_freq = HashMap::new();
    for word in text.split_whitespace() {
        *word_freq.entry(word).or_insert(0) += 1;
    }
    println!("Word frequencies: {:?}", word_freq);
    
    // Find sentences (if semicolons existed)
    let sentences: Vec<&str> = text.split(". ").collect();
    println!("Sentences: {:?}", sentences);
}

// Exercise 5.2: Builder pattern with ownership transfer - SOLVED
fn exercise_5_2() {
    println!("Exercise 5.2: Builder pattern with ownership transfer");
    
    let config = ConfigBuilder::new()
        .name("MyApp".to_string())
        .version("1.0.0".to_string())
        .add_feature("logging".to_string())
        .add_feature("metrics".to_string())
        .build();
    
    println!("Config: {:#?}", config);
    
    // Demonstrate error handling in builder
    demonstrate_builder_validation();
    
    println!("✅ Exercise 5.2 complete\n");
}

// SOLUTION: Complete Config and ConfigBuilder implementation
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    features: Vec<String>,
    debug_mode: bool,
    max_connections: usize,
}

struct ConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    features: Vec<String>,
    debug_mode: bool,
    max_connections: usize,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            name: None,
            version: None,
            features: Vec::new(),
            debug_mode: false,
            max_connections: 100,
        }
    }
    
    // Each method consumes self and returns Self for chaining
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }
    
    fn add_feature(mut self, feature: String) -> Self {
        self.features.push(feature);
        self
    }
    
    fn debug_mode(mut self, enabled: bool) -> Self {
        self.debug_mode = enabled;
        self
    }
    
    fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    // Build consumes the builder and produces the final config
    fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Unnamed App".to_string()),
            version: self.version.unwrap_or_else(|| "0.1.0".to_string()),
            features: self.features,
            debug_mode: self.debug_mode,
            max_connections: self.max_connections,
        }
    }
    
    // Alternative build with validation
    fn try_build(self) -> Result<Config, String> {
        let name = self.name.ok_or("Name is required")?;
        
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if self.max_connections == 0 {
            return Err("Max connections must be greater than 0".to_string());
        }
        
        Ok(Config {
            name,
            version: self.version.unwrap_or_else(|| "0.1.0".to_string()),
            features: self.features,
            debug_mode: self.debug_mode,
            max_connections: self.max_connections,
        })
    }
}

fn demonstrate_builder_validation() {
    println!("Builder with validation:");
    
    // Valid config
    match ConfigBuilder::new()
        .name("ValidApp".to_string())
        .max_connections(50)
        .try_build() {
        Ok(config) => println!("Valid config: {:?}", config.name),
        Err(e) => println!("Error: {}", e),
    }
    
    // Invalid config
    match ConfigBuilder::new()
        .max_connections(0)  // Invalid
        .try_build() {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {}", e),
    }
}

// Exercise 5.3: Shared mutable state with interior mutability - SOLVED
fn exercise_5_3() {
    println!("Exercise 5.3: Shared mutable state with interior mutability");
    
    let cache = ThreadSafeCache::new();
    
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    
    println!("key1: {:?}", cache.get("key1"));
    println!("key2: {:?}", cache.get("key2"));
    println!("key3: {:?}", cache.get("key3"));
    
    cache.clear();
    println!("After clear - key1: {:?}", cache.get("key1"));
    
    // Demonstrate thread safety
    demonstrate_concurrent_cache_access(cache.clone());
    
    println!("✅ Exercise 5.3 complete\n");
}

// SOLUTION: Thread-safe cache using Arc<Mutex<HashMap>>
struct ThreadSafeCache {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl ThreadSafeCache {
    fn new() -> Self {
        ThreadSafeCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    fn insert(&self, key: String, value: String) {
        let mut map = self.data.lock().unwrap();
        map.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<String> {
        let map = self.data.lock().unwrap();
        map.get(key).cloned()
    }
    
    fn clear(&self) {
        let mut map = self.data.lock().unwrap();
        map.clear();
    }
    
    fn len(&self) -> usize {
        let map = self.data.lock().unwrap();
        map.len()
    }
    
    fn keys(&self) -> Vec<String> {
        let map = self.data.lock().unwrap();
        map.keys().cloned().collect()
    }
}

impl Clone for ThreadSafeCache {
    fn clone(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),
        }
    }
}

fn demonstrate_concurrent_cache_access(cache: ThreadSafeCache) {
    println!("Concurrent cache access:");
    
    let mut handles = vec![];
    
    // Spawn threads that write to cache
    for i in 0..3 {
        let cache_clone = cache.clone();
        let handle = thread::spawn(move || {
            cache_clone.insert(format!("thread_{}", i), format!("value_{}", i));
            println!("Thread {} inserted data", i);
        });
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final cache contents: {:?}", cache.keys());
}

// Exercise 5.4: Command pattern with ownership - SOLVED
fn exercise_5_4() {
    println!("Exercise 5.4: Command pattern with ownership");
    
    let mut executor = CommandExecutor::new();
    
    let cmd1 = PrintCommand::new("Hello, World!".to_string());
    let cmd2 = AddNumberCommand::new(5, 3);
    let cmd3 = PrintCommand::new("Goodbye!".to_string());
    
    executor.execute(Box::new(cmd1));
    executor.execute(Box::new(cmd2));
    executor.execute(Box::new(cmd3));
    
    println!("Executed {} commands", executor.command_count());
    
    // Demonstrate advanced command patterns
    demonstrate_advanced_commands(&mut executor);
    
    println!("✅ Exercise 5.4 complete\n");
}

// SOLUTION: Command trait and implementations
trait Command {
    fn execute(&self);
    fn description(&self) -> &str { "Unknown command" }
    fn can_undo(&self) -> bool { false }
}

struct PrintCommand {
    message: String,
}

impl PrintCommand {
    fn new(message: String) -> Self {
        PrintCommand { message }
    }
}

impl Command for PrintCommand {
    fn execute(&self) {
        println!("📝 {}", self.message);
    }
    
    fn description(&self) -> &str {
        "Print message command"
    }
}

struct AddNumberCommand {
    a: i32,
    b: i32,
}

impl AddNumberCommand {
    fn new(a: i32, b: i32) -> Self {
        AddNumberCommand { a, b }
    }
}

impl Command for AddNumberCommand {
    fn execute(&self) {
        println!("🔢 {} + {} = {}", self.a, self.b, self.a + self.b);
    }
    
    fn description(&self) -> &str {
        "Add numbers command"
    }
}

// Additional command types
struct DelayCommand {
    duration_ms: u64,
    message: String,
}

impl DelayCommand {
    fn new(duration_ms: u64, message: String) -> Self {
        DelayCommand { duration_ms, message }
    }
}

impl Command for DelayCommand {
    fn execute(&self) {
        println!("⏰ Waiting {}ms...", self.duration_ms);
        thread::sleep(Duration::from_millis(self.duration_ms));
        println!("⏰ {}", self.message);
    }
    
    fn description(&self) -> &str {
        "Delay command"
    }
}

struct CommandExecutor {
    count: usize,
    history: Vec<String>,
}

impl CommandExecutor {
    fn new() -> Self {
        CommandExecutor {
            count: 0,
            history: Vec::new(),
        }
    }
    
    fn execute(&mut self, command: Box<dyn Command>) {
        println!("Executing: {}", command.description());
        self.history.push(command.description().to_string());
        command.execute();
        self.count += 1;
    }
    
    fn command_count(&self) -> usize {
        self.count
    }
    
    fn history(&self) -> &[String] {
        &self.history
    }
}

fn demonstrate_advanced_commands(executor: &mut CommandExecutor) {
    println!("Advanced command patterns:");
    
    // Batch execution
    let batch_commands: Vec<Box<dyn Command>> = vec![
        Box::new(PrintCommand::new("Batch 1".to_string())),
        Box::new(AddNumberCommand::new(10, 20)),
        Box::new(DelayCommand::new(50, "Delayed message".to_string())),
    ];
    
    for cmd in batch_commands {
        executor.execute(cmd);
    }
    
    println!("Command history: {:?}", executor.history());
}

// Exercise 5.5: Memory pool for batch processing - SOLVED
fn exercise_5_5() {
    println!("Exercise 5.5: Memory pool for batch processing");
    
    let mut pool = StringPool::new();
    
    for batch in 0..3 {
        println!("Processing batch {}", batch);
        
        let mut strings = Vec::new();
        
        for i in 0..5 {
            let mut s = pool.get_string();
            s.push_str(&format!("Batch {} Item {}", batch, i));
            strings.push(s);
        }
        
        for s in &strings {
            println!("  {}", s);
        }
        
        for s in strings {
            pool.return_string(s);
        }
    }
    
    println!("Pool efficiency: {:.1}% reuse", pool.reuse_rate() * 100.0);
    
    // Demonstrate pool statistics
    pool.print_statistics();
    
    println!("✅ Exercise 5.5 complete\n");
}

// SOLUTION: String pool with detailed statistics
struct StringPool {
    available: Vec<String>,
    total_requests: usize,
    reuses: usize,
    peak_pool_size: usize,
    total_returned: usize,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            available: Vec::new(),
            total_requests: 0,
            reuses: 0,
            peak_pool_size: 0,
            total_returned: 0,
        }
    }
    
    fn get_string(&mut self) -> String {
        self.total_requests += 1;
        
        if let Some(mut s) = self.available.pop() {
            s.clear();  // Clear content but keep capacity
            self.reuses += 1;
            s
        } else {
            String::new()
        }
    }
    
    fn return_string(&mut self, mut s: String) {
        s.clear();
        self.available.push(s);
        self.total_returned += 1;
        
        if self.available.len() > self.peak_pool_size {
            self.peak_pool_size = self.available.len();
        }
    }
    
    fn reuse_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.reuses as f64 / self.total_requests as f64
        }
    }
    
    fn print_statistics(&self) {
        println!("Pool Statistics:");
        println!("  Total requests: {}", self.total_requests);
        println!("  Reuses: {}", self.reuses);
        println!("  New allocations: {}", self.total_requests - self.reuses);
        println!("  Peak pool size: {}", self.peak_pool_size);
        println!("  Total returned: {}", self.total_returned);
        println!("  Current pool size: {}", self.available.len());
    }
    
    fn shrink_to_fit(&mut self) {
        self.available.shrink_to_fit();
    }
}

// Exercise 5.6: Copy-on-write (COW) implementation - SOLVED
fn exercise_5_6() {
    println!("Exercise 5.6: Copy-on-write (COW) implementation");
    
    let original = MyString::from("Hello, World!");
    println!("Original: {}", original);
    
    let copy1 = original.clone();
    let copy2 = original.clone();
    
    println!("Copy1: {}", copy1);
    println!("Copy2: {}", copy2);
    println!("Reference count: {}", original.reference_count());
    
    let mut copy3 = copy1.clone();
    copy3.push_str(" Modified");
    
    println!("After modification:");
    println!("Original: {}", original);
    println!("Copy3: {}", copy3);
    println!("Original ref count: {}", original.reference_count());
    println!("Copy3 ref count: {}", copy3.reference_count());
    
    demonstrate_cow_efficiency();
    
    println!("✅ Exercise 5.6 complete\n");
}

// SOLUTION: Copy-on-write string implementation
use std::fmt;

struct MyString {
    data: Rc<String>,
}

impl MyString {
    fn from(s: &str) -> Self {
        MyString {
            data: Rc::new(s.to_string()),
        }
    }
    
    fn push_str(&mut self, s: &str) {
        // Copy-on-write logic
        if Rc::strong_count(&self.data) > 1 {
            // Data is shared, need to make a private copy
            let mut new_string = (*self.data).clone();
            new_string.push_str(s);
            self.data = Rc::new(new_string);
        } else {
            // We're the only owner, can modify in place
            let string = Rc::make_mut(&mut self.data);
            string.push_str(s);
        }
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    fn reference_count(&self) -> usize {
        Rc::strong_count(&self.data)
    }
    
    fn capacity(&self) -> usize {
        self.data.capacity()
    }
    
    fn as_str(&self) -> &str {
        &self.data
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        MyString {
            data: Rc::clone(&self.data),  // Share the data
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn demonstrate_cow_efficiency() {
    println!("COW efficiency demonstration:");
    
    let base = MyString::from("Base string");
    println!("Base: {} (refs: {})", base, base.reference_count());
    
    // Create many clones - all share the same data
    let clones: Vec<MyString> = (0..5).map(|_| base.clone()).collect();
    println!("After 5 clones, ref count: {}", base.reference_count());
    
    // Modify one clone - only that one gets copied
    let mut modified = clones[0].clone();
    println!("Before modification - ref count: {}", modified.reference_count());
    
    modified.push_str(" - modified");
    println!("After modification:");
    println!("  Modified: {} (refs: {})", modified, modified.reference_count());
    println!("  Original: {} (refs: {})", base, base.reference_count());
}

// Exercise 5.7: Observer pattern with weak references - SOLVED
fn exercise_5_7() {
    println!("Exercise 5.7: Observer pattern with weak references");
    
    let mut subject = Subject::new();
    
    let observer1 = Rc::new(RefCell::new(ConsoleObserver::new("Observer1")));
    let observer2 = Rc::new(RefCell::new(ConsoleObserver::new("Observer2")));
    
    subject.attach(Rc::downgrade(&observer1));
    subject.attach(Rc::downgrade(&observer2));
    
    subject.notify("First notification");
    subject.notify("Second notification");
    
    // Drop an observer and notify again
    drop(observer1);
    subject.notify("Third notification (after drop)");
    
    println!("✅ Exercise 5.7 complete\n");
}

// SOLUTION: Observer pattern using weak references
struct Subject {
    observers: Vec<Weak<RefCell<dyn Observer>>>,
}

trait Observer {
    fn update(&mut self, message: &str);
    fn name(&self) -> &str;
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }
    
    fn attach(&mut self, observer: Weak<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    
    fn notify(&mut self, message: &str) {
        // Remove dead weak references and notify alive ones
        self.observers.retain(|weak_obs| {
            if let Some(obs) = weak_obs.upgrade() {
                obs.borrow_mut().update(message);
                true  // Keep this observer
            } else {
                false  // Remove dead reference
            }
        });
        
        println!("Notified {} observers", self.observers.len());
    }
    
    fn observer_count(&self) -> usize {
        self.observers.iter()
            .filter(|weak| weak.upgrade().is_some())
            .count()
    }
}

struct ConsoleObserver {
    name: String,
    message_count: usize,
}

impl ConsoleObserver {
    fn new(name: &str) -> Self {
        ConsoleObserver {
            name: name.to_string(),
            message_count: 0,
        }
    }
}

impl Observer for ConsoleObserver {
    fn update(&mut self, message: &str) {
        self.message_count += 1;
        println!("🔔 {}: Received message #{}: '{}'", 
                self.name, self.message_count, message);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

// Exercise 5.8: Advanced data structure with all patterns - SOLVED
fn exercise_5_8() {
    println!("Exercise 5.8: Advanced data structure with all patterns");
    
    let mut graph = Graph::new();
    
    let node1 = graph.add_node("Node1".to_string());
    let node2 = graph.add_node("Node2".to_string());
    let node3 = graph.add_node("Node3".to_string());
    
    graph.add_edge(node1, node2);
    graph.add_edge(node2, node3);
    graph.add_edge(node3, node1);  // Create a cycle
    
    println!("Graph traversal:");
    graph.traverse_from(node1);
    
    graph.print_statistics();
    
    println!("✅ Exercise 5.8 complete\n");
}

// SOLUTION: Graph data structure using multiple smart pointer patterns
type NodeId = usize;

struct Graph {
    nodes: Vec<Rc<RefCell<GraphNode>>>,
    next_id: NodeId,
}

struct GraphNode {
    id: NodeId,
    data: String,
    edges: RefCell<Vec<Weak<RefCell<GraphNode>>>>,  // Weak to prevent cycles
    visited: RefCell<bool>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            next_id: 0,
        }
    }
    
    fn add_node(&mut self, data: String) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Rc::new(RefCell::new(GraphNode {
            id,
            data,
            edges: RefCell::new(Vec::new()),
            visited: RefCell::new(false),
        }));
        
        self.nodes.push(node);
        id
    }
    
    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        if let (Some(from_node), Some(to_node)) = (self.get_node(from), self.get_node(to)) {
            from_node.borrow().edges.borrow_mut()
                .push(Rc::downgrade(&to_node));
        }
    }
    
    fn get_node(&self, id: NodeId) -> Option<Rc<RefCell<GraphNode>>> {
        self.nodes.iter()
            .find(|node| node.borrow().id == id)
            .cloned()
    }
    
    fn traverse_from(&self, start: NodeId) {
        self.reset_visited();
        
        if let Some(start_node) = self.get_node(start) {
            let mut queue = VecDeque::new();
            queue.push_back(start_node);
            
            while let Some(node) = queue.pop_front() {
                let node_borrow = node.borrow();
                
                if *node_borrow.visited.borrow() {
                    continue;
                }
                
                *node_borrow.visited.borrow_mut() = true;
                println!("  Visiting: {} ({})", node_borrow.data, node_borrow.id);
                
                // Add neighbors to queue
                for edge in node_borrow.edges.borrow().iter() {
                    if let Some(neighbor) = edge.upgrade() {
                        if !*neighbor.borrow().visited.borrow() {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }
    
    fn reset_visited(&self) {
        for node in &self.nodes {
            *node.borrow().visited.borrow_mut() = false;
        }
    }
    
    fn print_statistics(&self) {
        println!("Graph statistics:");
        println!("  Nodes: {}", self.nodes.len());
        
        let total_edges: usize = self.nodes.iter()
            .map(|node| node.borrow().edges.borrow().len())
            .sum();
        println!("  Edges: {}", total_edges);
        
        // Check for dead references
        let dead_refs: usize = self.nodes.iter()
            .map(|node| {
                node.borrow().edges.borrow().iter()
                    .filter(|weak| weak.upgrade().is_none())
                    .count()
            })
            .sum();
        println!("  Dead references: {}", dead_refs);
    }
}

// BONUS: Production-ready patterns demonstration
fn demonstrate_production_patterns() {
    println!("=== Production-Ready Patterns ===\n");
    
    // Pattern 1: Event-driven architecture
    demonstrate_event_system();
    
    // Pattern 2: Resource management with RAII
    demonstrate_resource_management();
    
    // Pattern 3: Plugin system with trait objects
    demonstrate_plugin_system();
}

// Event system pattern
struct EventBus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str) + Send + Sync>>>,
}

impl EventBus {
    fn new() -> Self {
        EventBus {
            subscribers: HashMap::new(),
        }
    }
    
    fn subscribe<F>(&mut self, event_type: String, handler: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.subscribers.entry(event_type)
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }
    
    fn publish(&self, event_type: &str, data: &str) {
        if let Some(handlers) = self.subscribers.get(event_type) {
            for handler in handlers {
                handler(data);
            }
        }
    }
}

fn demonstrate_event_system() {
    println!("Event-driven architecture:");
    
    let mut bus = EventBus::new();
    
    bus.subscribe("user_login".to_string(), |data| {
        println!("📧 Sending welcome email to {}", data);
    });
    
    bus.subscribe("user_login".to_string(), |data| {
        println!("📊 Logging user activity for {}", data);
    });
    
    bus.publish("user_login", "alice@example.com");
    println!();
}

// Resource management pattern
struct FileManager {
    open_files: RefCell<HashMap<String, std::fs::File>>,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            open_files: RefCell::new(HashMap::new()),
        }
    }
}

impl Drop for FileManager {
    fn drop(&mut self) {
        println!("🧹 FileManager: Cleaning up {} open files", 
                self.open_files.borrow().len());
    }
}

fn demonstrate_resource_management() {
    println!("Resource management with RAII:");
    
    {
        let _manager = FileManager::new();
        // Files would be opened here
        println!("📁 FileManager created and managing resources");
    } // FileManager dropped here, resources cleaned up
    
    println!();
}

// Plugin system pattern
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct UppercasePlugin;
struct ReversePlugin;

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str { "uppercase" }
    fn execute(&self, input: &str) -> String { input.to_uppercase() }
}

impl Plugin for ReversePlugin {
    fn name(&self) -> &str { "reverse" }
    fn execute(&self, input: &str) -> String { input.chars().rev().collect() }
}

struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }
    
    fn register(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }
    
    fn execute(&self, plugin_name: &str, input: &str) -> Option<String> {
        self.plugins.get(plugin_name)
            .map(|plugin| plugin.execute(input))
    }
}

fn demonstrate_plugin_system() {
    println!("Plugin system with trait objects:");
    
    let mut manager = PluginManager::new();
    manager.register(Box::new(UppercasePlugin));
    manager.register(Box::new(ReversePlugin));
    
    let input = "hello world";
    println!("Input: {}", input);
    
    if let Some(result) = manager.execute("uppercase", input) {
        println!("Uppercase: {}", result);
    }
    
    if let Some(result) = manager.execute("reverse", input) {
        println!("Reverse: {}", result);
    }
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zero_copy_processing() {
        let text = "hello world rust programming";
        assert_eq!(count_words(text), 4);
        assert_eq!(find_longest_word(text), Some("programming"));
        
        let words_with_r = find_words_starting_with(text, 'r');
        assert_eq!(words_with_r, vec!["rust"]);
    }
    
    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .name("TestApp".to_string())
            .version("2.0.0".to_string())
            .add_feature("auth".to_string())
            .debug_mode(true)
            .build();
        
        assert_eq!(config.name, "TestApp");
        assert_eq!(config.version, "2.0.0");
        assert!(config.debug_mode);
        assert_eq!(config.features, vec!["auth"]);
    }
    
    #[test]
    fn test_thread_safe_cache() {
        let cache = ThreadSafeCache::new();
        
        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.len(), 1);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
    }
    
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        
        // First request creates new string
        let s1 = pool.get_string();
        assert_eq!(pool.reuse_rate(), 0.0);
        
        // Return and get again - should reuse
        pool.return_string(s1);
        let _s2 = pool.get_string();
        assert!(pool.reuse_rate() > 0.0);
    }
    
    #[test]
    fn test_cow_string() {
        let original = MyString::from("test");
        let copy = original.clone();
        
        assert_eq!(original.reference_count(), 2);
        
        let mut modified = copy.clone();
        modified.push_str(" modified");
        
        // Original should still be shared, modified should be separate
        assert_eq!(original.reference_count(), 2);
        assert_eq!(modified.reference_count(), 1);
        assert_eq!(modified.as_str(), "test modified");
    }
    
    #[test]
    fn test_graph_structure() {
        let mut graph = Graph::new();
        
        let n1 = graph.add_node("A".to_string());
        let n2 = graph.add_node("B".to_string());
        
        graph.add_edge(n1, n2);
        
        assert_eq!(graph.nodes.len(), 2);
        
        // Test that we can traverse without infinite loops
        graph.traverse_from(n1);  // Should not hang
    }
    
    #[test]
    fn test_command_executor() {
        let mut executor = CommandExecutor::new();
        
        let cmd = PrintCommand::new("test".to_string());
        executor.execute(Box::new(cmd));
        
        assert_eq!(executor.command_count(), 1);
        assert_eq!(executor.history().len(), 1);
    }
}
```

## 🎓 Complete Code Walkthrough

### 1. Zero-Copy String Processing
```rust
// Efficient: Returns slices into original string
fn find_words_starting_with(text: &str, prefix: char) -> Vec<&str> {
    text.split_whitespace()
        .filter(|word| word.starts_with(prefix))
        .collect()  // Vec<&str> - no string allocation
}

// Alternative that allocates:
fn find_words_starting_with_owned(text: &str, prefix: char) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.starts_with(prefix))
        .map(|s| s.to_string())  // Allocates new strings
        .collect()
}
```

### 2. Builder Pattern with Ownership
```rust
// Ownership transfer pattern
impl ConfigBuilder {
    fn name(mut self, name: String) -> Self {  // Takes self by value
        self.name = Some(name);
        self  // Returns self by value
    }
    
    fn build(self) -> Config {  // Consumes builder
        Config { /* ... */ }
    }
}

// Usage: chaining transfers ownership at each step
let config = ConfigBuilder::new()
    .name("App".to_string())  // Moves builder
    .version("1.0".to_string())  // Moves builder again
    .build();  // Consumes final builder
```

### 3. Thread-Safe Shared State
```rust
// Pattern: Arc for sharing + Mutex for thread-safe mutation
struct ThreadSafeCache {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Clone for ThreadSafeCache {
    fn clone(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),  // Share the Arc, not the data
        }
    }
}

// Usage across threads
let cache = ThreadSafeCache::new();
let cache_clone = cache.clone();

thread::spawn(move || {
    cache_clone.insert("key".to_string(), "value".to_string());
});
```

### 4. Command Pattern with Trait Objects
```rust
// Dynamic dispatch pattern
trait Command {
    fn execute(&self);
}

struct CommandExecutor {
    count: usize,
}

impl CommandExecutor {
    fn execute(&mut self, command: Box<dyn Command>) {
        command.execute();  // Dynamic dispatch
        self.count += 1;
    }
}

// Uniform storage of different command types
let commands: Vec<Box<dyn Command>> = vec![
    Box::new(PrintCommand::new("Hello".to_string())),
    Box::new(AddNumberCommand::new(1, 2)),
];
```

### 5. Memory Pool Pattern
```rust
// Resource reuse pattern
struct StringPool {
    available: Vec<String>,
    total_requests: usize,
    reuses: usize,
}

impl StringPool {
    fn get_string(&mut self) -> String {
        self.total_requests += 1;
        
        if let Some(mut s) = self.available.pop() {
            s.clear();  // Clear content, keep capacity
            self.reuses += 1;
            s
        } else {
            String::new()  // Allocate new if pool empty
        }
    }
    
    fn return_string(&mut self, mut s: String) {
        s.clear();  // Reset for reuse
        self.available.push(s);
    }
}
```

### 6. Copy-on-Write Pattern
```rust
// Lazy copying pattern
struct MyString {
    data: Rc<String>,
}

impl MyString {
    fn push_str(&mut self, s: &str) {
        if Rc::strong_count(&self.data) > 1 {
            // Data is shared, must copy
            let mut new_data = (*self.data).clone();
            new_data.push_str(s);
            self.data = Rc::new(new_data);
        } else {
            // Sole owner, modify in place
            Rc::make_mut(&mut self.data).push_str(s);
        }
    }
}

impl Clone for MyString {
    fn clone(&self) -> Self {
        MyString {
            data: Rc::clone(&self.data),  // Share until mutation
        }
    }
}
```

## 🏆 Production-Ready Patterns

### Event-Driven Architecture
```rust
struct EventBus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str) + Send + Sync>>>,
}

// Usage: Decoupled event handling
bus.subscribe("user_login".to_string(), |user| {
    send_welcome_email(user);
});

bus.publish("user_login", "alice@example.com");
```

### Resource Management with RAII
```rust
struct FileManager {
    open_files: RefCell<HashMap<String, File>>,
}

impl Drop for FileManager {
    fn drop(&mut self) {
        // Automatic cleanup when dropped
        println!("Cleaning up {} files", self.open_files.borrow().len());
    }
}
```

### Plugin System with Trait Objects
```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

// Register different plugin implementations
manager.register(Box::new(UppercasePlugin));
manager.register(Box::new(ReversePlugin));
```

## 🎯 Key Learning Achievements

By completing this exercise, you've mastered:

### ✅ **Advanced Ownership Patterns**
- Zero-copy processing for maximum performance
- Builder patterns with fluent ownership transfer
- Thread-safe shared state management
- Dynamic dispatch with trait objects

### ✅ **Memory Management Excellence**
- Resource pooling for allocation efficiency
- Copy-on-write for lazy optimization
- RAII patterns for automatic cleanup
- Weak references to prevent cycles

### ✅ **Production-Ready Skills**
- Event-driven architectures
- Plugin systems with dynamic loading
- Resource management patterns
- Performance optimization techniques

### ✅ **Architectural Thinking**
- When to use each pattern
- Trade-offs between performance and flexibility
- Memory safety without sacrificing performance
- Designing APIs that guide correct usage

### ✅ **Complete Rust Mastery**
You can now:
- Design complex systems with Rust's ownership model
- Choose appropriate smart pointer combinations
- Build thread-safe concurrent applications
- Optimize for both memory and performance
- Create maintainable, safe, and efficient code

## 🚀 Next Steps

**Congratulations!** You've completed the most challenging module in the Rust bootcamp. You now have **master-level understanding** of Rust's ownership system and can:

1. **Design complex applications** using advanced ownership patterns
2. **Optimize for performance** while maintaining memory safety
3. **Build concurrent systems** with confidence
4. **Choose the right patterns** for each problem domain

You're ready for:
- **Module 03**: Error Handling (Result, Option, custom errors)
- **Advanced Rust topics**: Async programming, unsafe code, procedural macros
- **Real-world projects**: Building production systems with Rust

You've mastered Rust's most unique and powerful feature - the ownership system that makes Rust both fast and safe! 🦀✨