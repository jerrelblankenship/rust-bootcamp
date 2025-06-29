# Exercise 3 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Lifetime Issues

You've tried Level 1 hints but need more concrete guidance. Here are specific solutions for each lifetime challenge in Exercise 3.

## 🔧 Exercise 3.1: Function Returning References

**Problem**: `fn longest(x: &str, y: &str) -> &str` fails because Rust doesn't know which input the output relates to.

**Specific Solution**:
```rust
// SOLUTION: Add lifetime parameter to relate inputs and output
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn exercise_3_1() {
    println!("Exercise 3.1: Function returning references");
    
    let string1 = String::from("hello");
    let string2 = String::from("world!");
    
    let result = longest(&string1, &string2);  // Now works!
    println!("Longest string: {}", result);
    
    println!("✅ Exercise 3.1 complete\n");
}
```

**Key Learning**: `'a` means "both inputs and output must live for the same lifetime 'a".

## 🔧 Exercise 3.2: Understanding Lifetime Elision

**Analysis**: `first_word` works without explicit lifetimes due to elision rules.

**Specific Explanation**:
```rust
// This function works without explicit lifetimes
fn first_word(s: &str) -> &str {
    // Elision rule: single input lifetime -> output gets same lifetime
    // Rust infers: fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn exercise_3_2() {
    println!("Exercise 3.2: Lifetime elision rules");
    
    let text = String::from("hello world");
    let first = first_word(&text);
    println!("First word: {}", first);
    
    // ANSWERS:
    // 1. How many input lifetimes? ONE (&str parameter)
    // 2. How many output lifetimes? ONE (&str return)  
    // 3. Which elision rule? Rule 1: single input -> output gets same lifetime
    
    println!("✅ Exercise 3.2 complete\n");
}
```

**Key Learning**: When there's only one input lifetime, Rust automatically assigns it to the output.

## 🔧 Exercise 3.3: Structs with Lifetimes

**Problem**: `struct ImportantExcerpt { part: &str }` fails because structs with references need lifetime parameters.

**Specific Solution**:
```rust
// SOLUTION: Add lifetime parameter to struct
struct ImportantExcerpt<'a> {
    part: &'a str,  // This reference must live for lifetime 'a
}

fn exercise_3_3() {
    println!("Exercise 3.3: Structs with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..13],  // Works now!
    };
    
    println!("Excerpt: {}", excerpt.part);
    
    println!("✅ Exercise 3.3 complete\n");
}
```

**Key Learning**: Structs holding references need lifetime parameters to ensure the referenced data outlives the struct.

## 🔧 Exercise 3.4: Methods with Lifetimes

**Problem**: Need to implement methods on lifetime-parameterized structs.

**Specific Solution**:
```rust
// SOLUTION: Implement methods with lifetime parameters
impl<'a> ImportantExcerpt<'a> {
    // Method that returns a reference with same lifetime as self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part  // Return the part field
    }
}

fn exercise_3_4() {
    println!("Exercise 3.4: Methods with lifetimes");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: &novel[0..13],
    };
    
    // This works now!
    let announcement = excerpt.announce_and_return_part("Breaking news!");
    println!("Announcement result: {}", announcement);
    
    println!("✅ Exercise 3.4 complete\n");
}
```

**Key Learning**: Methods on lifetime-parameterized structs use elision - return values automatically get the lifetime of `&self`.

## 🔧 Exercise 3.5: Multiple Lifetime Parameters

**Problem**: `format_title` always returns the title, so output should have title's lifetime.

**Specific Solution**:
```rust
// SOLUTION: Output lifetime matches first parameter
fn format_title<'a>(title: &'a str, name: &str) -> &'a str {
    // Return value's lifetime only depends on title, not name
    title
}

fn exercise_3_5() {
    println!("Exercise 3.5: Multiple lifetime parameters");
    
    let name = String::from("Alice");
    let title = String::from("Dr.");
    
    let full_title = format_title(&title, &name);  // Works now!
    println!("Full title: {}", full_title);
    
    println!("✅ Exercise 3.5 complete\n");
}
```

**Key Learning**: Different parameters can have different lifetimes. Output lifetime only needs to match the parameters it actually uses.

## 🔧 Exercise 3.6: Static Lifetime

**Problem**: `get_custom_message` can't return `'static` because parameter doesn't live that long.

**Specific Solutions**:

**Solution 1 - Fix function signature**:
```rust
fn get_custom_message(msg: &str) -> String {  // Return owned String
    msg.to_string()  // Convert to owned data
}
```

**Solution 2 - Use proper lifetime**:
```rust
fn get_custom_message<'a>(msg: &'a str) -> &'a str {  // Use input's lifetime
    msg
}
```

**Complete exercise**:
```rust
fn exercise_3_6() {
    println!("Exercise 3.6: Static lifetime");
    
    let greeting = get_greeting();
    println!("Greeting: {}", greeting);
    
    // ANSWER: String literals are compiled into the binary and live forever
    
    let custom = get_custom_message("Hello");  // Works with either solution
    println!("Custom: {}", custom);
    
    println!("✅ Exercise 3.6 complete\n");
}

fn get_greeting() -> &'static str {
    "Hello, world!"  // String literals have 'static lifetime
}
```

**Key Learning**: `'static` means "lives for entire program duration". Only use when data truly lives that long.

## 🔧 Exercise 3.7: Lifetime Bounds in Generics

**Problem**: Generic `Wrapper<T>` needs lifetime bounds when `T` contains references.

**Specific Solution**:
```rust
// SOLUTION: Add lifetime parameter when T contains references
struct Wrapper<'a, T> {
    value: T,
} 

// Or more specifically for our use case:
struct Wrapper<'a> {
    value: &'a str,  // Specific type with lifetime
}

fn exercise_3_7() {
    println!("Exercise 3.7: Lifetime bounds in generics");
    
    let text = String::from("example text");
    let wrapper = Wrapper { value: &text };  // Works now!
    
    println!("Wrapped: {}", wrapper.value);
    
    println!("✅ Exercise 3.7 complete\n");
}
```

**Key Learning**: When generics contain references, they need lifetime parameters.

## 🔧 Exercise 3.8: Complex Lifetime Relationships

**Problem**: Cache storing references needs lifetime parameters throughout.

**Specific Solution**:
```rust
use std::collections::HashMap;

// SOLUTION: Add lifetime parameter to Cache
struct Cache<'a> {
    map: HashMap<String, &'a str>,  // Values are string slices
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }
    
    // Store method takes reference with same lifetime as cache
    fn store(&mut self, key: &str, value: &'a str) {
        self.map.insert(key.to_string(), value);
    }
    
    // Get method returns reference with same lifetime as stored values
    fn get(&self, key: &str) -> Option<&'a str> {
        self.map.get(key).copied()
    }
}

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
    
    println!("✅ Exercise 3.8 complete\n");
}
```

**Key Learning**: Complex data structures with references need careful lifetime management throughout.

## 🔧 Common Lifetime Patterns

### Pattern 1: Same Lifetime for All
```rust
// All parameters and return value share same lifetime
fn process<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    // Return one of the inputs
    if x.len() > y.len() { x } else { y }
}
```

### Pattern 2: Different Lifetimes
```rust
// Different lifetimes when they don't need to be related
fn extract_first<'a, 'b>(data: &'a str, _context: &'b str) -> &'a str {
    // Return value only depends on first parameter
    &data[0..1]
}
```

### Pattern 3: Struct with Lifetime
```rust
struct Container<'a> {
    data: &'a str,
    name: String,  // Owned data doesn't need lifetime
}

impl<'a> Container<'a> {
    fn get_data(&self) -> &str {  // Return inherits lifetime from &self
        self.data
    }
}
```

### Pattern 4: Multiple References in Struct
```rust
struct MultiRef<'a, 'b> {
    first: &'a str,
    second: &'b str,  // Different lifetimes allowed
}
```

## 💡 Key Learning Points

### When to Use Lifetime Annotations
- **Functions**: Multiple reference inputs, reference output
- **Structs**: When storing references as fields
- **Methods**: Usually inferred, but sometimes need explicit bounds
- **Generics**: When type parameters contain references

### Lifetime Elision Rules
1. **Each input gets its own lifetime parameter**
2. **If exactly one input lifetime → output gets same lifetime**
3. **If `&self` or `&mut self` → output gets lifetime of self**

### Common Solutions
```rust
// Problem: Returning local reference
fn bad() -> &str {
    let local = String::from("temp");
    &local  // ERROR: local dropped
}

// Solution 1: Return owned data
fn good1() -> String {
    String::from("temp")
}

// Solution 2: Take input parameter
fn good2(input: &str) -> &str {
    input
}

// Solution 3: Use 'static data
fn good3() -> &'static str {
    "static string"
}
```

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex03-level3.md) for full solutions.

## 🎓 Understanding Check

You should now understand:
1. **Why lifetime annotations are needed**: To prevent dangling references
2. **How to read lifetime syntax**: `'a` connects related lifetimes
3. **When lifetimes are inferred vs explicit**: Based on elision rules
4. **How to fix common lifetime errors**: Add parameters, change return types, or restructure

Ready for smart pointers in Exercise 4! 🦀