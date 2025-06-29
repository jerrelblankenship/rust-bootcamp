# Exercise 3 Hints - Level 3 (Complete Solutions)

## 🎯 Complete Working Implementation

You've worked through earlier levels but need to see the complete, working implementation. Here's the full solution with detailed explanations.

## 📝 Complete ex03-lifetimes.rs Implementation

```rust
// Exercise 3: Lifetimes - Complete Solutions
//
// This file demonstrates all lifetime concepts from Module 02

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 3: Lifetimes (Complete Solutions) ===\n");
    
    // All exercises working and uncommented
    exercise_3_1();
    exercise_3_2();
    exercise_3_3();
    exercise_3_4();
    exercise_3_5();
    exercise_3_6();
    exercise_3_7();
    exercise_3_8();
    
    println!("\n🎉 All lifetime exercises completed!");
    
    // Demonstrate advanced lifetime patterns
    demonstrate_advanced_lifetimes();
}

// Exercise 3.1: Function returning references - SOLVED
fn exercise_3_1() {
    println!("Exercise 3.1: Function returning references");
    
    let string1 = String::from("hello");
    let string2 = String::from("world!");
    
    let result = longest(&string1, &string2);  // Works now!
    println!("Longest string: {}", result);
    
    // Demonstrate with different string lengths
    let short = String::from("hi");
    let longer = String::from("hello there");
    let result2 = longest(&short, &longer);
    println!("Longest of '{}' and '{}': '{}'", short, longer, result2);
    
    println!("✅ Exercise 3.1 complete\n");
}

// SOLUTION: Add lifetime annotation to relate inputs and output
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Exercise 3.2: Lifetime elision rules - SOLVED
fn exercise_3_2() {
    println!("Exercise 3.2: Lifetime elision rules");
    
    let text = String::from("hello world rust programming");
    let first = first_word(&text);
    println!("First word: {}", first);
    
    // ANSWERS:
    // 1. How many input lifetimes does first_word have? ONE
    // 2. How many output lifetimes does it have? ONE
    // 3. Which elision rule applies? Rule 1: single input lifetime -> output gets same lifetime
    
    // Demonstrate that elision works
    let sentence = "This is a test sentence";
    let word = first_word(sentence);
    println!("First word of '{}': '{}'", sentence, word);
    
    println!("✅ Exercise 3.2 complete\n");
}

// This function works without explicit lifetimes due to elision rules
// Rust infers: fn first_word<'a>(s: &'a str) -> &'a str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Exercise 3.3: Structs with lifetimes - SOLVED
fn exercise_3_3() {
    println!("Exercise 3.3: Structs with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..13],  // Works now with lifetime parameter
    };
    
    println!("Excerpt: {}", excerpt.part);
    
    // Demonstrate that the struct can't outlive the referenced data
    let excerpt2;
    {
        let temp_novel = String::from("In a hole in the ground there lived a hobbit.");
        excerpt2 = ImportantExcerpt {
            part: &temp_novel[0..10],
        };
        println!("Excerpt2 inside scope: {}", excerpt2.part);
    }
    // excerpt2 can't be used here because temp_novel was dropped
    
    println!("✅ Exercise 3.3 complete\n");
}

// SOLUTION: Add lifetime parameter to struct
struct ImportantExcerpt<'a> {
    part: &'a str,  // The reference must live for lifetime 'a
}

// Exercise 3.4: Methods with lifetimes - SOLVED
fn exercise_3_4() {
    println!("Exercise 3.4: Methods with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..13],
    };
    
    // Method works now!
    let announcement = excerpt.announce_and_return_part("Breaking news!");
    println!("Announcement result: {}", announcement);
    
    // Demonstrate additional methods
    println!("Length: {}", excerpt.length());
    println!("Contains 'Call': {}", excerpt.contains("Call"));
    
    println!("✅ Exercise 3.4 complete\n");
}

// SOLUTION: Implement methods for ImportantExcerpt
impl<'a> ImportantExcerpt<'a> {
    // Method that returns reference with same lifetime as self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part  // Return the part field - lifetime inferred from &self
    }
    
    // Additional helper methods
    fn length(&self) -> usize {
        self.part.len()
    }
    
    fn contains(&self, pattern: &str) -> bool {
        self.part.contains(pattern)
    }
    
    // Method that takes another reference with different lifetime
    fn compare_with<'b>(&self, other: &'b str) -> bool {
        self.part.len() > other.len()
    }
}

// Exercise 3.5: Multiple lifetime parameters - SOLVED
fn exercise_3_5() {
    println!("Exercise 3.5: Multiple lifetime parameters");
    
    let name = String::from("Alice");
    let title = String::from("Dr.");
    
    let full_title = format_title(&title, &name);  // Works now!
    println!("Full title: {}", full_title);
    
    // Demonstrate different lifetime requirements
    let result;
    {
        let temp_name = String::from("Bob");
        result = format_title(&title, &temp_name);  // Works because result only depends on title
    }
    println!("Result: {}", result);  // title still alive
    
    println!("✅ Exercise 3.5 complete\n");
}

// SOLUTION: Output lifetime only depends on first parameter
fn format_title<'a>(title: &'a str, name: &str) -> &'a str {
    // This function always returns the title, so the output lifetime
    // only needs to match the title's lifetime, not the name's
    println!("Formatting title for {}", name);
    title
}

// Exercise 3.6: Static lifetime - SOLVED
fn exercise_3_6() {
    println!("Exercise 3.6: Static lifetime");
    
    let greeting = get_greeting();
    println!("Greeting: {}", greeting);
    
    // ANSWER: String literals are compiled into the binary and live for 
    // the entire duration of the program, hence 'static lifetime
    
    let custom = get_custom_message("Hello");  // Works with corrected function
    println!("Custom: {}", custom);
    
    // Demonstrate 'static lifetime
    let static_ref = get_static_reference();
    println!("Static reference: {}", static_ref);
    
    println!("✅ Exercise 3.6 complete\n");
}

fn get_greeting() -> &'static str {
    "Hello, world!"  // String literals have 'static lifetime
}

// SOLUTION: Return owned String instead of trying to return 'static
fn get_custom_message(msg: &str) -> String {
    format!("Custom: {}", msg)  // Return owned data
}

// Alternative solution with proper lifetime
fn get_custom_message_ref<'a>(msg: &'a str) -> &'a str {
    msg  // Return the input reference
}

// Demonstrate true 'static data
fn get_static_reference() -> &'static str {
    const STATIC_DATA: &str = "This lives forever";
    STATIC_DATA
}

// Exercise 3.7: Lifetime bounds in generics - SOLVED
fn exercise_3_7() {
    println!("Exercise 3.7: Lifetime bounds in generics");
    
    let text = String::from("example text");
    let wrapper = Wrapper { value: &text };  // Works now!
    
    println!("Wrapped: {}", wrapper.value);
    
    // Demonstrate with different types
    let number = 42;
    let num_wrapper = NumberWrapper { value: &number };
    println!("Number wrapper: {}", num_wrapper.value);
    
    println!("✅ Exercise 3.7 complete\n");
}

// SOLUTION: Add lifetime parameter for references
struct Wrapper<'a> {
    value: &'a str,  // Reference needs explicit lifetime
}

// Generic version with lifetime bounds
struct GenericWrapper<'a, T> {
    value: &'a T,  // Generic reference with lifetime
}

// For owned data, no lifetime needed
struct NumberWrapper<'a> {
    value: &'a i32,
}

// Exercise 3.8: Complex lifetime relationships - SOLVED
fn exercise_3_8() {
    println!("Exercise 3.8: Complex lifetime relationships");
    
    let mut cache = Cache::new();
    
    let data1 = String::from("cached data 1");
    let data2 = String::from("cached data 2");
    
    cache.store("key1", &data1);  // Works now!
    cache.store("key2", &data2);
    
    if let Some(value) = cache.get("key1") {
        println!("Retrieved: {}", value);
    }
    
    // Demonstrate cache operations
    cache.store("key3", "static string");  // 'static data works too
    
    println!("Cache contents:");
    cache.print_all();
    
    println!("Cache size: {}", cache.size());
    
    println!("✅ Exercise 3.8 complete\n");
}

// SOLUTION: Cache implementation with proper lifetimes
struct Cache<'a> {
    map: HashMap<String, &'a str>,  // Values have lifetime 'a
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }
    
    // Store method - value must live for lifetime 'a
    fn store(&mut self, key: &str, value: &'a str) {
        self.map.insert(key.to_string(), value);
    }
    
    // Get method - return value has same lifetime as stored data
    fn get(&self, key: &str) -> Option<&'a str> {
        self.map.get(key).copied()
    }
    
    // Additional utility methods
    fn remove(&mut self, key: &str) -> Option<&'a str> {
        self.map.remove(key)
    }
    
    fn size(&self) -> usize {
        self.map.len()
    }
    
    fn print_all(&self) {
        for (key, value) in &self.map {
            println!("  {}: {}", key, value);
        }
    }
    
    fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}

// BONUS IMPLEMENTATIONS - Advanced Lifetime Patterns

fn demonstrate_advanced_lifetimes() {
    println!("=== Advanced Lifetime Patterns ===\n");
    
    // Pattern 1: Multiple lifetime parameters
    demonstrate_multiple_lifetimes();
    
    // Pattern 2: Lifetime subtyping  
    demonstrate_lifetime_subtyping();
    
    // Pattern 3: Higher-ranked trait bounds
    demonstrate_higher_ranked_lifetimes();
    
    // Pattern 4: Self-referential structs (showing limitations)
    demonstrate_self_referential_limitations();
}

// Challenge 1: Function with three string parameters
fn choose_string<'a>(a: &'a str, b: &'a str, c: &'a str, choice: u8) -> &'a str {
    match choice {
        0 => a,
        1 => b,
        _ => c,
    }
}

fn demonstrate_multiple_lifetimes() {
    println!("Multiple lifetime parameters:");
    
    let s1 = String::from("first");
    let s2 = String::from("second");
    let s3 = String::from("third");
    
    let chosen = choose_string(&s1, &s2, &s3, 1);
    println!("Chosen: {}", chosen);
    
    // Different lifetimes example
    let result = longest_with_context(&s1, &s2, "context");
    println!("Longest with context: {}", result);
    
    println!();
}

// Different lifetimes for different purposes
fn longest_with_context<'a, 'b>(x: &'a str, y: &'a str, context: &'b str) -> &'a str {
    println!("Context: {}", context);  // 'b lifetime only used here
    if x.len() > y.len() { x } else { y }  // Return has 'a lifetime
}

// Challenge 2: Struct with two different reference types
struct TwoRefs<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> TwoRefs<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        TwoRefs { first, second }
    }
    
    fn first(&self) -> &str {
        self.first
    }
    
    fn second(&self) -> &str {
        self.second
    }
    
    fn compare_lengths(&self) -> &str {
        if self.first.len() > self.second.len() {
            "First is longer"
        } else {
            "Second is longer or equal"
        }
    }
}

fn demonstrate_lifetime_subtyping() {
    println!("Lifetime subtyping:");
    
    let long_lived = String::from("long lived string");
    let refs;
    
    {
        let short_lived = String::from("short lived");
        refs = TwoRefs::new(&long_lived, &short_lived);
        println!("First: {}, Second: {}", refs.first(), refs.second());
    }
    // refs can't be used here because short_lived was dropped
    
    println!();
}

// Higher-ranked trait bounds (advanced)
fn demonstrate_higher_ranked_lifetimes() {
    println!("Higher-ranked trait bounds:");
    
    // Function that works with any lifetime
    let closure = |x: &str| x.len();
    let result = apply_to_string(&String::from("test"), closure);
    println!("Result: {}", result);
    
    println!();
}

fn apply_to_string<F>(s: &String, f: F) -> usize 
where 
    F: for<'a> Fn(&'a str) -> usize,  // Higher-ranked trait bound
{
    f(s)
}

// Challenge 4: Limitations of self-referential structs
fn demonstrate_self_referential_limitations() {
    println!("Self-referential struct limitations:");
    println!("Rust doesn't allow structs to directly reference themselves");
    println!("Solutions: Box, Rc, or external storage");
    
    // This pattern doesn't work in Rust:
    // struct SelfRef {
    //     data: String,
    //     reference: &str,  // Can't reference self.data
    // }
    
    // Solution: Use Box for recursive structures
    let list = create_recursive_list();
    print_list(&list);
    
    println!();
}

// Recursive structure using Box (proper way)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_recursive_list() -> List {
    use List::*;
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} -> ", value);
            print_list(next);
        }
        List::Nil => println!("Nil"),
    }
}

// Challenge: Database with connections (complex lifetime scenario)
struct Database<'a> {
    connections: Vec<Connection<'a>>,
}

struct Connection<'a> {
    url: &'a str,
    active: bool,
}

impl<'a> Database<'a> {
    fn new() -> Self {
        Database {
            connections: Vec::new(),
        }
    }
    
    fn add_connection(&mut self, url: &'a str) {
        self.connections.push(Connection {
            url,
            active: true,
        });
    }
    
    fn get_active_connections(&self) -> Vec<&Connection<'a>> {
        self.connections.iter().filter(|conn| conn.active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_longest_function() {
        let s1 = "short";
        let s2 = "longer string";
        let result = longest(s1, s2);
        assert_eq!(result, "longer string");
        
        let s3 = "medium length";
        let result2 = longest(s2, s3);
        assert_eq!(result2, "longer string");
    }
    
    #[test]
    fn test_important_excerpt() {
        let text = String::from("Hello world");
        let excerpt = ImportantExcerpt { part: &text[0..5] };
        assert_eq!(excerpt.part, "Hello");
        assert_eq!(excerpt.length(), 5);
        assert!(excerpt.contains("Hell"));
    }
    
    #[test]
    fn test_cache_operations() {
        let mut cache = Cache::new();
        let data1 = String::from("value1");
        let data2 = String::from("value2");
        
        cache.store("key1", &data1);
        cache.store("key2", &data2);
        
        assert_eq!(cache.get("key1"), Some("value1"));
        assert_eq!(cache.get("key2"), Some("value2"));
        assert_eq!(cache.get("nonexistent"), None);
        assert_eq!(cache.size(), 2);
    }
    
    #[test]
    fn test_wrapper_struct() {
        let text = String::from("wrapped text");
        let wrapper = Wrapper { value: &text };
        assert_eq!(wrapper.value, "wrapped text");
    }
    
    #[test]
    fn test_two_refs_struct() {
        let s1 = String::from("first");
        let s2 = String::from("second string");
        let refs = TwoRefs::new(&s1, &s2);
        
        assert_eq!(refs.first(), "first");
        assert_eq!(refs.second(), "second string");
    }
    
    #[test]
    fn test_choose_string() {
        let a = "apple";
        let b = "banana";
        let c = "cherry";
        
        assert_eq!(choose_string(a, b, c, 0), "apple");
        assert_eq!(choose_string(a, b, c, 1), "banana");
        assert_eq!(choose_string(a, b, c, 2), "cherry");
    }
    
    #[test]
    fn test_lifetime_elision() {
        let text = "hello world rust";
        let first = first_word(text);
        assert_eq!(first, "hello");
        
        let single = "single";
        let result = first_word(single);
        assert_eq!(result, "single");
    }
}
```

## 🎓 Complete Code Walkthrough

### 1. Lifetime Annotation Syntax
```rust
// Basic lifetime annotation
fn function<'a>(param: &'a str) -> &'a str { param }

// Multiple lifetime parameters
fn function<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }

// Struct with lifetime
struct Container<'a> {
    field: &'a str,
}

// Implementation with lifetime
impl<'a> Container<'a> {
    fn method(&self) -> &str { self.field }
}
```

### 2. Lifetime Elision Rules
```rust
// Rule 1: Single input -> output gets same lifetime
fn single_input(s: &str) -> &str { s }
// Inferred as: fn single_input<'a>(s: &'a str) -> &'a str

// Rule 2: Method with &self -> output gets lifetime of self
impl<'a> Container<'a> {
    fn get(&self) -> &str { self.field }  // Returns &'a str
}

// Rule 3: Multiple inputs need explicit annotation
fn multiple<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 3. Complex Lifetime Relationships
```rust
// Different lifetimes for different purposes
fn process<'a, 'b>(data: &'a str, context: &'b str) -> &'a str {
    println!("Context: {}", context);  // 'b only used for printing
    data  // Return has 'a lifetime
}

// Struct with multiple lifetime parameters
struct MultiRef<'a, 'b> {
    primary: &'a str,    // Primary data
    metadata: &'b str,   // Metadata with different lifetime
}
```

### 4. Static Lifetime Usage
```rust
// String literals are 'static
fn get_constant() -> &'static str {
    "This is a compile-time constant"
}

// Static variables are 'static
static GLOBAL_DATA: &str = "Global static data";

fn get_global() -> &'static str {
    GLOBAL_DATA
}

// Constants are 'static
const CONSTANT_DATA: &str = "Constant data";
```

### 5. Lifetime Bounds in Generics
```rust
// Generic with lifetime bound
struct Container<'a, T> 
where 
    T: 'a,  // T must live at least as long as 'a
{
    data: &'a T,
}

// Higher-ranked trait bounds
fn apply_closure<F>(s: &str, f: F) -> usize 
where
    F: for<'a> Fn(&'a str) -> usize,  // Works with any lifetime
{
    f(s)
}
```

## 🏆 Advanced Lifetime Patterns

### Lifetime Subtyping
```rust
// Longer lifetimes can substitute for shorter ones
fn accepts_short<'short>(_: &'short str) {}

fn provides_long<'long>(s: &'long str) {
    accepts_short(s);  // 'long can substitute for 'short
}
```

### Multiple Return References
```rust
fn split_string<'a>(s: &'a str) -> (&'a str, &'a str) {
    let mid = s.len() / 2;
    (&s[..mid], &s[mid..])
}
```

### Conditional Lifetimes
```rust
fn maybe_return<'a>(condition: bool, s: &'a str) -> Option<&'a str> {
    if condition {
        Some(s)
    } else {
        None
    }
}
```

### Lifetime Bounds in Traits
```rust
trait Display<'a> {
    fn display(&self) -> &'a str;
}

struct Container<'a> {
    data: &'a str,
}

impl<'a> Display<'a> for Container<'a> {
    fn display(&self) -> &'a str {
        self.data
    }
}
```

## 🎯 Key Learning Achievements

By completing this exercise, you've mastered:

### ✅ **Lifetime Fundamentals**
- Understanding why lifetimes exist and what problems they solve
- Reading and writing lifetime annotation syntax
- Applying lifetime elision rules correctly

### ✅ **Complex Lifetime Scenarios**
- Multiple lifetime parameters for different relationships
- Structs and methods with lifetime parameters
- Generic types with lifetime bounds

### ✅ **Memory Safety Through Lifetimes**
- Preventing dangling references at compile time
- Understanding when references are valid
- Designing APIs that express lifetime relationships clearly

### ✅ **Real-World Patterns**
- Building data structures that hold references
- Creating caches and containers with proper lifetime management
- Working with static data and string literals

### ✅ **C# to Rust Translation**
You can now handle C# patterns in Rust:
- Object references → Rust references with explicit lifetimes
- Method return values → Lifetime-aware return types
- Data structures with references → Lifetime-parameterized structs
- Long-lived data → Static lifetimes or owned data

## 🚀 Next Steps

**Congratulations!** You've mastered Rust's lifetime system. You're ready for:

1. **Module 02 Exercise 4**: Smart Pointers (Rc, Arc, Box)
2. **Module 02 Exercise 5**: Advanced Ownership Patterns
3. **Advanced Rust concepts**: Async programming, trait objects, and more

Lifetimes are one of Rust's most challenging concepts - mastering them means you understand the core of Rust's memory safety guarantees! 🦀