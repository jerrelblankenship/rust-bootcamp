# Module 02 Debugging Checklist - Ownership and Borrowing

## 🔍 Ownership-Specific Debugging Approach

Ownership errors are unique to Rust and require a different debugging mindset from C#. Follow this systematic approach:

### **Step 1: Understand the Ownership Error Type**

**Common ownership error patterns:**
- [ ] "value borrowed after move" → Ownership transfer issue
- [ ] "cannot borrow ... as mutable" → Borrowing rule violation  
- [ ] "lifetime mismatch" → Reference validity issue
- [ ] "cannot assign to immutable" → Mutability issue

### **Step 2: Trace Data Flow**
Ask these ownership questions:
- [ ] **Who owns this data?** Only one owner at a time
- [ ] **Where does ownership transfer?** Function calls, assignments, returns
- [ ] **What borrows what?** References don't take ownership
- [ ] **How long do references live?** Must not outlive the owned data

### **Step 3: Apply C# Mental Model Translation**

**C# GC Model → Rust Ownership:**
```csharp
// C# - Multiple references, GC cleans up
var data = new List<int> { 1, 2, 3 };
var copy1 = data;  // Both refer to same object
var copy2 = data;  // All references valid until GC
ProcessData(data); // Original still usable
```

```rust
// Rust - Single ownership, explicit transfers
let data = vec![1, 2, 3];
let copy1 = data;        // data MOVED to copy1
// let copy2 = data;     // ERROR: data already moved
// process_data(data);   // ERROR: data no longer available

// Solutions:
let data = vec![1, 2, 3];
let copy1 = data.clone();    // Explicit copy
process_data(&data);         // Borrow instead of move
```

### **Step 4: Common Ownership Debugging Patterns**

**"value borrowed after move"**
```rust
// Problem:
let s = String::from("hello");
let s2 = s;           // s moved to s2
println!("{}", s);    // ERROR: s no longer valid

// Solutions:
// Option 1: Clone
let s = String::from("hello");
let s2 = s.clone();   // Explicit copy
println!("{}", s);    // Works - s still valid

// Option 2: Borrow
let s = String::from("hello");
let s2 = &s;          // Borrow, don't move
println!("{}", s);    // Works - s still owned
```

**"cannot borrow as mutable"**
```rust
// Problem:
let s = String::from("hello");
s.push_str(" world"); // ERROR: s is immutable

// Solution:
let mut s = String::from("hello"); // Add mut
s.push_str(" world"); // Works
```

**Multiple borrows conflict:**
```rust
// Problem:
let mut s = String::from("hello");
let r1 = &s;           // Immutable borrow
let r2 = &mut s;       // ERROR: Can't have mutable while immutable exists
println!("{} {}", r1, r2);

// Solution: Separate scopes
let mut s = String::from("hello");
{
    let r1 = &s;       // Immutable borrow
    println!("{}", r1);
} // r1 scope ends
let r2 = &mut s;       // Now mutable borrow is OK
r2.push_str(" world");
```

### **Step 5: Function Ownership Patterns**

**Taking ownership vs borrowing:**
```rust
// Takes ownership - original becomes unusable
fn take_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

// Borrows - original remains usable
fn borrow_string(s: &String) {
    println!("{}", s);
} // s reference ends, original unaffected

// Mutable borrow - can modify without taking ownership
fn modify_string(s: &mut String) {
    s.push_str(" modified");
}
```

### **Step 6: Smart Pointer Solutions**

When you need shared ownership (like C# references):
```rust
use std::rc::Rc;

// Multiple ownership with reference counting
let data = Rc::new(vec![1, 2, 3]);
let data1 = Rc::clone(&data);  // Increment reference count
let data2 = Rc::clone(&data);  // Both data1 and data2 can use it

// For mutable shared data
use std::cell::RefCell;
let data = Rc::new(RefCell::new(vec![1, 2, 3]));
data.borrow_mut().push(4);  // Runtime borrow checking
```

## 🚨 Debugging Workflow for Ownership Errors

### **1. Read the Error Carefully**
```bash
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:4:5
   |
2  |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`
3  |     let s2 = s;
   |              - value moved here
4  |     println!("{}", s);
   |     ^^^^^^^^^^^^^^^^^ value borrowed here after move
```

**Error analysis checklist:**
- [ ] What value was moved? (`s`)
- [ ] Where was it moved? (line 3: `let s2 = s`)
- [ ] Where are we trying to use it? (line 4: `println!`)

### **2. Choose Your Solution Strategy**

**Strategy 1: Clone (explicit copy)**
- [ ] Use when you need independent copies
- [ ] Performance cost but conceptually simple
- [ ] `let s2 = s.clone();`

**Strategy 2: Borrow (share without moving)**
- [ ] Use when you just need to read the data
- [ ] Zero cost but more complex lifetime rules
- [ ] `let s2 = &s;`

**Strategy 3: Redesign (change function signatures)**
- [ ] Make functions borrow instead of take ownership
- [ ] Change `fn process(s: String)` to `fn process(s: &str)`

**Strategy 4: Smart pointers (shared ownership)**
- [ ] Use `Rc<T>` for shared immutable data
- [ ] Use `Arc<T>` for shared data across threads
- [ ] Use `RefCell<T>` for interior mutability

### **3. Test Your Fix**
- [ ] Does it compile? `cargo check`
- [ ] Does it behave correctly? `cargo test`
- [ ] Do you understand why it works?

## 🎯 Ownership Debugging Questions

**Before making changes, ask:**
1. **"Who should own this data?"** - The part of code responsible for cleanup
2. **"Does this need to be moved or just borrowed?"** - Move if transferring responsibility
3. **"How long does this reference need to live?"** - Must not outlive the owned data
4. **"Can I solve this with borrowing instead of cloning?"** - Usually more efficient

**After fixing, verify:**
1. **"Does the ownership transfer make logical sense?"** - Clear responsibility
2. **"Are the lifetimes correct?"** - References don't outlive data
3. **"Is this the most efficient solution?"** - Avoid unnecessary clones

## 🔧 Debugging Tools

```bash
# Check borrowing without full compilation
cargo check

# Explain specific error codes
rustc --explain E0382

# Run with enhanced error messages
RUST_BACKTRACE=1 cargo run

# Use Rust analyzer for real-time feedback
# (VS Code extension shows ownership issues immediately)
```

## 📚 Learning Mindset for Ownership

**Remember:**
- **Ownership is Rust's superpower** - It prevents crashes C# can have
- **The borrow checker is your friend** - It catches bugs before runtime
- **Every error teaches you** - You're building intuition for safe code
- **Start simple** - Use cloning first, optimize with borrowing later

**Mental model shift from C#:**
- In C#: "Multiple variables can refer to the same object"
- In Rust: "One variable owns the data, others can borrow it"

---

**Ready to master ownership?** Work through the exercises systematically and let the compiler guide your learning!
