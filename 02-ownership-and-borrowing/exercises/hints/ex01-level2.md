# Exercise 1 Hints - Level 2 (Specific Guidance)

## 🎯 Specific Solutions for Ownership Issues

You've tried Level 1 hints but need more concrete guidance. Here are specific solutions for each ownership challenge in Exercise 1.

## 🔧 Exercise 1.1: Fix the Move Error

**Problem**: `println!("s1 = {}, s2 = {}", s1, s2);` fails because `s1` was moved to `s2`.

**Specific Solution**:
```rust
fn exercise_1_1() {
    println!("Exercise 1.1: Fix the move error");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Create independent copy
    
    println!("s1 = {}, s2 = {}", s1, s2);  // Now both work!
    
    println!("✅ Exercise 1.1 complete\n");
}
```

**Why this works**: `.clone()` creates a deep copy, so both `s1` and `s2` own their own data.

**Alternative approach**:
```rust
let s1 = String::from("hello");
let s2 = &s1;  // Borrow instead of move
println!("s1 = {}, s2 = {}", s1, s2);
```

## 🔧 Exercise 1.2: Copy vs Move Semantics

**Analysis of each case**:

```rust
fn exercise_1_2() {
    println!("Exercise 1.2: Understanding Copy vs Move semantics");
    
    // This works - integers implement Copy
    let x = 5;
    let y = x;  // x is copied, not moved
    println!("x = {}, y = {}", x, y);
    
    // Fix: Clone the String
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit clone
    println!("s1 = {}, s2 = {}", s1, s2);
    
    // This works - arrays of Copy types implement Copy
    let a = [1, 2, 3];
    let b = a;  // Array is copied
    println!("a = {:?}, b = {:?}", a, b);
    
    // Fix: Clone the Vec
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone();  // Explicit clone
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
    
    // Answers:
    // 1. Which types implement Copy trait? i32, bool, char, fixed-size arrays of Copy types
    // 2. Which types require explicit cloning? String, Vec, HashMap, custom structs
    // 3. Difference: Copy is automatic and cheap, Move transfers ownership
    
    println!("✅ Exercise 1.2 complete\n");
}
```

## 🔧 Exercise 1.3: Functions and Ownership

**Problem**: Function `takes_ownership` consumes the string, making it unusable afterward.

**Solution Option 1 - Modify the function call**:
```rust
fn exercise_1_3() {
    println!("Exercise 1.3: Functions and ownership");
    
    let s = String::from("hello");
    
    // Use borrowing instead
    borrows_string(&s);
    println!("Can I still use s? {}", s);  // Works now!
    
    println!("✅ Exercise 1.3 complete\n");
}

// Add this function
fn borrows_string(some_string: &String) {
    println!("Borrowed: {}", some_string);
    // Nothing is dropped - we're just borrowing!
}
```

**Solution Option 2 - Return ownership**:
```rust
fn exercise_1_3() {
    let s = String::from("hello");
    
    let s = takes_and_returns(s);  // Get ownership back
    println!("Can I still use s? {}", s);
    
    println!("✅ Exercise 1.3 complete\n");
}

fn takes_and_returns(some_string: String) -> String {
    println!("Got and returning: {}", some_string);
    some_string  // Return ownership
}
```

## 🔧 Exercise 1.4: Returning Ownership Patterns

**Problem**: `append_world` takes ownership of `s1`, so it's no longer usable.

**Solution**: Use borrowing version:
```rust
fn exercise_1_4() {
    println!("Exercise 1.4: Returning ownership patterns");
    
    let s1 = String::from("hello");
    let s2 = append_world_borrow(&s1);  // Borrow instead
    
    println!("Original: {}", s1);  // Now works!
    println!("Result: {}", s2);
    
    println!("✅ Exercise 1.4 complete\n");
}

fn append_world_borrow(s: &str) -> String {
    format!("{} world", s)  // Create new String
}
```

## 🔧 Exercise 1.5: Clone vs Move Usage

**Implementation**:
```rust
fn exercise_1_5() {
    println!("Exercise 1.5: Clone vs Move");
    
    let original = String::from("hello");
    
    // This works multiple times - borrowing
    use_string(&original);
    use_string(&original);
    
    // This works only once - moves ownership
    use_string_once(original.clone());  // Clone for multiple calls
    use_string_once(original.clone());  // Clone again
    
    // Alternative: use the clone function
    use_string_with_clone(&original);
    use_string_with_clone(&original);
    
    println!("✅ Exercise 1.5 complete\n");
}

fn use_string_with_clone(s: &String) {
    let owned = s.clone();  // Clone inside the function
    println!("Using cloned string: {}", owned);
}
```

## 🔧 Exercise 1.6: Ownership in Collections

**Solution strategies**:

```rust
fn exercise_1_6() {
    println!("Exercise 1.6: Ownership in collections");
    
    // Strategy 1: Clone before pushing
    let mut vec = Vec::new();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    
    vec.push(s1.clone());
    vec.push(s2.clone());
    println!("s1: {}", s1);  // Works - we cloned
    println!("s2: {}", s2);  // Works - we cloned
    println!("vec: {:?}", vec);
    
    // Strategy 2: Accept the move
    let mut vec2 = Vec::new();
    let s3 = String::from("moved");
    let s4 = String::from("data");
    
    vec2.push(s3);  // s3 is moved
    vec2.push(s4);  // s4 is moved
    // Don't try to use s3 or s4 after this
    println!("vec2: {:?}", vec2);
    
    println!("✅ Exercise 1.6 complete\n");
}
```

## 🔧 Exercise 1.7: Implement Stack

**Complete implementation**:
```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);  // Takes ownership of item
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()  // Gives ownership to caller
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    // Bonus: Peek without taking ownership
    fn peek(&self) -> Option<&T> {
        self.items.last()  // Return reference, not ownership
    }
}
```

## 🔧 Exercise 1.8: Drop and RAII

**Complete implementation**:
```rust
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("🗑️ Dropping resource: {}", self.name);
    }
}
```

**Prediction**: You'll see drops in reverse order (LIFO - Last In, First Out):
1. `d3` drops first (created last in inner scope)
2. `d2` drops second 
3. `d1` drops last (created first)
4. For move semantics: only `d5` drops (d4 was moved)

## 💡 Key Learning Points

### When to Clone vs Move
- **Clone when**: You need independent copies that can be modified separately
- **Move when**: Transferring responsibility for cleanup
- **Borrow when**: Just reading data without needing ownership

### Function Design Patterns
```rust
// Takes ownership - use when function needs to keep the data
fn consume_data(data: String) -> ProcessedData

// Borrows immutably - use for reading without modification
fn read_data(data: &str) -> Summary

// Borrows mutably - use for modification without taking ownership
fn modify_data(data: &mut String)

// Returns ownership - use when creating new data
fn create_data() -> String
```

### Memory Management Insights
- **Stack data** (Copy types): Automatic copying, very fast
- **Heap data** (Move types): Ownership transfer, prevents double-free
- **RAII**: Resources cleaned up deterministically when owner drops

## ➡️ Next Level

Need complete working implementations? Try [Level 3 Hints](ex01-level3.md) for full solutions.

## 🎓 Understanding Check

You should now understand:
1. **Why cloning works**: Creates independent copies with separate cleanup
2. **When to use borrowing**: Reading data without taking responsibility
3. **How ownership prevents bugs**: No use-after-free or double-free possible
4. **Stack design patterns**: How ownership flows through data structures

Ready for borrowing and references in Exercise 2! 🦀