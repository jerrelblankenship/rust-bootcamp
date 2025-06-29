# Exercise 5 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Advanced Ownership Patterns

You've tried Level 1 hints but need more concrete guidance. Here are specific solutions for each advanced pattern challenge in Exercise 5.

## 🔧 Exercise 5.1: Zero-Copy String Processing

**Problem**: Need to process text without creating unnecessary String allocations.

**Specific Solutions**:
```rust
// SOLUTION: Use iterator methods that return string slices
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()  // Iterator operation, no allocation
}

fn find_longest_word(text: &str) -> Option<&str> {
    text.split_whitespace()
        .max_by_key(|word| word.len())  // Compare by length, return slice
}

fn find_words_starting_with(text: &str, prefix: char) -> Vec<&str> {
    text.split_whitespace()
        .filter(|word| word.starts_with(prefix))  // Filter returns slices
        .collect()  // Collect slices into Vec
}

fn exercise_5_1() {
    println!("Exercise 5.1: Zero-copy string processing");
    
    let text = "The quick brown fox jumps over the lazy dog";
    
    let word_count = count_words(text);
    let longest = find_longest_word(text);
    let words_starting_with_t = find_words_starting_with(text, 't');
    
    println!("Word count: {}", word_count);
    println!("Longest word: {:?}", longest);
    println!("Words starting with 't': {:?}", words_starting_with_t);
    
    println!("✅ Exercise 5.1 complete\n");
}
```

**Key Learning**: Chain iterator methods to avoid intermediate allocations.

## 🔧 Exercise 5.2: Builder Pattern with Ownership Transfer

**Problem**: Create fluent API that properly handles ownership.

**Specific Solution**:
```rust
// SOLUTION: Define structs and builder with owned data
#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    features: Vec<String>,
}

struct ConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    features: Vec<String>,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            name: None,
            version: None,
            features: Vec::new(),
        }
    }
    
    // SOLUTION: Take self by value to enable chaining + ownership transfer
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self  // Return owned self
    }
    
    fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }
    
    fn add_feature(mut self, feature: String) -> Self {
        self.features.push(feature);
        self
    }
    
    // SOLUTION: Consume builder and create final config
    fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Unnamed".to_string()),
            version: self.version.unwrap_or_else(|| "0.1.0".to_string()),
            features: self.features,
        }
    }
}

fn exercise_5_2() {
    println!("Exercise 5.2: Builder pattern with ownership transfer");
    
    let config = ConfigBuilder::new()
        .name("MyApp".to_string())
        .version("1.0.0".to_string())
        .add_feature("logging".to_string())
        .add_feature("metrics".to_string())
        .build();  // Builder is consumed here
    
    println!("Config: {:#?}", config);
    
    println!("✅ Exercise 5.2 complete\n");
}
```

**Key Learning**: `self` parameters enable chaining while transferring ownership at each step.

## 🔧 Exercise 5.3: Thread-Safe Shared State

**Problem**: Create cache that can be safely shared across threads.

**Specific Solution**:
```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// SOLUTION: Use Arc<Mutex<HashMap>> for thread-safe shared mutable data
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
        let mut map = self.data.lock().unwrap();  // Lock for exclusive access
        map.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<String> {
        let map = self.data.lock().unwrap();
        map.get(key).cloned()  // Clone the value to return owned data
    }
    
    fn clear(&self) {
        let mut map = self.data.lock().unwrap();
        map.clear();
    }
}

// SOLUTION: Clone the Arc, not the HashMap
impl Clone for ThreadSafeCache {
    fn clone(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),  // Share the same HashMap
        }
    }
}

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
    
    println!("✅ Exercise 5.3 complete\n");
}
```

**Key Learning**: `Arc<Mutex<T>>` provides thread-safe shared mutable access.

## 🔧 Exercise 5.4: Command Pattern with Ownership

**Problem**: Store and execute different types of commands.

**Specific Solution**:
```rust
// SOLUTION: Define trait for commands
trait Command {
    fn execute(&self);
}

// SOLUTION: Command implementations with owned data
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
        println!("Printing: {}", self.message);
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
        println!("{} + {} = {}", self.a, self.b, self.a + self.b);
    }
}

// SOLUTION: Command executor using trait objects
struct CommandExecutor {
    count: usize,
}

impl CommandExecutor {
    fn new() -> Self {
        CommandExecutor { count: 0 }
    }
    
    // Takes ownership of boxed command
    fn execute(&mut self, command: Box<dyn Command>) {
        command.execute();
        self.count += 1;
    }
    
    fn command_count(&self) -> usize {
        self.count
    }
}

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
    
    println!("✅ Exercise 5.4 complete\n");
}
```

**Key Learning**: `Box<dyn Trait>` allows storing different types uniformly.

## 🔧 Exercise 5.5: Memory Pool for Batch Processing

**Problem**: Reuse String allocations to improve performance.

**Specific Solution**:
```rust
// SOLUTION: Pool that tracks reuse statistics
struct StringPool {
    available: Vec<String>,
    total_requests: usize,
    reuses: usize,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            available: Vec::new(),
            total_requests: 0,
            reuses: 0,
        }
    }
    
    fn get_string(&mut self) -> String {
        self.total_requests += 1;
        
        if let Some(mut s) = self.available.pop() {
            s.clear();  // Clear contents but keep capacity
            self.reuses += 1;
            s
        } else {
            String::new()  // Create new string
        }
    }
    
    fn return_string(&mut self, mut s: String) {
        s.clear();  // Clear contents
        self.available.push(s);  // Return to pool
    }
    
    fn reuse_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.reuses as f64 / self.total_requests as f64
        }
    }
}

fn exercise_5_5() {
    println!("Exercise 5.5: Memory pool for batch processing");
    
    let mut pool = StringPool::new();
    
    for batch in 0..3 {
        println!("Processing batch {}", batch);
        
        let mut strings = Vec::new();
        
        // Get strings from pool
        for i in 0..5 {
            let mut s = pool.get_string();
            s.push_str(&format!("Batch {} Item {}", batch, i));
            strings.push(s);
        }
        
        // Use the strings
        for s in &strings {
            println!("  {}", s);
        }
        
        // Return strings to pool
        for s in strings {
            pool.return_string(s);
        }
    }
    
    println!("Pool efficiency: {:.1}% reuse", pool.reuse_rate() * 100.0);
    
    println!("✅ Exercise 5.5 complete\n");
}
```

**Key Learning**: Pool pattern reuses allocations by clearing content but preserving capacity.

## 🔧 Exercise 5.6: Copy-on-Write Implementation

**Problem**: Share data until modification is needed, then copy.

**Specific Solution**:
```rust
use std::rc::Rc;
use std::fmt;

// SOLUTION: Use Rc to share data until mutation
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
        // SOLUTION: Copy-on-write logic
        if Rc::strong_count(&self.data) > 1 {
            // Data is shared, need to copy
            let mut new_data = (*self.data).clone();
            new_data.push_str(s);
            self.data = Rc::new(new_data);
        } else {
            // We're the only owner, can modify in place
            let data = Rc::make_mut(&mut self.data);
            data.push_str(s);
        }
    }
    
    fn shared_instances() -> usize {
        // Simplified - in real implementation you'd track this globally
        0  // Placeholder
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

fn exercise_5_6() {
    println!("Exercise 5.6: Copy-on-write (COW) implementation");
    
    let original = MyString::from("Hello, World!");
    println!("Original: {}", original);
    
    let copy1 = original.clone();
    let copy2 = original.clone();
    
    println!("Copy1: {}", copy1);
    println!("Copy2: {}", copy2);
    println!("Shared data: {}", MyString::shared_instances());
    
    let mut copy3 = copy1.clone();
    copy3.push_str(" Modified");  // This triggers copy-on-write
    
    println!("After modification:");
    println!("Original: {}", original);
    println!("Copy3: {}", copy3);
    println!("Shared instances: {}", MyString::shared_instances());
    
    println!("✅ Exercise 5.6 complete\n");
}
```

**Key Learning**: COW shares until mutation, then creates private copy.

## 🔧 Advanced Pattern Combinations

### Pattern: Shared Cache with TTL
```rust
use std::time::{Duration, Instant};

struct CacheEntry<T> {
    value: T,
    inserted_at: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    fn is_expired(&self) -> bool {
        self.inserted_at.elapsed() > self.ttl
    }
}

struct TTLCache<K, V> {
    data: Arc<Mutex<HashMap<K, CacheEntry<V>>>>,
}
```

### Pattern: Builder with Validation
```rust
impl ConfigBuilder {
    fn build(self) -> Result<Config, String> {
        let name = self.name.ok_or("Name is required")?;
        let version = self.version.unwrap_or_else(|| "1.0.0".to_string());
        
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        Ok(Config { name, version, features: self.features })
    }
}
```

### Pattern: Command Queue with Priorities
```rust
use std::cmp::Ordering;

struct PriorityCommand {
    command: Box<dyn Command>,
    priority: u8,
}

impl PartialEq for PriorityCommand {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityCommand {}

impl PartialOrd for PriorityCommand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityCommand {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)  // Reverse for max-heap
    }
}
```

## 💡 Key Learning Points

### Advanced Pattern Categories
- **Zero-copy**: Maximize performance by avoiding allocations
- **Builder**: Ergonomic APIs with ownership-aware chaining
- **Shared state**: Thread-safe access to mutable data
- **Polymorphism**: Trait objects for dynamic dispatch
- **Resource reuse**: Pool patterns for allocation efficiency
- **Lazy copying**: COW for memory-efficient sharing

### Performance Considerations
```rust
// Zero-copy: No allocations
let words: Vec<&str> = text.split_whitespace().collect();

// Builder: Move semantics avoid cloning
let config = Builder::new().name(s).build();  // s is moved

// Shared state: Lock contention
let data = cache.lock().unwrap();  // Potential blocking

// Trait objects: Dynamic dispatch overhead
let cmd: Box<dyn Command> = Box::new(cmd);  // Virtual function calls

// Memory pools: Allocation efficiency
let s = pool.get_string();  // Reuses existing allocation
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex05-level3.md) for full solutions.

## 🎓 Understanding Check

You should now understand:
1. **Zero-copy techniques**: When and how to avoid allocations
2. **Builder ownership**: Why builders consume `self`
3. **Thread-safe sharing**: Arc + Mutex patterns
4. **Dynamic dispatch**: Trait objects vs generics trade-offs
5. **Resource pooling**: When reuse makes sense
6. **Copy-on-write**: Lazy copying for efficiency

Ready to apply these patterns in real Rust applications! 🦀